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

    // 多行会保留换行符
    let long_string = "String literals
    can span multiple lines.
    The linebreak and indentation here ->\
    <- can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test_def_char() {
    // 由于 Unicode 都是 4 个字节编码，因此字符类型都是占用 4 个字节：
    let x = '中';
    let y = 'A';
    println!("'中'占用{}字节",std::mem::size_of_val(&x));
    println!("'A'占用了{}字节",std::mem::size_of_val(&y)); 
}

#[test]
fn test_def_string() {
    // to String
    let _s = String::from("hello");
    let s = "hello".to_string();
    // to &str
    let _s1= &s[..]; 
    let _s1= s.as_str();

    // &s[..]; // &str
    // say_hello(s.as_str());
    //say_hello(&s); // &String -> &str (`deref` 隐式强制转换)
}