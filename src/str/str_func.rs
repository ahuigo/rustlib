/*
# 字符串操作(&str + String) 一般是用`String`类型来操作字符串, `&str` 只有.replace() .split() 非inplace方法:
1. inplace 方法仅适用于`String`
2. 以下方法基本都是inplace，无返回值
*/
#[test]
fn test_push() {
    let mut s = String::from("start:");
    s.push_str("rust"); //追加字符串
    s.push('!'); //追加字符char
    println!("{}", s);
}

#[test]
fn test_insert() {
    let mut s = String::from("12345678");
    s.insert_str(6, " I like");
    s.insert(5, ',');
    println!("{}", s); //12345,6 I like78
}

#[test]
fn test_replace() {
    /*
    ## replace
    replacen 第三个参数则表示替换的个数
        s.replacen("rust", "RUST", 1);

    repalce range该方法仅适用于 String 类型: 第一个参数表示替换的范围

        s.replace_range(7..8, "R");
    */
    let s = "rust rust!";
    let new_str = s.replace("rust", "RUST"); //RUST, RUST!
    dbg!(new_str);
}

// 与字符串删除相关的方法有 4 个，他们分别是 pop()，remove()，truncate()，clear()。这四个方法仅适用于 String only类型(inplace)。
#[test]
fn test_del_pop() {
    // 该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 `Option<char>` 类型，如果字符串为空，则返回 None。 示例代码如下：
    let mut string_pop = String::from("rust pop 中文");
    let p1 = string_pop.pop(); //Some( '文',)
    dbg!(p1);
    dbg!(string_pop);

    //2. remove() 方法是按照`字节`位置来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 直接删除第二个汉字
    string_remove.remove(3);
    dbg!(string_remove);

    // 3. truncate —— 删除字符串中从指定位置开始到结尾的全部字符,也是按照`字节`来处理字符串的
    // s.truncate(3);

    // 4. s.clear() —— 相当于s.truncate(0)
}
#[test]
fn test_concat_plus() {
    /*
    1.使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型。
    其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法, 返回`String`
        fn add(self, s: &str) -> String
    */
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string rust会自动编译器解引用为&str
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
fn test_concat_format() {
    // format! 的用法与 print! 的用法类似，详见格式化输出。
    // let s = format!("{} {}!", s1, s2);
}
#[test]
fn test_loop() {
    // ### loop char
    for c in "中国人".chars() {
        println!("{}", c); //type: char
    }
    // ### loop bytes
    for b in "中".bytes() {
        println!("{}", b); // 类型:u8
    }
}

#[allow(dead_code)]
fn test_slice() {
    /*
    ## slice 字串
    标准库做不到，得用这个
    https://crates.io/crates/utf8_slice
    */
}
