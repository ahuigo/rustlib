#[test]
fn generic_func() {
    fn add2<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    add2(1, 2);
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
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}
#[allow(unused)]
#[test]
fn generic_method_specify() {
    struct Point<T, U> {
        x: T,
        y: U,
    }
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

#[test]
fn generic_const() {
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

#[allow(unused)]
fn generic_compile(){
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