
#[test]
fn def_destructuring() {
    let (a, b, c, d, e);
    (a, b) = (9, 8); // tuple
    [c, .., d, _] = [1, 2, 3, 4, 5]; //array: [i32;5]
    [e, ..] = [100;5];// array
    // Struct { e, .. } = Struct { e: 5 };
    println!("{}{}{}{},{}", a, b, c, d,e)
}