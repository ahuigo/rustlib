#[test]
fn generic_arg() {
    fn add2<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    add2(1, 2);
}

#[test]
fn generic_func_where() {
    // struct Cacher<T: Fn(u32) -> u32> 
    // 还可以使用 `where` 来约束 T
    struct Cacher<T> where T: Fn(u32) -> u32, {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}
#[test]
fn generic_struct() {
    #[allow(dead_code)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let _a = Point { x: 5, y: "ab" };
}

#[test]
fn generic_enum() {
    #![allow(unused)]
    // 如果成功打开文件，则返回 Ok(std::fs::File)，因此 T 对应的是 std::fs::File 类型；
    // 而当打开文件时出现问题时，返回 Err(std::io::Error)，E 对应的就是 std::io::Error 类型。
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

#[allow(unused)]
#[test]
fn generic_method() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }
    impl<T> Point<T, T> {
        fn mixup<U, V>(self, p: Point<U, V>) -> Point<T, V> {
            let Point { x, .. } = self; // self.x is moved
            Point { x, y: p.y }
        }
    }
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}
#[allow(unused)]
#[test]
fn generic_method_specify() {
    struct Point<T, U> {
        x: T,
        y: U,
    }
    // 不要用impl<i32, i8> Point<i32, i8>，因为impl后面的类型参数是用来声明泛型类型的，而不是用来指定具体类型的。
    impl Point<i32, i8> {
        fn mixup(self, other: Point<i32, i8>) -> Point<i32, i8> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}

#[allow(unused)]
#[test]
fn generic_slice() {
    fn display_slice<T: std::fmt::Debug>(arr: &[T]) {
        println!("{:?}", arr);
    }
}
#[test] // 约束为实现了Mul trait的type
#[allow(unused)]
fn generic_constraint() {
    use std::ops;
    fn multiply<T: ops::Mul<Output = T>>(x: T, y: T) -> T {
        x * y
    }
}

#[test]
#[allow(unused)]
fn generic_const() {
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);

    // const泛型参数只能是：1. 单独的 const 泛型参数; 2. 字面量 (i.e. 整数, 布尔值或字符); 3. const 表达式( 表达式中不能包含任何 泛型参数)
    fn foo<const N: usize>() {}
    fn bar<T, const M: usize>() {
        foo::<M>(); // ok: 第一种
        foo::<2021>(); // ok: 符合第二种
        foo::<{ 20 * 100 + 20 * 10 + 1 }>(); // ok: 符合第三种
                                             // foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M
                                             // foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T

        let _: [u8; M]; // ok: 符合第一种
                        // let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
    }
}

// rust night 版本才支持
/*
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
fn generic_const_expr() {
    fn check_size<T>(val: T)
    where
        Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,//const 表达式
    {
    }

    pub enum Assert<const CHECK: bool> {}
    pub trait IsTrue {}
    impl IsTrue for Assert<true> {}

    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
    check_size([(); 31].map(|_| "hello你好".to_string())); // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
    check_size(['中'; 191]); // A char takes 4 bytes in Rust
}
*/

#[allow(unused)]
/*
rust 代码分发方式：
1. 静态分发(static dispatch): `Box<T>`, 编译器会为每一个泛型参数对应的具体类型生成一份代码，对于运行期性能完全没有任何影响。
2. 动态分发(dynamic dispatch): `Box<dyn Trait>`, 运行时，才能确定需要调用什么方法。trait代码中的关键字 dyn 正是在强调这一“动态”的特点
 */
fn generic_compile() {
    /*
    # 泛型代码的单态化(monomorphization)来保证效率: Rust在编译期中一个泛型对应的多个类型，生成各自的代码，因此损失了编译速度和增大了最终生成文件
    的大小。

    Option 枚举的例子：

        fn main() {
            let integer = Some(5);
            let float = Some(5.0);
        }

    编译时对泛型单态化时, 发现两种 Option<T>： 一种对应 i32 另一种对应 f64。
    它会将泛型定义 Option<T> 展开为 Option_i32 和 Option_f64，

        enum Option_i32 {
            Some(i32),
            None,
        }

        enum Option_f64 {
            Some(f64),
            None,
        }

        fn main() {
            let integer = Option_i32::Some(5);
            let float = Option_f64::Some(5.0);
        }

    */
}
