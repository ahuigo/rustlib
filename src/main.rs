mod unittest;
mod str;
mod slice;
mod spec;
mod var;

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // change(&s1);
}
