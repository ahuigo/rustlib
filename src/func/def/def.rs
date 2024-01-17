/*
1. 发散函数(diverge function): 也叫`never 函数`,顾名思义，无法收敛的函数, 永远不会正常结束(panic 或死循环)。
它们的返回类型是 !，这是一个特殊的类型，表示 "never type"。

例如，panic!() 宏和 loop {} 表达式都是发散的，因为它们都不会返回。(方法名带!只是表示它是宏，要展开的)

    #[allow(dead_code)]
    fn never_returns() -> ! {
        panic!("This function never returns!");
    }
*/

// 2. 返回`()` 空元组的函数，称为`unit function`，它们的返回类型是 ()。
#[test]
fn returns_unit() {
    println!("This function returns unit type: ()");
}
