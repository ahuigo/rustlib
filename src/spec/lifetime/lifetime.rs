/*
lifetime的作用：
1.　避免悬垂引用(dangling reference)

示例对比a生命周期、b生命周期：

    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
*/

mod lifetime_rule {
    /*
    生命周期消除规则:
    1. 对于参数：每一个引用参数都会获得独自的生命周期
        例如两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推。
    2. 对于输出(1个参数)：
        例如函数 fn foo(x: &i32) -> &i32等同于 fn foo<'a>(x: &'a i32) -> &'a i32
    3. 返回值: 返回值中的引用，它的生命周期要么跟某个引用参数相同，要么是 'static
        3.0 对于输出(0个参数）相当于static: fn _s<'a>() -> &'a str { "foo" }, 返回值的生命周期是函数的生命周期
        3.0 对于输出(单个参数）若只有一个输入生命周期(函数参数中只有一个引用类型)，所有返回值的生命周期都等于该输入生命周期
        3.1 对于输出(多个参数）若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
        3.2 对于输出(多个参数）其它情况，要手动标注生命周期: fn longest(x: &str, y: &str) -> &str// 不确认是x还是y的生命周期
    4. 避免引用:
        1. 使用move/copy代替引用borrow
    */
    #[test] //1. 用Move/Copy 代替引用(borrow)
    fn return_with_move() {
        fn _mystr<'a>() -> String {
            //通过move避免悬垂引用(avoid　dangling reference for &str)
            String::from("really long string")
        }
    }
}

#[test]
fn lifetime_func() {
    // 'a是两个参数 x 和 y的 生命周期的交集： 返回值的生命周期是x和y的作用域重合的部分 （这样才是安全的）
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let x = string1.as_str();
    {
        let string2 = String::from("xyz");
        let result = longest(x, string2.as_str());
        println!("The longest string is {}", result); //x,y重合作用域在这里结束
    }
}

mod lifetime_struct {
    #[test]
    fn lifetime_struct_define() {
        struct ImportantExcerpt<'a> {
            part: &'a str, //part引用的str必须比这个结构体的实例活得更久
        }
        let i;
        {
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            i = ImportantExcerpt {
                part: first_sentence,
            };
            println!("{:?}", i.part);
        }
    }

    #[test]
    fn lifetime_struct_init() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Example<'a, 'b> {
            //b 的生命周期>=结构体的周期
            a: &'a u32,
            b: &'b i32,
        }

        /* method list:
        fn _fix_me<'a, 'b>(foo: &Example<'a, 'b>) -> &'b i32 {
        fn _fix_me<'a: 'b, 'b>(foo: &'a Example) -> &'b i32{}
        fn _fix_me<'b>(foo: &Example<'_, 'b>) -> &'b i32 {
        fn _fix_me<'a>(foo: &'a Example) -> &'a i32{
        */
        fn _fix_me<'a>(foo: &'a Example) -> &'a i32 {
            //b的生命周期>=a, 直接用‘a 代替’b
            foo.b
        }
    }
}

mod lifetime_method {
    #[test] //method lifetime的语法跟泛型参数语法很相似：
    fn lifetime_method_self() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        impl<'a> ImportantExcerpt<'a> {
            //方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则
            fn return_part(&self, note: &str) -> &str {
                //第1,3规则: <'b>(&'a self, note: &'b str) -> &'a str
                println!("Attention please: {}", note);
                self.part
            }
        }
        let _a = ImportantExcerpt { part: "a" }.return_part("eat");
    }
}

#[test] // 生命周期约束
fn lifetime_constraint() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    // 由于 &'a self 是被引用的一方，因此引用它的announcement &'b str 必须要活得比它短，否则会出现悬垂引用
    // 即'a 被'b使用，比'b活得更久
    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    impl<'a> ImportantExcerpt<'a> {
        fn _s1<'b>(&'a self, _second: &'b str) -> &'b str
        where
            'a: 'b,
        {
            self.part
        }
    }
    impl<'a, 'b> ImportantExcerpt<'a> {
        fn _s2(&'a self, _second: &'b str) -> &'b str
        where
            'a: 'b,
        {
            self.part
        }
    }
    let _a = ImportantExcerpt { part: "a" }.announce_and_return_part("eat");
}

mod lifetime_static {
    #[test]
    fn static_const() {
        fn _f() -> &'static str {
            // 被硬编码进 Rust 的二进制文件中，因此这些字符串变量全部具有 'static 的生命周期
            let _b: &'static str = "aa";
            let _b = "aa"; //The reference of const string　隐式具有'static生命周期
            "aa"
        }
        fn _f2<'a>() -> &'a str {
            // 无参数时，等价static, 返回值跟函数活得一样长
            "aa"
        }
        /* 只有const ref 才能用static，以下是错误的, 因为String 不是const
           fn _fn() -> &'static String {
               &String::from("foo")
           }
        */
    }

    #[test]
    fn static_generic() {
        // T:'static in https://course.rs/advance/lifetime/static.html
        use std::fmt::Display;
        fn print<T: Display + 'static>(message: &T) {
            println!("{}", message);
        }
        let mark_twain = "Samuel Clemens";
        print(&mark_twain);
        print(&"mark_twain");
    }
}
