/*
通常rust 我们经常的字符串类型是
1. `String`，可变的字符串类型
2. `&str` 不可变的字符串类型引用: 分为不可变常量引用(static)、不可变变量引用
 */
#[test]
fn test_def_raw() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
#[test]
fn test_def_escape() {
    // 通过 \x + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3f means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 多行会保留换行符,  使用\忽略换行符
    let long_string = "String literals
    can span multiple lines.
    The linebreak and indentation here ->\
    <- can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test_def_char_len() {
    // 由于 Unicode 都是 4 个字节编码，因此字符类型都是占用 4 个字节：
    let x = '中';
    let y = 'A';
    let _y_u8 = b'A';
    assert_eq!(std::mem::size_of_val(&x), 4);
    assert_eq!(std::mem::size_of_val(&y), 4);
    // String 是utf8编码，一个中文字符占3个字节
    assert_eq!(std::mem::size_of_val("aa中"), 5);//2+3

}
#[test]
fn def_string_vec(){
    let s = String::from("hello");
    // some bytes, in a vector

    let v = vec![104, 101, 108, 108, 111];
    // Turn a bytes vector into a String
    // We know these bytes are valid, so we'll use `unwrap()`.
    let s1 = String::from_utf8(v).unwrap();
    assert_eq!(s, s1);

    // let s to 
    let mut s2 = s;
    s2.push_str("a")

}

#[test]
fn def_string_mut() {
    // Note: 修改的两个条件——　mut　＋　可变类型
    // String:有所有权, 可增长（mut）。
    // &String:无所有权, 可增长
    // &str:无所有权, 不可增长
    let s1 = "hello".to_string();

    // BadCase：s2 是可变引用, 所以可以修改s2指向变量字符串。但是不能修改s2指向的变量的data
    // let mut s2 = &s; 
    // s.push('!');

    // 修改1：move+mut
    let mut s = s1;
    // s[0] = 'H'; //error: 因为一个字符是多字节的，不能直接用index 访问、修改
    s.push('!');


    // 修改2:(要转成vec<u8>)
    let mut bytes = s.into_bytes(); // move to bytes Vec<u8>
    bytes[0] = b'a';
    s = String::from_utf8(bytes).expect("Invalid UTF-8"); // move bytes to s
    dbg!(s);

}
#[test]
fn def_str_slice() {
    let _s = String::from("hello");
    let s = "hello".to_string();


    // to slice &str
    let _s1 = &s[..];
    let _s1 = s.as_str();
    // let _a = _s1[0];

    //say_hello(&s); // &String -> &str (`deref` 隐式强制转换)
}

#[test]
fn def_concat_plus() {
    /*
    使用 + 或者 += 连接字符串，要求
    1. 右边的参数必须为字符串的切片引用（Slice）类型。
    2. 左值必须为 String 类型（ownership）
    其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法, 返回`String`
        fn add(self, s: &str) -> String
    */
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string 会自动被编译器解引用为&str
    let result = string_append + &string_rust;
    let mut _result = result + "!";

    // 2. 注意，self所有权被转移走了(可变引用), 然后返回新的String，self就会被释放
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1, s2还是有效的
    let s3 = s1 + &s2;
    println!("{}|{}", s2, s3); // world!|hello,world!
}

#[test]
fn def_concat_format() {
    // format! 的用法与 print! 的用法类似，详见格式化输出。
    let _s = format!("{} {}!", "a", "b");
}


#[test]
fn def_cap(){
    let mut s = String::with_capacity(25);
    println!("{}", s.capacity());
    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }
}

#[test]
fn def_ptr(){
    use std::mem;
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping the String's data
    let mut s1 = mem::ManuallyDrop::new(story);

    let ptr = s1.as_mut_ptr();
    let len = s1.len();
    let capacity = s1.capacity();

    // story has nineteen bytes
    assert_eq!(16, len);

    // re-build a String via: ptr, len, and capacity. 
    let s2 = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*s1, s2);
}

#[test]
fn type_box_str() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);

    fn greetings(s: &str) {
        println!("{}", s)
    }
}

#[test]
fn type_box_ref() {
    let s: Box<&str> = "hello, world".into();
    greetings(&s);// 引用类型，不会转移所有权: &s 类型是&Box<&str>
    greetings(*s);// 解引用类型，用来获取s指向的值: s 类型是Box<&str>,*s 指向的值类型是&str
    print!("{}", s );
    fn greetings(s: &str) {
        println!("{}", s)
    }
}