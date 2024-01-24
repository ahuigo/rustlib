#[test]
fn match_eq_range() {
    macro_rules! matches2 {
    ($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    };
}

    let foo = 'f';
    // match{ 'A'..='Z' | 'a'..='z' => true, _ => false}
    assert!(matches2!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    // match{ Some(x) if x > 2 => true, _ => false}
    assert!(matches2!(bar, Some(x) if x > 2));
}
