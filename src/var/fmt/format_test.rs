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
    /*
    {:?}: Uses the Debug trait for formatting. This is useful for debugging, as it will print the data in a way that's useful for developers.
    {:#?}:  Formated with pretty-print.
    {}: Uses the Display trait for formatting. This is meant for user-facing output.
    {:o}: Formats integers as octal.
    {:x}: Formats integers as lowercase hexadecimal.
    {:X}: Formats integers as uppercase hexadecimal.
    {:p}: Formats pointers.
    {:b}: Formats integers as binary.
    {:e}: Formats floating point numbers in scientific notation.
    {:E}: Formats floating point numbers in scientific notation with an uppercase 'E'.
    {:0>width}: Formats with leading zeros.
    {:.*}: Specifies the width and precision for a floating point number.
     */
    #[test]
    fn test_format_raw() {
        // 不带格式化缩进的Raw
        println!("{}", format!("{:?}", (100, 200))); // (100, 200)
                                                     // 带格式化缩进的Raw
        println!("{}", format!("{:#?}", (100, 200))); // => "(
                                                      //       100,
                                                      //       200,
                                                      //     )"
    }
    #[test]
    fn test_format_num() {
        let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
        //   0.1 + 0.2= 3fd3333333333334
        println!("   0.1 + 0.2= {:x}", (xyz.0 + xyz.1).to_bits());
        println!("{}", xyz.0 + xyz.1);
        println!("{:.2}", 0.3);
        println!("{:b}", 10u8);// 1010
        // => "0042" with leading zeros
        println!("{}", format!("{:04}", 42));
    }
    #[test]
    fn test_format_args() {
        // 使用对象的display方法(相当于js .toString())
        println!("{}", format!("{} {}", 1, 2)); // => "1 2"
        println!("{}", format!("{value}", value = 4)); // => "4"
        let people = "Rustaceans";
        println!("{}", format!("Hello {people}!")); // => "Hello Rustaceans!"
    }
}
