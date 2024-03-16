#[test]
fn test_let_mut() {
    /**** let 变量遮蔽 */
    let x = 5;
    let x = x + 1; //生成了完全不同的新变量, 涉及一次内存
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x); // 6

    /**** let 声明新类型变量 */
    // 字符串类型
    let spaces = "中国人";
    // usize数值类型
    let spaces = spaces.len();
    println!("spaces len: {}", spaces); // 9

    /*
    注意：let mut 不可声明新变量
    let mut spaces = "abc";
    */
}

#[test]
fn def_multi() {
    let (a, b, c, d);
    (a, b) = (9, 8);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e, .. } = Struct { e: 5 };
    println!("{}{}{}{}", a, b, c, d)
}

#[test]
fn def_datatype() {
    //显式类型/隐式类型
    let a: i32 = 30i32;
    let mut b = 30_i32;
    // ### 下划线开头忽略未使用的变量
    let _x = 5;
    let c = "42".parse::<i32>();
    b = b + 2_000;
    println!("{},{},{:?}", a, b, c);
}
