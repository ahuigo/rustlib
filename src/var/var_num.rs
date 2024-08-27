#[test]
fn num_type(){
    let _x = 1i32;
    let y = 0.2f32;
    _= y;
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
    for(_i, _c) in _chars.enumerate(){
        println!("{}:{}", _i, _c);
    }
}
#[test]
fn type_as(){
    let speed = 42u8;
    let cph = 2u8;
    let x = (speed * cph) as f64 * 0.9;
    let y = x as u32; // 84*0.9=75.6->75
    dbg!(x);
    dbg!(y);
    // let _x = speed * 100; //overflow
}