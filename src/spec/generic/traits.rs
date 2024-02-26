#[allow(unused)]
use core::fmt::Debug;
#[allow(unused)]
use core::fmt::Display;


#[test]
fn trait_method() {
    // 定义trait method
    pub trait Summary {
        fn summarize(&self, prefix:&str) -> String;
    }
    // 定义结构体
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    // 实现trait 行为
    impl Summary for Weibo {
        fn summarize(&self, _pre:&str) -> String {
            format!("{}:{}发表了微博{}", _pre,self.username, self.content)
        }
    }

    let weibo = Weibo {
        username: String::from("张三"),
        content: String::from("今天天气不错"),
    };
    dbg!(weibo.summarize("_"));
}

#[test] // ?Sized 是一个特殊的 trait bound，表示类型可能不是 Sized 的(即动态大小类型, 如切片编译时大小未知)
fn trait_method_unsized_arg_associated() {
    //　模仿：[1, 2].get(0).unwrap();
    trait ListIndex<T: ?Sized> {
        type Output: Sized; //trait associated type, 这里定义一个关联类型占位泛型
        fn get(self, slice: &T) -> Option<&Self::Output>;
    }
    impl<T> ListIndex<[T]> for usize {//编译usize时,T类型不知道，所以需要?Sized
        type Output = T;
        fn get(self, slice: &[T]) -> Option<&Self::Output> {
            slice.get(self)
        }
    }
    // 使用1
    #[allow(unused)]
    let v = vec![String::from("中")];
    let v = [String::from("aaa"), String::from("b")];
    let index: usize = 1;
    let e = index.get(&v).unwrap();
    dbg!(&v[0]);
    dbg!(e);
}

#[test]
fn trait_method_default() {// default method
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    // 定义结构体
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    // 使用默认sumarize 行为
    impl Summary for Weibo {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
}

#[test]
fn trait_prelude_tryinto() {
    let a: i32 = 10;
    let b: u16 = 100;
    // 隐式调用　use std::convert::TryInto;
    // 因为常用的标准库中的特征通过 std::prelude 模块提前引入到当前作用域中，
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

#[allow(unused)]
#[test]
fn arg_trait_type() { //特征约束(trait bound): 参数被约束成Trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub fn notify(item1: &impl Summary, item2: &impl Summary) {
        println!("Breaking news! {},{}", item1.summarize(), item2.summarize());
    }
}

#[allow(unused)]
#[test] //特征约束(trait bound): 参数被约束成Trait generic
fn arg_trait_generic() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    // pub fn notify1(item1: &impl Summary, item2: &impl Summary) {}
    pub fn notify2<T: Summary>(item1: &T, item2: &T) {}
}

#[allow(unused)]
#[test] //多重约束（multiple bounds）
fn arg_trait_multi_bounds() {
    use core::fmt::Display;
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub fn notify1(item: &(impl Summary + Display)) {}
    pub fn notify2<T: Summary + Display>(item: &T) {}
}

#[allow(unused)]
#[test] // Where 约束 当特征约束变得很多时，函数的签名将变得很复杂：
fn arg_trait_multi_bounds_generic() {
    fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        1
    }
    //改进，通过 where: Constraint1 + Constraint2 + ... + ConstraintN
    fn some_function2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        1
    }
}

#[test] // 约束trait自己的Output
#[allow(unused)]
fn arg_trait_constraint_trait() {
    use std::ops;
    fn multiply<T: ops::Mul<Output = T>>(x: T, y: T) -> T {
        x * y
    }
}

#[allow(unused)]
#[test] //返回实现了特征的类型
fn return_trait() {
    trait Summary {
        fn summarize(&self) -> String {
            format!("(Read more ...)")
        }
    }
    impl Summary for Weibo {}
    pub struct Weibo {
        pub username: String,
    }

    //return trait
    fn returns_summarizable() -> impl Summary {
        Weibo {
            username: String::from("sunface"),
        }
    }
}
#[allow(unused)]
#[test] //返回实现了特征的类型
fn return_trait_different_dyn() {
    trait Animal {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }
    struct Sheep {}
    struct Cow {}
    impl Animal for Sheep {}
    impl Animal for Cow {}

    // Returns different struct that implements Animal
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
}


#[test]
fn case_add() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Point<T: Add<T, Output = T>> {
        //限制类型T必须实现了Add特征，否则无法进行+操作。
        x: T,
        y: T,
    }

    impl<T: Add<T, Output = T>> Add for Point<T> {
        type Output = Point<T>;

        fn add(self, p: Point<T>) -> Point<T> {
            Point {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }

    fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
        a + b
    }

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    // println!("{:?}", p1+ p2); //ok
    println!("{:?}", add(p1, p2));
}

#[test]
fn case_display() {
    use std::fmt;
    use std::fmt::Display;

    #[derive(Debug)]
    #[allow(unused)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    impl File {
        fn new(name: &str) -> File {
            File {
                name: String::from(name),
                data: Vec::new(),
            }
        }
    }

    impl Display for File {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "<{}>", self.name)
        }
    }

    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);
    println!("{}", f6);
}
