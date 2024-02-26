/*
集合(collection)与语言级别的数组、字符串类型不同, 集合是在标准库中定义的:
1. 集合分配在堆上，因此都可以进行动态的增加和减少。
2. Vector 集合大小、宽度、高度竟然全部一致。与数组不同:
    1. 它大小是可以动态调整的, slice/array 创建后是固定大小的
    2. 它负责管理其内容;　切片本身不拥有其内容: 切片只是一个引用到另一个序列（如数组或 Vec<T>）的部分或全部。，
    2. 它可以存储任意类型的数据(如果类型不同，就需要用枚举类型、特征对象包裹成同一个类型)
    3. 从0实现动态数组Vector: https://nomicon.purewhite.io/vec/vec.html
3. HashMap 类型，该类型允许你在里面存储 KV 对，每一个 K 都有唯一的 V 与之配对。
4. String 集合
 */
#[test]
fn init_vector() {
    // 1. with new type
    let _: Vec<i8> = Vec::new();

    // 2. with new value
    let mut v = Vec::new();
    v.push(1u8);

    // 3. with macro
    let _v = vec![1, 2, 3];
    // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
    let _v = vec![1, 2, 3];
    let _v = vec![0; 3]; // 默认值为 0，初始长度为 3
}
#[test]
//只要为 Vec 实现了 From<T> 特征，那么 T 就可以被转换成 Vec。
fn init_from_into() {
    //4. with from
    let arr = [1, 2, 3];
    let v1: Vec<i32> = arr.into();
    let v2 = Vec::from(arr);
    let _v2 = arr.to_vec();
    assert_eq!(v1, v2);

    // String
    let s = "hello".to_string();
    let _v1: Vec<u8> = s.clone().into();
    let _v2 = Vec::from(s.clone());
    let _v3 = s.clone().into_bytes();

    // 迭代器 Iterators 可以通过 collect 变成 Vec
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);
}

#[test]
fn init_vector_capacity() {
    // with capacity: 其实Rust 的vec容量调整策略是加倍，例如 2 -> 4 -> 8 ..)。
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]); // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    // reserve
    v.reserve(100); // 调整 v 的容量，至少要有 100 的容量
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    //shrink
    v.shrink_to_fit(); // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );
}

mod init_vector_multi_type {
    #[test]
    fn vector_multi_type_enum() {
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let v = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];

        for ip in v {
            println!("{:?}", ip);
        }
    }

    #[test]
    fn vector_multi_type_trait_dyn() {
        trait IpAddr {
            fn display(&self);
        }
        struct V4(String);
        impl IpAddr for V4 {
            fn display(&self) {
                println!("ipv4: {:?}", self.0)
            }
        }
        struct V6(String);
        impl IpAddr for V6 {
            fn display(&self) {
                println!("ipv6: {:?}", self.0)
            }
        }

        //multi trait dyn(表示存储的是特征对象：对象是动态生成的)
        let _v: Vec<&dyn IpAddr> = vec![&V4("127.0.0.1".to_string()), &V6("::1".to_string())];
        //multi box trait dyn
        let _v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];
    }
}

mod update_vector {
    #[test]
    fn vec_push() {
        // 1. push
        let mut v = Vec::new();
        v.push(1);
        v.extend([2, 3]); // 附加数据到 v
        dbg!(v);
    }

    #[test]
    fn vec_operate() {
        let mut v = vec![1, 2];
        assert!(!v.is_empty()); // 检查 v 是否为空

        v.insert(2, 3); // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3]
        assert_eq!(v.remove(1), 2); // 移除指定位置的元素并返回, v: [1, 3]
        assert_eq!(v.pop(), Some(3)); // 删除并返回 v 尾部的元素，v: [1]
        assert_eq!(v.pop(), Some(1)); // v: []
        assert_eq!(v.pop(), None); // 记得 pop 方法返回的是 Option 枚举值
        v.clear(); // 清空 v, v: []

        let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
        v.append(&mut v1); // 将 v1 中的所有元素move到 v 中, v1: []
        v.truncate(1); // 截断到指定长度，多余的元素被删除, v: [11]
        v.retain(|x| *x > 10); // 保留满足条件的元素，即删除不满足条件的元素

        let mut v = vec![11, 22, 33, 44, 55];
        // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
        let mut m: Vec<_> = v.drain(1..=3).collect();

        let v2 = m.split_off(1); // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
        dbg!(m, v2);
    }
}

mod read_vec {
    #[test]
    fn read_by_index() {
        // 1. by index
        let v = vec![1, 2, 3, 4, 5];
        let _third: &i32 = &v[2];
        let _third: Option<&i32> = v.get(2);
    }

    #[test]
    fn read_range() {
        let v = vec![11, 22, 33, 44, 55];
        let slice = &v[1..=3];
        assert_eq!(slice, &[22, 33, 44]);
    }

    #[test]
    fn read_loop() {
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 10
        }
        for i in &v {
            println!("{i}");
        }
    }
}

#[test]
fn borrow_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; //immutable borrow
    println!("The first element is: {first}"); // ok
    v.push(6); //mutable borrow.
               // println!("The first element is: {first}"); //first can't be used later
}

mod vec_sort {
    #[test]
    fn vec_sort_int() {
        let mut v = vec![1, 5, 3, 2, 4];
        // v.sort();
        v.sort_unstable();
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn vec_sort_float() {
        let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
        // float 中没有实现 Ord 特征(ordering)，只实现了 PartialOrd(不能比较NaN)
        vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
    }
    #[test]
    fn vec_sort_struct() {
        #[derive(Debug)]
        struct Person {
            #[allow(unused)]
            name: String,
            age: u32,
        }

        impl Person {
            fn new(name: String, age: u32) -> Person {
                Person { name, age }
            }
        }

        // run
        let mut people = vec![
            Person::new("Zoe".to_string(), 25),
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
        ];
        // 定义一个按照年龄倒序排序的对比函数
        people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

        dbg!(people);
    }
    #[test]
    fn vec_sort_struct_with_derive() {
        // 通过derive 让Person类型实现Ord特征: 先按照name排序，再按照age排序
        #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
        struct Person {
            name: String,
            age: u32,
        }

        impl Person {
            fn new(name: String, age: u32) -> Person {
                Person { name, age }
            }
        }

        // run
        let mut people = vec![
            Person::new("Zoe".to_string(), 25),
            Person::new("Al".to_string(), 60),
            Person::new("Al".to_string(), 30),
            Person::new("John".to_string(), 1),
            Person::new("John".to_string(), 25),
        ];

        people.sort_unstable();

        println!("{:?}", people);
    }
}
