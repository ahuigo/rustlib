#[test]
fn def_destructuring() {
    let (a, b, c, d);
    (a, b) = (9, 8);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e, .. } = Struct { e: 5 };
    println!("{}{}{}{}", a, b, c, d)
}