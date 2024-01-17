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
    #[test]
    fn test_println() {
        // positional args
        println!("{} days", 31);
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        //## raw print`:?`
        // `{:?}`和`{:#?}` 区别是, 后者是带格式化缩进的
        let a: u8 = 255;
        println!("a:{:?}, b:{:#?}, c={}", a, a, a); 
        println!("raw str:{:?}", "中"); // "中"

        // print hex
        let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
        println!("   0.1 + 0.2= {:x}", (xyz.0 + xyz.1).to_bits());
        println!("   hex(-1i16)= {:x}", -1i16);

        // As can named arguments.
        println!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );

        // Special formatting can be specified after a `:`.
        println!(
            "{} of {:b} people know binary, the other half doesn't",
            1, 2
        );

        // "     1", 5 white spaces and a "1".
        println!("{number:>width$}", number = 1, width = 6);
        // You can pad numbers with extra zeroes. This will output "000001".
        println!("{number:0>width$}", number = 1, width = 6);
        println!("My name is {0}, {1} {0}", "Bond", "Jame");

        // this will output "     1", 5 white spaces and a "1".
        let number: f64 = 1.0;
        let width: usize = 6;
        println!("{number:>width$}");
    }
}
