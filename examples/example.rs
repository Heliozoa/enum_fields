#![allow(dead_code)]

#[enum_fields::add(b: bool, o: Option<bool>)]
enum Enum {
    FirstVariant { i: isize, u: usize },
    SecondVariant { f: f64, s: String },
}

fn main() {
    let mut e = Enum::FirstVariant {
        i: 1,
        u: 2,
        b: true,
        o: None,
    };

    println!("{}", e.b());
    // prints: true

    let b = e.b_mut();
    *b = false;
    println!("{}", e.b());
    // prints: false
}
