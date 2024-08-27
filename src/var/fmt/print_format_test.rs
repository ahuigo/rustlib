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

    /*
    {:?}: Uses the `Debug` trait for formatting.
        1. Debug: https://course.rs/basic/formatted-output.html#debug-%E7%89%B9%E5%BE%81
        2. 也可使用 dbg! 宏，它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息。除此之外，它最终还会把表达式值的所有权返回！
            1. dbg! 输出到标准错误输出 stderr
            2. 而 println! 输出到标准输出 stdout。
    {:#?}:  Formated with pretty-print.
    {}: Uses the `Display trait` for formatting.
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
    fn format_raw() { // raw print(要求必须实现Debug trait)
        // 1. 不带格式化缩进的Raw
        println!("{}", format!("{:?}", (100, 200))); // (100, 200)
        println!("raw str:{:?}", "中"); // "中"
        // 2. 带格式化缩进的Raw
        println!("{:#?}", (100, 200));  // => "(
                                        //       100,
                                        //       200,
                                        //     )"

    }
    #[test]
    fn format_num() {
        // 2. binary
        println!("{:b}", 10u8); // 1010
        // 3. hex
        println!("   hex(-1i16)= {:x}", -1i16);// ffff
        let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
        println!("   0.1 + 0.2= {:x}", (xyz.0 + xyz.1).to_bits());
            //   0.1 + 0.2= 3fd3333333333334
        // 4. float
        println!("{}", xyz.0 + xyz.1);
        // 5. precision: Keep 2 significant digits
        println!("{:.2}", 0.3);
    }
    #[test]
    fn format_args() {
        // 使用对象的display方法(相当于js .toString())
        println!("{}", format!("{} {}", 1, 2)); // => "1 2"
        println!("{}", format!("{value}", value = 4)); // => "4"
        let people = "Rustaceans";
        println!("{}", format!("Hello {people}!")); // => "Hello Rustaceans!"
    }
    #[test]
    fn println_positional_args() {
        // 1. positional args
        println!("{} days", 31);
        println!("My name is {0}, {1} {0}", "Bond", "Jame");
    }

    #[test]
    fn format_named() {
        let name = "Peter";
        println!("{name}");
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );
    }

    #[test]
    fn format_pad() {
        // 1. padding
        println!("{}", format!("{:04}", 42));
            // => "0042" with leading zeros

        // 2. "     1", 5 white spaces and a "1".
        println!("{number:>width$}", number = 1, width = 6);
        // 3. pad numbers with extra zeroes. This will output "000001".
        println!("{number:0>width$}", number = 1, width = 6);

        // 3. this will also output "     1", 5 white spaces and a "1".
        let number: f64 = 1.0;
        let width: usize = 6;
        println!("{number:>width$}");
    }

    #[test]
    fn dbg_borrow_struct() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };
        dbg!(&rect1);//borrow
        dbg!(rect1);//move
    }
}
