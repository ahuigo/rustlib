#[test]
fn mem_gc(){
    let name = String::from("Sunface");
    // 这行代码在 Rust 中的作用是显式地释放 name 变量的所有权，并调用其析构函数以清理其占用的资源
    std::mem::drop(name); 

}
