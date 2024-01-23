

/*
# 字符串操作一般是inplace:只有.replace() .split() 非inplace方法:
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

        s.replace_range(7..8, "RUST");//inplace
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
    // 只删除第二个汉字
    string_remove.remove(3);
    dbg!(string_remove);

    // 3. truncate —— 删除字符串中从指定位置开始到结尾的全部字符,也是按照`字节`来处理字符串的
    // s.truncate(3);

    // 4. s.clear() —— 相当于s.truncate(0)
}
#[test]
fn test_loop() {
    // ### loop char
    let s = String::from("中国人");
    for c in s.chars() {
        println!("{}", c); //type: char
    }
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            assert_eq!(c, '中')
        }
    }

    // ### loop bytes
    for b in "中".bytes() {
        println!("{}", b); // 类型:u8
    }
}

#[test]
fn start_with() {
    let s = "hello rust";
    s.starts_with("hello");
}

#[allow(dead_code)]
fn test_slice() {
    /*
    ## slice 字串
    标准库做不到，得用这个
    https://crates.io/crates/utf8_slice
    */
}
