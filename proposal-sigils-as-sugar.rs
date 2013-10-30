use allocator::GC;
use allocator::CC;
use allocator::EC;

use @ for GC;
use © for CC;
use € for EC;

struct Pt { x: int, y: int }

fn get_x(p: @Pt) -> int { x.p }
// expands to:
fn get_x(p: GC<Pt>) -> int { x.p }

fn make_pt(a: int, b: int) -> @Pt {
    return @Pt { x: a, y: b };
}
// expands to:
fn make_pt(a: int, b: int) -> GC<Pt> {
    return new (GC::allocator()) Pt { x: a, y: b };
}

// The above is the easy stuff.  And it could be layered, AFAICT, on
// top of what is currently being proposed for smart-syntax library
// support.
//
// The hard part (AFAICT) is how to deal with destructuring-pattern
// binding.

// My current inclination: a semi-nonlocal expansion into magic
// move/copy/borrow methods depending on the nature of the pattern.


fn with_pt_borrow<T>(p: @Pt, f: fn(int, int) -> T) -> T {
    match p {
        @Pt{ x: ref a, y: ref b } => f(a, b)
    }
}
// expands to:
fn with_pt_borrow<T>(p: GC<Pt>, f: fn(int, int) -> T) -> T {
    match GC::borrow(p) {
        &Pt{ x: ref a, y: ref b } => f(a, b)
    }
}

// In fact its possible the above might suffice, or maybe we'll need
// borrow and mut_borrow variants.  (The question I was trying to
// resolve was whether we would also need move/copy, e.g. below)

fn with_pt_is_it_move_or_copy<T>(p: @Pt, f: fn(int, int) -> T) -> T {
    match p {
        @Pt{ x: a, y: b } => f(a, b)
    }
}
// expands to:
fn with_pt_is_it_move_or_copy<T>(p: GC<Pt>, f: fn(int, int) -> T) -> T {
    match GC::move(p) {
        Pt{ x: a, y: b } => f(a, b)
    }
}
// or maybe expands to:
fn with_pt_is_it_move_or_copy<T>(p: GC<Pt>, f: fn(int, int) -> T) -> T {
    match GC::copy(p) {
        Pt{ x: a, y: b } => f(a, b)
    }
}
// I cannot yet tell what is necessary (nor what is even
// possible/desirable).

// The big problem with the above proposal is that it requires
// rewriting the expression being passed to match.  That may be
// non-starter due to the potential desire to handle nested
// occurrences of the sigils:

struct Move { from: @Pt, to: @Pt }
fn with_move_borrow<T>(m: @Move, f: fn(int, int, int, int) -> T) -> T {
    match m {
        @Move{ from: @Pt{ x: a, y: b }, to: @Pt{ x: c, y: d } } => {
            f(a, b, c, d)
        }
    }
}
// expands to:
struct Move { from: GC<Pt>, to: GC<Pt> }
fn with_move_borrow<T>(m: GC<Move>, f: fn(int, int, int, int) -> T) -> T {
    match ??? {
        &Move{ from: &Pt{ x: a, y: b }, to: &Pt{ x: c, y: d } } => {
            f(a, b, c, d)
        }
    }
}

// on the other hand, I do not know how we currently compile match.
// Ah, it is handled directly in trans, not via a prior desugaring
// (see librustc/middle/trans/_match.rs).  Maybe that would have to
// change first.
