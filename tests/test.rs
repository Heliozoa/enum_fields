#![allow(unused)]

#[enum_fields::add(b: u8, c: u16)]
enum AddField {
    FirstVariant { i: isize, u: usize },
    SecondVariant { f: f64, s: String },
}

#[test]
fn has_fields() {
    let _ = AddField::FirstVariant {
        b: 0,
        c: 1,
        i: 0,
        u: 0,
    };
    let _ = AddField::SecondVariant {
        b: 2,
        c: 3,
        f: 0.0,
        s: String::default(),
    };
}

#[test]
fn has_accessors() {
    let mut fv = AddField::FirstVariant {
        b: 0,
        c: 1,
        i: 0,
        u: 0,
    };
    assert_eq!(fv.b(), &0);
    assert_eq!(fv.c(), &1);
    *fv.b_mut() = 10;
    *fv.c_mut() = 11;
    assert_eq!(fv.b(), &10);
    assert_eq!(fv.c(), &11);
}

#[enum_fields::add(b: u8)]
enum UnnamedVariants {
    FirstVariant { i: isize, u: usize },
    SecondVariant(f64, String),
    None,
}

#[test]
fn unnamed_has_field() {
    let _ = UnnamedVariants::FirstVariant { b: 0, i: 0, u: 0 };
    let _ = UnnamedVariants::SecondVariant(0.0, String::default());
    let _ = UnnamedVariants::None;
}

#[test]
fn unnamed_has_accessor() {
    let mut fv = UnnamedVariants::FirstVariant { b: 0, i: 0, u: 0 };
    let sv = UnnamedVariants::SecondVariant(0.0, String::default());
    let nv = UnnamedVariants::None;
    assert_eq!(fv.b(), Some(&0));
    assert_eq!(sv.b(), None);
    assert_eq!(nv.b(), None);
    fv.b_mut().map(|b| *b = 10);
    assert_eq!(fv.b(), Some(&10));
}
