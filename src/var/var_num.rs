#[test]
fn test_complex_num() {
    use num::complex::Complex;
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im) //13.2 + 21i
}

#[test]
fn test_overflow() {
        let a: u8 = 255;
        // Option<u8>
        let b = a.checked_add(20);
        // b:None, b:None
        println!("b:{:?}, b:{:#?}", b, b);
}