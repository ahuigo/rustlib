#[test]
fn get_char() {
    // get char from  &str
    let s = "hello";
    let _ = s.chars().nth(0).unwrap();
    // get char from string
    let s = String::from("中hello");
    let c = s.chars().nth(0).unwrap();
    println!("{}", c)
}

// String 类型是一个可变复杂类型，由存储在`栈`中的: 8bytes堆指针、8bytes字符串长度、8bytes字符串容量共同组成(必须是 mut)
// &str 是不可变引用(immutable reference)
#[test]
fn set_char() {
    // set char from  &str(immutability)
    let s = "hello";
    // Convert to String(String 有可变的能力，但是必须是 mut)
    let mut s = s.to_string();
    // s[0]= 'H'; // error
    s.replace_range(0..1, "H"); 
    // convert string to &str
    let s = s.as_str();
    println!("{}", s);
}
