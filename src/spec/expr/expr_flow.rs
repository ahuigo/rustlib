#[test]
fn if_else() {
    let condition = true;
    let _n = if condition {
        5
    } else if 13 % 3 == 1 {
        6
    } else {
        7
    };
}

#[test]
fn for_seq() {
    // for i in 0..collection.len()
    for i in 1..5 {
        println!("{}", i);
    }
}
#[test]
fn for_seq_with_tail() {
    for i in 1..=5 {
        println!("{}", i);
    }
}
#[test]
fn for_copy() {
    let arr = [[1, 2]];
    for _item in arr { //ref borrow
         // ...
    }
    dbg!(arr);
}
#[test]
fn for_move() {
    let arr = ["a".to_string()];
    for _item in arr { //move
    }
    // dbg!(arr);//arr is moved
}

#[test]
fn for_borrow_immut() {
    let arr = ["a".to_string()];
    for _item in &arr { //ref borrow
         // ...
    }
    dbg!(arr);
}

#[test]
fn for_borrow_mut() {
    let mut arr = ["a".to_string()];
    for _item in &mut arr {
        //ref mut borrow
        // dbg!(arr);//error
        _item.push_str("b");
    }
    dbg!(arr); //error
}

#[test]
fn for_iter_borrow() {
    let arr = ["a".to_string()];
    // 1. iter(ref borrow)
    for element in arr.iter() {
        // dbg!(arr);//err: borrowed
        println!("the value is: {}", element);
    }
    // 2. iter with enumerate index
    for (i, _item) in arr.iter().enumerate() {
        dbg!(i);
    }
    dbg!(arr);
}

#[test]
fn loop_break() {
    let mut n = 0;
    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n += 1;
    }

    println!("我出来了！");
}
#[test]
fn loop_break_outer() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also ok
            }
            count += 2;
        }

        count += 5;

        #[allow(unused_labels)]
        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30)
}
#[test]
fn loop_break_expr() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
#[test]
fn loop_while() {
    let mut n = 0;
    while n < 5 {
        println!("{}", n);
        n += 1;
    }
}
