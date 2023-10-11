/*
 * Printing is handled by a series of macros defined in std::fmt some of which include:
    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as format! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint!but a newline is appended.
    */
#[cfg(test)]
mod tests {

    // 这个属性告诉 Rust 编译器下面的函数是一个测试函数。cargo test 时会识别它
    #[test]
    fn test_format() {
        println!("{}",format!("hello2"));
        println!("{}",format!("{} {}", 1, 2));           // => "1 2"
        println!("{}",format!("{:?}", (3, 4)));          // => "(3, 4)"
        println!("{}",format!("{value}", value=4));      // => "4"
        let people = "Rustaceans";
        println!("{}",format!("Hello {people}!"));       // => "Hello Rustaceans!"
        println!("{}",format!("{:04}", 42));             // => "0042" with leading zeros
        println!("{}",format!("{:#?}", (100, 200)));     // => "(
                                          //       100,
                                          //       200,
                                          //     )"
    }
}