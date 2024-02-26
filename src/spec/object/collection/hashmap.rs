#[allow(unused_imports)]
use std::collections::HashMap;
/*
HashMap 也是集合类型:
1. HashMap 需要手动通过 use ... , 它不像String 和 Vec 并没有包含在 Rust 的 prelude 中
2. HashMap 也是内聚性的: K,V 必须是同一类型
2. HashMap 存储在堆上，通过栈上的引用来访问

    可以使用 HashMap::with_capacity(capacity) 创建指定大小的 HashMap，避免频繁的内存分配和拷贝，提升性能。
*/
mod init_hashmap {
    #[allow(unused_imports)]
    use std::collections::HashMap;
    #[test]
    fn init_hashmap() {
        // 1. with new
        let mut my_gems = HashMap::new();
        my_gems.insert("键1", 1);
        // my_gems["key"] = 2;// error: trait `IndexMut` is required to modify indexed content

        // 2. with value list
        let teams_list = vec![("中国队".to_string(), 100), ("美国队".to_string(), 10)];
        let _tmap: HashMap<_, _> = teams_list.into_iter().collect(); // support: slice/array/vec
        println!("{:?}", _tmap);
    }

    #[test]
    fn init_key_custom() {
        // key 必须实现HASH/Eq特征
        // 1. f32 和 f64 浮点数，没有实现 std::cmp::Eq 特征，因此不可以用作 HashMap 的 Key。
        // 2. 可以使用&str 配合get 查询 String key
        #[derive(Hash, Debug, Eq, PartialEq)]
        struct Viking {
            name: String,
            country: String,
        }

        impl Viking {
            fn new(name: &str, country: &str) -> Viking {
                Viking {
                    name: name.to_string(),
                    country: country.to_string(),
                }
            }
        }
        // 使用 HashMap 来存储 viking 的生命值
        let _vikings = HashMap::from([(Viking::new("Einar", "Norway"), 25)]);
    }

    #[test]
    fn init_move() {
        use std::collections::HashMap;

        let name = String::from("Sunface");
        let age = 18;

        let mut handsome_boys = HashMap::new();
        handsome_boys.insert(&name, age); //默认是move，改成引用

        // std::mem::drop(name);//error: it is borrowed  by boys
        println!("{:?}", handsome_boys);
    }
    #[test]
    fn init_capacity_len() {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
        map.insert(1, 2);
        map.insert(3, 4);
        // 事实上，虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
        assert!(map.capacity() >= 100);

        // 对容量进行收缩，你提供的值仅仅是一个允许的最小值，实际上，Rust 会根据当前存储的数据量进行自动设置，当然，这个值会尽量靠近你提
        map.shrink_to(50);
        assert!(map.capacity() >= 50);

        // 让 Rust  自行调整到一个合适的值，剩余策略同上
        map.shrink_to_fit();
        assert!(map.capacity() >= 2);
        println!("len:{},cap:{}", map.len(), map.capacity());
    }

    #[test]
    fn init_performance() {
        /*
        HashMap 使用的哈希函数是 SipHash，基于 Google 的 SwissTable:
        1. 它的性能不是很高: SipHash 在中等大小的 Key 上，性能相当不错，但是对于小型的 Key （例如整数）或者大型 Key （例如字符串）来说，性能还是不够好。
        2. 但是安全性很高: 对于抵抗 HashDos 攻击非常有效。
        若你需要极致性能，例如实现算法，可以考虑这个库：ahash。
        */

        /*
            // 指定第三方库
            use std::hash::BuildHasherDefault;
            use std::collections::HashMap;
            // 引入第三方的哈希函数
            use twox_hash::XxHash64;

            // 指定HashMap使用第三方的哈希函数XxHash64
            let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
            hash.insert(42, "the answer");
            assert_eq!(hash.get(&42), Some(&"the answer"));
            }
        */
    }
}

mod read_write {
    #[allow(unused_imports)]
    use std::collections::HashMap;
    #[test]
    fn hashmap_get_value() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        let team_name = String::from("Blue");

        let _s = scores[&team_name]; //move or copy
        let _s2: Option<&i32> = scores.get(&team_name); //borrow
        let _s3 = scores.get(&team_name).copied().unwrap_or(0); //copy
    }
    #[test]
    fn hashmap_get_key() {
        let scores: HashMap<&str, i32> = HashMap::new();
        if scores.contains_key("Daniel") {
            assert_eq!(scores["Daniel"], 10);
        }
    }

    #[test]
    fn hashmap_forin() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // borrow ref
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // move/copy
        for (key, value) in scores {
            println!("{}: {}", key, value);
        }
    }
    #[test]
    fn hashmap_write() {
        let mut scores = HashMap::new();

        scores.insert("Blue", 10);

        // 覆盖已有的值
        let old = scores.insert("Blue", 20);
        assert_eq!(old, Some(10));

        // 查询Yellow对应的值，若不存在则插入新值
        let v = scores.entry("Yellow").or_insert(5);
        // 用引用修改v的值
        *v += 1; //
        assert_eq!(scores["Yellow"], 6); // 插入5
    }
}
