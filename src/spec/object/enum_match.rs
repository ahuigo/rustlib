#[test]
fn def_option() {
    // Option<T> 是一个内置prelude库中的泛型枚举(使用时不需要加Option::), 它有两个成员:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let _x: Option<u32> = Some(2);
    let _y: Option<f64> = Some(3.14);
    let _z: Option<&str> = Some("hello");
    let _nothing: Option<u32> = None;
}

#[test]
fn match_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), //expression value
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

#[test]
fn match_destructuring_if() {
    #[allow(unused)]
    enum Message {
        Count(i32),
        Move { x: i32, y: i32 },
    }

    let msg = Message::Move { x: 1, y: 1 };

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    }else if let Message::Count(x) = msg {
        dbg!(x);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}
