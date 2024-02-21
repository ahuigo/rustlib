#[test] // ?Sized 是一个特殊的 trait bound，表示类型可能不是 Sized 的(即动态大小类型, 如切片编译时大小未知)
fn trait_type_associated() {//关联类型(associated type): trait 声明一个待定类型
    pub trait Iterator {
        type Item;
    
        fn next(&mut self) -> Option<Self::Item>;
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
        let mut c = Counter{count: 1};
        c.next();
    }

}
#[test] 
fn trait_type_generate() {// 用generic　代替关联type
    pub trait Iterator<Item> {
        fn next(&mut self) -> Option<Item>;
    }
    struct Counter<Item> {
        count: Item,
    }
    impl<Item: Copy> Iterator<Item> for Counter<Item> {// 实现impl 更复杂
        fn next(&mut self) -> Option<Item> {
            Some(self.count)
        }
    }
    if true {
        let mut c = Counter{count: 1};
        c.next();
    }
    /*
    generic vs associated type：
    1. 以上generic实现要写`impl<Item> Iterator<Item>...`
    2. 用关联类型的话, 只需要写`impl Iterator`，如果要实现复杂的Item就很方便, 比如：

        pub trait Iterator: Clone + Default + fmt::Debug + Decodable + Encodable {
            type Item: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
            fn is_null(&self) -> bool;
        }
     */

// 再比如： 如果使用泛型，你将得到以下的代码：

//     trait Container<A,B> {
//         fn contains(&self,a: A,b: B) -> bool;
//     }

//     fn difference<A,B,C>(container: &C) -> i32
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