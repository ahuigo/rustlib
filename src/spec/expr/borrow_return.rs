#[test] // ref string 2 times
fn ref_string_double() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);
    s.clear();
    fn first_word(s: &String) -> &str {
        &s[..1] //&&String 就是&str: ref string 2 times
    }
}


#[test]
// 函数返回值:做不可变的引用
fn borrow_return() {
    let mut s = String::from("hello world");
    s.push_str(", world");

    let word = first_word(&s); // s已经被借用给word, 为不可变引用
                               // s.clear(); // error: s不能变成可变引用, word 还没有释放
    println!("the first word is: {}", word);
    fn first_word(s: &String) -> &str {
        // immutable borrow
        &s[..1]
    }
}

// 函数返回值nil:失去作用域的ownership后，引用指向的数据会被释放掉，所以返回引用是不安全的
#[test]
fn borrow_return_nil() {
    /* // 这段代码就是一个悬垂引用(dangling reference), 指向空间已经被释放了
    fn dangle() -> &String {
        // dangle 返回一个字符串的引用
        let s = String::from("hello"); // s 是一个新字符串
        &s // 返回字符串 s 的引用
    } // 这里 s 离开作用域并被丢弃。其内存被释放。
    let _x = dangle(); // 问题：返回的引用指向的数据已经被释放了
    */
}


#[test]
fn return_borrow_static() {
    let c= word();
    println!("{}", c);
    fn word() -> &'static str {
        let b = "aa";//The reference of const string　隐式具有'static生命周期
        b
        // return "word"
    }
}

#[test] 
fn return_borrow_lifetime() {
    // 参数扩展了lifetime:fn xx<'life>(x: &'life str) -> &'life str {}
    // `&self 或者 &mut self self` 的生存期分配给所有输出参数
    let s = String::from("hello world");
    let c= word(&s);
    println!("{}", c);
    fn word(_x:&str) -> &str {
        return "word"
    }
}
#[test]
fn return_borrow_lifetime_extend() {
    let s = String::from("hello world");
    let c= word(&s);
    println!("{}", c);
    fn word<'a>(_x:&'a str) -> &'a str {
        return "word"
    }
}

/*
We can't return inner variable's reference, because it will be dropped after function return.

    fn word<'life>() -> &'life str { 
        let s = String::from("hello world");
        let b = &s[..];
        b //returns a value referencing data owned by the current function
    }

Instead of trying to return a reference, return an owned object. 
1. String instead of &str, 
2. Vec<T> instead of &[T], 
3. T instead of &T, etc.
 */
#[test] 
fn return_inner_var1() {
    fn word() -> String{ 
        let s = String::from("hello");
        s // move operation for s
    }
    let word = word();// move s
    println!("the first word is: {}", word);
}