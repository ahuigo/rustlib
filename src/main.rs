mod unittest;
mod str;
mod spec;
mod var;
mod os;

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // change(&s1);
}
