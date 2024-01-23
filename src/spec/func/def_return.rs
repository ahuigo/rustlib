/*
1. 发散函数(diverge function): 也叫`never 函数`,顾名思义，无法收敛的函数/无返回值的函数/永远不会正常结束(panic 或死循环)。
它们的返回类型是 !，这是一个特殊的类型，表示 "never type"。

例如，panic!() 宏和 loop {} 表达式都是发散的，因为它们都不会返回。(方法名带!只是表示它是宏，要展开的)

    #[allow(dead_code)]
    fn never_returns() -> ! {
        panic!("This function never returns!");
    }
*/

// 2. 返回`()` 空元组的函数，称为`unit function`，它们的返回类型是 ()。
#[test]
fn return_empty_unit() -> () {
    println!("This function returns unit type: ()");
}

// 2.1func 表达式如果不返回任何值，会隐式地返回一个 `()`空单元类型。要用 `;` 结束表达式, 否则就是返回表达式的值了。
#[test]
fn return_default_unit() {
    fn ret_unit_type() {
        let x = 1;
        // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
        // 类似三元运算符，在Rust里我们可以这样写
        let _y = if x % 2 == 1 { "odd" } else { "even" };
        // 或者写成一行
        let _z = if x % 2 == 1 { "odd" } else { "even" };
    }
    assert_eq!(ret_unit_type(), ())
}

// 最后一个不带分号的表达式的值，会被隐式地当做返回值
#[test]
fn return_default_value() {
    fn num() -> i32 {
        let y = {
            let x = 3;
            x + 1 // 没有分号，这是一个语句块表达式，返回值是 4
        };
        y - 2
    }
    assert_eq!(num(), 2);
}
