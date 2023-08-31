// pub 表明可以被其它模块访问
#[allow(dead_code)] // 不显示: warning: function `add` is never used
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}