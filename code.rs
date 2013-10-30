use std::util::NonCopyable;
use std::fmt::Default;
use std::fmt;
struct Nc(NonCopyable);
impl Nc { fn new() -> Nc { Nc(NonCopyable) } }
impl Default for Nc {
    fn fmt(_: &Nc, f: &mut fmt::Formatter) {
        write!(f.buf, "Nc")
    }
}
impl<'self> Default for &'self Nc {
    fn fmt(_: & &Nc, f: &mut fmt::Formatter) {
        write!(f.buf, "&Nc")
    }
}


fn main() {
    println!("Hello world");
    struct Pt { x: int, y: int };
    struct NcPt { x: int, y: int, n: Nc };
    let p = @Pt { x: 3, y: 4 };
    let q = ~Pt { x: 5, y: 6 };
    let r = ~NcPt { x: 5, y: 6, n: Nc::new() };
    match p { @Pt { x: a, y: b } => { println!("a: {} b: {}", a, b); } }
    match q { ~Pt { x: c, y: d } => { println!("c: {} d: {}", c, d); } }

    match q { ~Pt { x: ref c, y: d } => { println!("c: {:?} d: {}", c, d); } }
    match r { ~NcPt { x: ref c, y: d, n: ref n } => { println!("c: {:?} d: {} n: {}", c, d, n); } }
    match &r { &~NcPt { x: ref c, y: d, n: ref n } => { println!("c: {:?} d: {} n: {}", c, d, n); } }
}
