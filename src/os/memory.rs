#[test]
fn mem_gc(){
    let name = String::from("Sunface");
    std::mem::drop(name);
}
