
// 这是一个条件编译标志. cargo build 或者 cargo run, 它包围的代码不会被编译进最终的二进制文件中
#[cfg(test)]
// 这里`mod tests`定义一个新的mod 名字叫tests, 完整命名空间是 crate::unittest::foo_test::tests; 
// 这个不是必须的, 只是为了防止命名冲突
mod tests {
    // use std::io::{self, Write, Read};
    // use crate::unittest::foo::add;
    use crate::unittest::foo;


    // 这个属性告诉 Rust 编译器下面的函数是一个测试函数。cargo test 时会识别它
    #[test]
    fn myadd() {
        assert_eq!(foo::add(2, 3), 5);
    }
}