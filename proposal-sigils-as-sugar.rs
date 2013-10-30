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

// The above is the easy stuff.
//
// The hard part (AFAICT) is how to deal with destructuring-pattern
// binding.

// My current inclination: a semi-nonlocal expansion into magic
// move/copy/borrow methods depending on the nature of the pattern.

fn with_pt_move<T>(p: @Pt, f: fn(int, int) -> T) -> T {
    match p {
        @Pt{ x: a, y: b } => f(a, b)
    }
}
// expands to:
fn with_pt_move<T>(p: GC<Pt>, f: fn(int, int) -> T) -> T {
    match GC::move(p) {
        Pt{ x: a, y: b } => f(a, b)
    }
}

fn with_pt_borrow<T>(p: @Pt, f: fn(int, int) -> T) -> T {
    match p {
        @Pt{ x: ref a, y: ref b } => f(a, b)
    }
}
// expands to:
fn with_pt<T>(p: GC<Pt>, f: fn(int, int) -> T) -> T {
    match GC::borrow(p) {
        &Pt{ x: ref a, y: ref b } => f(a, b)
    }
}
