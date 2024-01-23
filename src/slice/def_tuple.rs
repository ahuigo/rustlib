// 模式匹配 解构赋值　元组
#[allow(dead_code)] 
fn def_tuple() {
    let tup = (500, 6.4, 1);
    // 元组的成员还可以是一个元组
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // 元组可以是多组不同类型的元素
    let _t2: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    // 用.访问元组(array/slice 不行)
    let _a = tup.0;

    // 用模式匹配解构元组\slice
    let (_x, _y, _)= tup;
}

// return tuple
#[test]
fn return_tuple() {
    /*
        1. main: 函数`main()` `println!()` 返回就是这个空单元类型 `()`
        2. 可以用 `()` 作为 map 的值:
            表示我们不关注具体的值，只关注 key。 类似Go `struct{}` 类似，完全不占用任何内存。
    */
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() 返回字符串的长度
        (s, length)
    }
    // let (s2, len) = calculate_length(String::from("hello"));
    let _tup = calculate_length(String::from("hello"));
}


#[test]
fn x(){
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}