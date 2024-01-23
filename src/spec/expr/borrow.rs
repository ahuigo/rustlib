/*
赋值时有三种行为：
1. Move 移动: 浅copy(拷贝指针、长度和容量而不拷贝数据)+所有权转移:  String/Struct本身赋值时就是move(浅copy)
1. Copy: 基础类型、数组(固定长度:也是基础类型)、以及不可变常量字符串, 无所有权，也不转移所有权
1. 引用&T： &str &String &array,引用会被借用，但是不转移所有权，也不会copy　数据
    1. &str/&String/&array 将引用赋值给别人时：
        1. 如果是mut, 没有实现Copy trait, 就是moved
        1. 如果是immut, 实现Copy trait, 就是copied(&str 只有copied)
    2. 原owner 还可用吗？
        1. mut: 不可用，因为所有权转移了(临时)
        1. immut: 可用，&T has Copy trait
 */
#[test]
fn ownership_move() {
    // Copy类型：基础类型＋元组（仅当元组元素都有Copy特征，比如`(u32,f64)`, 而`(i32,String)`就不是）都实现了 Copy trait
    // Move类型: String, Vec, Box, Rc, Arc, Cell, RefCell, Mutex, RwLock, Fn, FnMut, FnOnce, 闭包, 迭代器, 通道, 文件句柄, 套接字, 其他自定义类型
    let s1 = String::from("hello");
    let s2 = s1; // s1 被move到 s2 中，s1 将无效
    let s3 = s2.clone(); // s2.clone 不会有所有权转移
    println!("s2 = {}, s3 = {}", s2, s3);
    fn take_ownership(some_string: String) {
        println!("{}", some_string);
    }
    take_ownership(s2); // move s2, s2 将无效
}

#[test]
// 目前所有基本类型，如整数、浮点数和字符都是Copy类型。默认情况下，struct/enum 不是Copy，但你可以派生 Copy trait:
fn ownership_copy() {
    // Note:这里的Point 是浅copy
    #[derive(Copy, Clone, Debug)]
    struct Point {
        _x: i32,
        _y: i32,
    }
    // #[derive(Copy, Clone)]
    // enum SignedOrUnsignedInt {
    //     Signed(i32),
    //     Unsigned(u32),
    // }
    let p1 = Point { _x: 0, _y: 0 };
    let p2 = p1; // Copy
    println!("{:?},{:?}", p1, p2)
}

// 如何避免传参、返回时发生Move呢？就是borrowing：**获取变量的引用，称之为借用(borrowing)**
#[test]
fn borrow() {
    let x = String::from("hello");
    let y = &x; //y 是 x 的一个引用：允许你使用值，但不获取其所有权。（类似智能指针，属于安全指针）

    println!("{}", x); // x 仍然有效，因为允许存在多个不可变引用
    println!("{}", *y); // 使用 *y 来解出引用所指向的值
    println!("{}", y); // 这样也可以, 因为println!宏会自动解引用
}

#[test]
// mut borrow所有权转移（临时Move）
fn owner_borrowed_mut_move(){//Not Copy trait
    #[allow(unused_mut)]
    let mut s = String::from("hello");
    let b1 = &s;
    // s.push_str(", world");//err: owner is mutable borrowed
    println!("{}",b1);
    s.push_str(", world");//ok, borrower give back owner


}

#[test]
// immut borrow所有权不转移（&T copyied）
fn owner_borrowed_immut_copy(){
    //Note: copy的是引用，而不是数据(&T 实现了Copy)
    let s = String::from("hello");
    let b1 = &s;
    println!("{},{}",s,b1);
}

#[test]
fn borrow_mut() {
    let mut x = String::from("hello");
    let y = &mut x; //y 被借用为可变引用，x 不能再使用了
    y.push('a');
    // println!("{}", x); // error
    println!("{}", *y); // 使用 *y 来解出引用所指向的值
    println!("{}", y); // 这样也可以, 因为println!宏会自动解引用
}

// 可变的引用(作用域期间内只能有一个)
#[test]
fn borrow_mut_multi() {
    let mut s = String::from("hello"); 
    {
        let _r2 = &mut s; //ok, 作用域内只有一个可变引用
    }
    let r1 = &mut s; // s被借出
    // let r2 = &mut s; //error[E0499]: cannot borrow `s` as mutable more than once at a time
    {
        // let _r2 = &mut s; //error[E0499], 作用域内跟r1 冲突
    }
    // println!("{}", s); // error[E0502]:s 不可用，因为它被借出了 
    println!("{}", r1);
}

#[test]
fn borrow_int() {
    let mut x = 5;
    let y = &mut x; //y 被借用为可变引用，x 不能再使用了
    // assert_eq!(5, x); //error
    assert_eq!(5, *y);
}


// `let s=&T` 符号即是不可变引用，它们允许你使用值，但是不获取所有权, 不能修改它
#[test]
fn borrow_func_immut() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    fn calculate_length(s: &String) -> usize {
        // s.push_str(", world"); // error: 引用不可变
        s.len()
    }
}
// 可变的引用: `&mut s` 和接受可变引用参数 `some_string: &mut String` 的函数。
#[test]
fn borrow_func_mut() {
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}


// 可变的引用scope
#[test]
fn borrow_scope() {
    let mut s = String::from("hello");
    let r1 = &s; // immutable
    let r2 = &s; // immutable
    println!("{} and {}", r1, r2); // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s; // 没问题, 因为r1,r2早结束了
    println!("{}", r3);
}


#[test]
fn borrow_move_mut_string() {
    // String 没有实现 Copy trait, 所以move
    let mut x = String::from("hello");
    let y = &mut x; //y 被借用为可变引用(mut borrow)，x 不能再使用了
    // println!("{}", x); //error
    let y1 = y; // y 是可变的引用，被move到 y1 中，y 将无效
    // y.clear();//error
    println!("{}", y1); 
}


#[test]
fn borrow_copy_immut_string() {
    // &T 实现了Copy
    let x = String::from("hello");
    let y = &x; //y 被借用为不可变引用，x 还可再使用了
    let y1 = y; //Copy, 因为&str 字面值就是一个不可变的引用&T
    println!("{},{}", x,y1); 
}

#[test]
fn borrow_copy_immut_str() {
    // &str 实现了Copy
    let x = "abc"; //borrow ref literal
    let y = x; //Copy(borrow 2 times)
    println!("{},{}", x, y);
}

#[test]
fn borrow_copy_immut_array() {
    let x = [1,2,3];
    let y1 = &x;
    let y2 = y1; //y1 is copied
    println!("{:?},{:?}",y1, y2); //ok
}
#[test]
fn borrow_move_mut_array() {
    let mut x = [1,2,3];
    let y1 = &mut x;
    let y2 = y1; //y1 is moved(No Copy trait)
    println!("{:?}" ,y2); 
}
