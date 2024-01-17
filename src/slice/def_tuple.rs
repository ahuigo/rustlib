// define tuple
#[allow(dead_code)] 
fn def_tuple() {
    let tup = (500, 6.4, 1);

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
