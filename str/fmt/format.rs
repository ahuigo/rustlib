/*
 * Printing is handled by a series of macros defined in std::fmt some of which include:

    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as format! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint!but a newline is appended.
    */

fn main(){
    println!("{}",format!("hello2"));
    format!("Hello");                 // => "Hello"
    format!("Hello, {}!", "world");   // => "Hello, world!"
    format!("The number is {}", 1);   // => "The number is 1"
    format!("{:?}", (3, 4));          // => "(3, 4)"
    format!("{value}", value=4);      // => "4"
    let people = "Rustaceans";
    format!("Hello {people}!");       // => "Hello Rustaceans!"
    format!("{} {}", 1, 2);           // => "1 2"
    format!("{:04}", 42);             // => "0042" with leading zeros
    format!("{:#?}", (100, 200));     // => "(
                                      //       100,
                                      //       200,
                                      //     )"
}
