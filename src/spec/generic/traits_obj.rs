#[test]
#[allow(unused)]
/*特征对象
1. 特征对象指向实现了 trait Draw 特征的类型的实例，也就是指向了 Button 或者 SelectBox 的实例，这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。
2. 可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。
    2.1 Box.new创建一个 Box<T> 类型的智能指针，指针指向的数据被放置在了堆上

rust 代码分发方式：
1. 静态分发(static dispatch): `Box<T>`, 编译器会为每一个泛型参数对应的具体类型生成一份代码，对于运行期性能完全没有任何影响。
2. 动态分发(dynamic dispatch): `Box<dyn Trait>`, 运行时，才能确定需要调用什么方法。trait代码中的关键字 dyn 正是在强调这一“动态”的特点
*/
fn trait_obj_dispatch() {
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    // 动态分发：dyn 关键字只用在特征对象的类型声明上，表明运行时才知道类型。在创建时无需使用 dyn
    fn dispatch_dyn(x: &dyn Draw) {
        x.draw();
    }
    // 动态分发box：传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
    // 1. 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    // 2. 注意：Box<dyn Draw> 没有实现trait Draw，它只是对 &dyn Draw 引用包装(注意：有dyn)
    fn dispatch_dyn_box(x: Box<dyn Draw>) {
        x.draw();
    }

    // 静态分发: Static dispatch generic
    fn dispatch_generic<T: Draw>(x: &T) {
        x.draw();
    }
    // 静态分发: Static dispatch generic box
    fn dispatch_generic_box<T: Draw>(x: Box<T>) {
        x.draw();
    }
    // 静态分发: Static dispatch impl
    fn dispatch_impl(x: &impl Draw) {
        //静态分发,impl 就是泛型的语法糖
        x.draw();
    }

    fn main() {
        let x = 1.1f64;
        let y = 8u8;

        // 动态分发：&dyn Draw
        dispatch_dyn(&x);
        dispatch_dyn(&y);
        // dispatch_dyn(Box::new(y)); // trait Draw is not implemented for `Box<u8>`

        // 动态分发: Box智能指针(Box<dyn Draw>)
        // Box创建一个智能指针,只不过它包裹的值会被强制分配在堆上
        dispatch_dyn_box(Box::new(x));
        dispatch_dyn_box(Box::new(y));

        // 静态分发：泛型T
        dispatch_impl(&y);
        dispatch_generic(&y);
        dispatch_generic_box(Box::new(y));
    }
    main();
}

#[test] // dyn prop for trait
fn trait_obj_prop() {//trait as porp
    //1. Draw
    trait Draw {
        fn draw(&self) -> String;
    }
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    // 2. iter Vec
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,// trait as prop
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                dbg!(component.draw());
            }
        }
    }
    // 3. run
    Screen {
        components: vec![Box::new(11), Box::new(22)],
    }
    .run(); // s.run();
}

#[test]
/*
下例是静态分发，编译Box<T> 时类型Box<u8>不生成trait Draw(u8本身是符合trait特征的)
 */
fn trait_obj_prop_generic() {
    //1. Draw
    trait Draw {
        fn draw(&self) -> String;
    }
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    #[allow(unused_allocation)]
    Box::new(11u8).draw();

    // 这一句不能省略: 因为 Box<Draw> 本身不符合trait Draw
    // 否则：the trait bound `Box<u8>: Draw` is not satisfied
    impl Draw for Box<u8> {
        fn draw(&self) -> String {
            format!("Box<u8>: {}", **self)
        }
    }

    // 2. iter Vec
    struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                dbg!(component.draw());
            }
        }
    }

    // 3. run
    let s = Screen {
        components: vec![
            Box::new(11u8), //静态 dispatch
                            // 12u8,
                            // 22,
        ],
    };
    s.run(); // s.run();
}

#[allow(unused)] // dyn特征对象实例的结构
fn trait_obj_struct() {
    /*
    refer to: https://course.rs/basic/trait/trait-object.html
    关于特征对象（dyn trait实例）的结构:
    1. 特征对象大小不固定: 对于特征 Draw，它的实例是不同的类型实现。
    2. 几乎总是使用特征对象的引用方式，如 &dyn Draw、Box<dyn Draw> ,它的引用类型的size是固定的: 即ptr+vptr
        2.1. ptr指向实现了特征 Draw 的具体类型的实例，如类型 Button/SelectBox 的实例
        2.2. vptr指向一个虚表 vtable，vtable 中保存了可以调用的实现于特征 Draw 的方法。当调用方法时，直接从 vtable 中找到方法并调用(因为不同的类型的方法实现不同的, 所以需要通过vtable动态查找)
         */
}

#[test]// 特征对象的限制
#[allow(unused)] 
fn trait_return_self_limit() {
    /*
    不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，它的对象才是安全的：
    1. 方法的返回类型不能是 Self(因为trait对象类型是不确定的)
    2. 方法没有任何泛型参数: 使用特征对象时其具体类型被抹去了,不知道参数类型是什么

    标准库中的 Clone 特征就不符合对象安全的要求：
    pub trait Clone {
        fn clone(&self) -> Self;
    }

    如果违反了对象安全的规则，编译器会提示你。例如:
    pub struct Screen {
        pub components: Vec<Box<dyn Clone>>,//Clone 签名不知道具体Self类型
      |     ^^^^^^^^^^^^^^^^^the trait `std::clone::Clone` cannot be made into an object
    }
     */

    // return trait self with Box<dyn>
    fn trait_self_with_box() {
        trait MyTrait {
            fn f(&self) -> Box<dyn MyTrait>;
        }
        impl MyTrait for String {
            fn f(&self) -> Box<dyn MyTrait> {
                Box::new(self.clone())
            }
        }
        fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {// box只在作为形参可接受dyn trait with Self
            x.f()
        }
        my_function(Box::new(String::from("abc")));
    }

    // return self with generic
    fn trait_self_with_generic() {
        trait MyTrait<T> {
            fn f(&self) -> T;
        }
        impl MyTrait<String> for String {
            fn f(&self) -> String {
                self.clone()
            }
        }
        fn my_function1<T:MyTrait<T>>(x: T) -> T { // T形参能接收trait对象返回自身
            x.f()
        }
        fn my_function2<T: MyTrait<T>>(x: Box<dyn MyTrait<T>>) -> T {// box T作为形参时可接受trait T
            x.f()
        }
        fn my_function3<T: MyTrait<T>>(x: Box<T>) -> T {// box T作为形参时可接受trait T
            x.f()
        }
        my_function1(String::from("abc"));
        my_function2(Box::new(String::from("abc")));
        my_function3(Box::new(String::from("abc")));
    }

    // 3. return trait self with impl(impl 是泛型的语法糖)
    fn trait_self_with_impl() {
        trait MyTrait {
            fn f(&self) -> Self;
        }
        impl MyTrait for String {
            fn f(&self) -> Self {
                self.clone()
            }
        }
        fn my_function(x: impl MyTrait) -> impl MyTrait { // impl形参就是泛型的语法糖
            x.f()
        }
        my_function(String::from("abc"));
    }

    //4. return trait self with type Output
    fn trait_self_with_output() {
        trait MyTrait {
            type Output;
            fn f(&self) -> Self::Output;
        }
        
        impl MyTrait for String {
            type Output = String;
            fn f(&self) -> Self::Output { self.clone() }
        }
        
        fn my_function<T: MyTrait<Output = T>> ( x: Box<dyn MyTrait<Output = T>>) -> T { // could be discarded
            x.f()
        }
        
        fn my_function_2<T: MyTrait<Output = T>> ( x: Box<T>) -> T { // could be discarded
            x.f()
        }
        fn my_function_3<T: MyTrait<Output = T>> ( x: T) -> T { // could be discarded
            x.f()
        }
        
        fn main() {
            my_function( Box::new( String::from("abc") ));
            my_function_2( Box::new( String::from("abc") ));
            my_function_3( String::from("abc") );
        }
    }

}

#[test]
#[allow(unused)]
fn trait_return_box_or_ref() {
    trait Summary {
        fn summarize(&self) -> String {
            format!("(Read more ...)")
        }
    }
    impl Summary for Weibo {}
    impl Summary for Twitter {}
    struct Weibo {}
    struct Twitter {}

    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     /*
    //     1. if else 要求返回类型一致: // if switch { Twitter {} } else { Weibo {} }
    //     */
    // }

    //  可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。
    fn return_box(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(Twitter {})
        } else {
            Box::new(Weibo {})
        }
    }
    fn return_ref(switch: bool) -> &'static dyn Summary {
        //返回static引用所有权
        if switch {
            &Twitter {}
        } else {
            &Weibo {}
        }
    }
}
