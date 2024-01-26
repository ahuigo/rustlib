#[test]
fn impl_struct() {
    #[allow(unused)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        // 关联函数Associated function(因为它没有 self，是静态方法), 返回Self类型Circle
        pub fn new(x: f64, y: f64, radius: f64) -> Self {
            Circle { x, y, radius }
        }

        // Circle的方法，&self表示借用当前的Circle结构体
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }
    let c = Circle::new(2f64, 2.0, 3.0);
    dbg!(c.area());
}

#[test]
fn impl_enum() {
    #![allow(unused)]
    enum Message {
        Write(String),
    }

    impl Message {
        fn call(&self) {
            match self {
                Message::Write(x) => println!("{}", x), // borrow
                _ => println!("other"),
            };
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
