#[test]
fn num_type(){
    let _x = 1i32;
    let _y = 0.2f32;
}
#[test]
fn num_complex() {
    use num::complex::Complex;
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im) //13.2 + 21i
}

#[test]
fn num_overflow() {
        let a: u8 = 255;
        // Option<u8>
        let b = a.checked_add(20);
        // b:None, b:None
        println!("b:{:?}, b:{:#?}", b, b);
}

#[test]
fn range_num(){
    let _n1 = 0..10;
    let _n1 = 0..=9;
    let _chars = 'A'..='Z';
}