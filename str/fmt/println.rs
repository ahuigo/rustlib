fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // "     1", 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);
    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "Jame");

    // this will output "     1", 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");
}
