//`format!`,`println!`是rust内部定义的宏(带`!`)，我们可以自定义一个求平方的宏：
// #![allow(unused_variables)], (必须放第一行) 由于多了一个!, 代表整个文件都不检查未使用的变量; 否则就是只对下面的块代码不检查

#[test]
fn square() {
    macro_rules! square {
        ($x:expr) => {
            $x * $x
        };
    }
    let num = 5;
    let result = square!(num);
    println!("Square of {} is {}", num, result);
}
