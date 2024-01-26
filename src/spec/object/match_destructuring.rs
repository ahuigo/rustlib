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
fn match_expr_enum() {
    #[allow(unused)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    let coin = Coin::Penny;
    let _n = match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel | Coin::Dime => 5,
        _ => 25,
    };
}

#[test]
fn match_expr_enum_some() {
    // 1. match some
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), //expression value
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // 2. match multiple patterns and ignore the rest
    let _n = match five {
        Some(_) | None => 8,
        #[allow(unreachable_patterns)]
        _ => 10,
    };
}

#[test]
fn match_expr_num() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

#[test]
fn match_expr_struct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

#[test]
fn match_pattern_rest() {
    let dire: Option<i32> = None;
    match dire {
        Some(_) => println!("Some"),
        // 等价于 _ => println!("other case: None")
        rest => println!("other case: {:?}", rest),
    };
}
#[test]
fn match_pattern_mut_move() {
    let mut v = String::from("hello,");
    let r = &mut v;//borrow
    r.push_str(" ");
    /*
    match r { //2. Error: cannot move out of `*r(String)` which is behind a mutable reference(不允许*r(String) move到match内部, *r没有String的所有权)
        // 1. move occurs: because `value` has type `String`, which does not implement the `Copy` trait
        &mut value => println!("This is String: {}",value), 
    };
    */
    match r { //can move out of `r(&String)`(r拥有ref引用本身所有权，可以move到match内部，注意不是String的所有权)
        value => {
            value.push_str(" world!")
        }
    };
    // r.push_str(" ");//Error: borrow of moved value: `r`
    v.push_str(" ");//ok
}
#[test]
fn match_pattern_enum() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y); //Some(5),10
}
#[test]
fn match_pattern_enum_move_and_borrow() {
    // 1. move
    let x = Some("abc".to_string());
    match x {
        Some(y) => println!("{}",y),
        _ => println!("Default case, x = {:?}", x),
    };
    // dbg!(x);//Error: borrow of moved value: `x`

    //2. borrow
    enum Message {
        Write(String),
    }
    let x = Message::Write("abc".to_string());
    match &x {
        Message::Write(y) => println!("{}",y),
    };

}
#[test]
fn match_pattern_matches() {
    // matches! 宏：https://mp.weixin.qq.com/s/u1x3-c4M53W-1b6kT_rgOQ
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }
    // if MyEnum::Foo == MyEnum::Foo {}// error: Missing PartialEq
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // 1. v.iter().filter(|x| x == MyEnum::Foo); // &&MyEnum == Foo 无法编译
    // 2. matches! 是一个match宏, 返回true or false
    let v2iter = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    for v in v2iter {
        println!("{:?}", v);
    }
}

#[test]
fn match_pattern_range() {
    // 1. use matches!
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    // 2. use match
    match 'f' {
        'A'..='Z' | 'a'..='z' => true,
        _ => false,
    };
}
#[test]
fn match_pattern_range_at() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        }
        | Message::Hello {
            id: id_variable @ (8..=9 | 10),
        } => {
            println!("Found an id in range: {}", id_variable) //Found an id in range: 5
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
#[test] //@允许将匹配值绑定到变量
fn match_pattern_bind_at() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    //1. 绑定变量
    match 1 {
        // 绑定 `num` 到匹配的数字上
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }

    // 1.1 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    // 1.2. 绑定+解构+if
    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}

#[test] //match 匹配守卫：https://doc.rust-lang.org/stable/book/ch18-03-pattern-syntax.html#extra-conditionals-with-match-guards
fn match_pattern_guard() {
    // 1. guard with match
    let a = Some("hello world".to_string());
    match a {
        Some(ref x) if x.len() > 10 => println!("{:?}", a), // 只是一个ref引用，没有发生所有权转移，所以不会报错
        _ => (),
    }
    //2. guard with matches
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2)); // match{ Some(x) if x > 2 => true, _ => false}

    //3. guard with multiple patterns
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => panic!("won't run 456"),
        _ => println!("no"),
    }
}

#[test] //pattern match
fn destructuring_struct_tuple_array() {
    //1. 函数参数也支持destructuring

    //2 解构struct(with alias)
    let Point { x: a, y: b } = Point { x: 2, y: 5 };
    assert_eq!(a + b, 7);
    let Point { x, .. } = Point { x: 2, y: 5 };
    assert_eq!(x, 2);

    //3. 解构tutple
    let (_, _y) = (1, 2);
    let (_a, .., _b, _c) = (1, 2, 3, 4);
    struct Point {
        x: i32,
        y: i32,
    }
    // with `..` rest
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    let ((.., _inches), Point { x: _x, .. }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{},{},{},{}", feet, inches, x, y);

    //4. destruct　array slice
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
    }
    if let &[.., y] = arr {
        assert_eq!(y, 514);
    }
    let arr: &[u16] = &[];
    if let &[.., y] = arr {
        panic!("not run{}", y)
    }
    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [_x, ..]));
}

#[test]
fn destructuring_ignore_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("use underscore to ignore value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
}

#[test] //pattern match
fn destructuring_if() {
    #[allow(unused)]
    enum Message {
        Count(i32),
        Move { x: i32, y: i32 },
    }

    let msg = Message::Move { x: 1, y: 1 };
    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else if let Message::Count(x) = msg {
        dbg!(x);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}
#[test]
fn destructuring_while() {
    #[allow(unused)]
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素Option<i32>
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

#[test]
fn destructuring_ref1() {
    let s = Some(String::from("hello world"));
    if let Some(ref x) = s {
        //ref borrow
        println!("{}", x);
    }
    println!("{:?}", s);
}
#[test]
fn destructuring_ref2() {
    let s = Some(String::from("hello world"));
    if let Some(x) = &s {
        //ref borrow
        println!("{}", x);
    }
    println!("{:?}", s);
}
