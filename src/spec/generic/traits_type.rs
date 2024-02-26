#[test] // ?Sized 是一个特殊的 trait bound，表示类型可能不是 Sized 的(即动态大小类型, 如切片编译时大小未知)
fn trait_type_associated() {
    //关联类型(associated type): trait 声明一个待定类型
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>; //如果不需要改变self就不需要&mut
    }
    struct Counter {
        count: u32,
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            Some(self.count)
        }
    }

    if true {
        let mut c = Counter { count: 1 };
        c.next();
    }
}
#[test]
fn trait_type_generic() {
    // 用generic　代替关联type
    pub trait Iterator<Item> {
        fn next(&mut self) -> Option<Item>;
    }
    struct Counter<Item> {
        count: Item,
    }
    impl<Item: Copy> Iterator<Item> for Counter<Item> {
        fn next(&mut self) -> Option<Item> {
            Some(self.count)
        }
    }
    if true {
        let mut c = Counter { count: 1 };
        c.next();
    }
    /*
    实现impl trait时，associated type相比generic更简化：
    1. 以上generic实现要写`impl<Item> Iterator<Item>...`
    2. 用关联类型的话, 只需要写`impl Iterator`，如果要实现复杂的Item就很方便, 比如：

        pub trait Iterator: Clone + Default + fmt::Debug + Decodable + Encodable {
            type Item: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
            fn is_null(&self) -> bool;
        }
     */

    // 再比如： 如果用泛型traits 作为参数，你将得到以下的代码：

    //     trait Container<A,B> {
    //         fn contains(&self,a: A,b: B) -> bool;
    //     }

    //     fn difference<A,B,C>(container: &C) -> i32 // 再次声明依赖的ABC泛型
    //     where
    //         C : Container<A,B> {...}

    // 可以看到，由于使用了泛型，导致函数头部也必须增加泛型的声明，而使用关联类型，将得到可读性好得多的代码：

    //     trait Container{
    //         type A;
    //         type B;
    //         fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
    //     }

    //     fn difference<C: Container>(container: &C) {}
}

#[test]
fn trait_type_default() {
    use std::ops::Add;
    // trait Add<RHS=Self> {//default associated type is Self
    //     type Output;
    //     fn add(self, rhs: RHS) -> Self::Output;
    // }

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

#[test]
fn trait_type_default_change() {
    use std::ops::Add;
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        // change: RHS=Meters
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    let _a = Millimeters(1) + Meters(10);
}
mod trait_type_default_recursive {
    #[test]
    fn default_change_inner() {
        use std::ops::Sub;
        #[derive(Debug, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        // 有两个Output: T内部的Output, Point自己的Output。另外还有一个Sub<Rhs> 右值
        // impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T> {
        // impl<T: Sub<Output = T>> Sub<Self> for Point<T> {
        // impl<T: Sub<Output = T>> Sub for Point<T> {
        impl<T: Sub<Output = T>> Sub for Point<T> {
            // type Output = Point<T::Output>;
            // type Output = Point<T>;
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                Point {
                    x: self.x - other.x,// T -T -> T::Output = T
                    y: self.y - other.y,
                }
            }
        }
        assert_eq!(
            Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
            Point { x: 1, y: 3 }
        );
    }

    #[test]
    fn default_nochange_inner() {
        use std::ops::Sub;
        #[derive(Debug, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }
        // impl<T: Sub<Output=T>> 变成　impl<T: Sub> // 即T内部的Output 不改变
        impl<T: Sub> Sub for Point<T> {
            type Output = Point<T::Output>;
            fn sub(self, other: Self) -> Self::Output {
                Point {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }
    }
}

#[test]
fn trait_def_same_method() { //完全限定语法1
    //multiple traits has same method
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let person = Human {};
    //完全限定语法1
    Pilot::fly(&person); // 调用Pilot特征上的方法
    Wizard::fly(&person); // 调用Wizard特征上的方法
    person.fly(); // 调用Human类型自身的方法(如果没有这个方法，会报错)
}

#[test]
fn trait_def_same_method_without_self() { //完全限定语法
    //关联函数associated function 没有self
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        #[allow(dead_code)]
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    //完全限定语法
    println!("dog name:{}", <Dog as Animal>::baby_name()); 
}

#[test]
// Supertraits: 例如让一个特征 A 使用另一个特征 B 的功能。这种情况下，一个类型要实现特征 A 首先要实现特征 B， 特征 B 就被称为 supertrait
fn trait_type_constrait() {
    // 对特征定义的特征约束(参考trait_arg_type 中将参数约束为特征，则这里是对定义本身进行约束)
    // trait CompSciStudent: Programmer + Student {}
    use std::fmt::Display;
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            // Display 已实现了ToString: impl<T: fmt::Display + ?Sized> ToString for T {}
            let output = self.to_string(); // self 有Display特征(实现了to_string+fmt)
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }

    use std::fmt;

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // f.write_fmt(format_args!("({}, {})", self.x, self.y));
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

#[test]
fn traint_newtype() {
    //在外部为内部类型（非本地类型）添加内部特征
    /*
    例子: 我们有一个动态数组类型： Vec<T>，它定义在标准库中，还有一个特征 Display，它也定义在标准库中，
    如果没有 newtype，我们是无法为 Vec<T> 实现 Display 的: impl<T> std::fmt::Display for Vec<T>
    */
    use std::fmt;

    struct Wrapper(Vec<String>); //newtype here

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    //run
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    /*
    1. 不好的地方:
        你都无法直接调用，需要先用 self.0 取出数组，然后再进行调用。
    2. Rust 提供了一个特征叫 Deref:
        实现该特征后，可以自动做一层类似类型转换的操作，可以将 Wrapper 变成 Vec<String> 来使用。
     */
}
