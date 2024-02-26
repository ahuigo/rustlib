#[test]
fn def_struct() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    // 1. init struct
    let p = Point { x: 5, y: 10 };

    // 2. move p
    fn print_point(point: Point) {
        println!("Point is at ({}, {})", point.x, point.y);
    }
    print_point(p); //move p

    // 3. 如何避免传参、返回时发生Move呢？就是borrowing：**获取变量的引用，称之为借用(borrowing)**, 允许你使用值，但不获取其所有权
    let p = &Point { x: 5, y: 10 };
    let p2 = p; // copy p pointer
    println!("{:?},{:?}", p, p2);
}

#[test]
#[allow(dead_code)]
fn init_struct() {
    #[derive(Debug)]
    struct User {
        name: String,
        age: i32,
        score: i32,
    }

    // 1.1 init struct without field name
    let name = "Alex".to_string();
    let u1 = User {
        name,
        age: 1,
        score: 100,
    };

    // 1.2 init struct with struct update syntax
    let _u2 = User { age: 2, ..u1 };
    // println!("{:?}", u1); // error: partial move occurs because `u1.name` has type `String`, which does not implement the `Copy` trait
    println!("{:?}", u1.age); // age 没有被move, 可以继续使用
}

#[test]
fn init_destructuring() {
    struct User {
        age: i32,
    }
    let u1 = User { age: 1 };

    let User { age: age1 } = u1;
    let User { age } = u1;
    println!("{},{}", age1, age);
}

#[test]
fn def_mut_struct() {
    #[derive(Debug)]
    #[allow(unused)]
    struct User {
        name: String,
        age: i32,
        score: i32,
    }

    let mut u1 = User {
        name: "Alex".to_string(),
        age: 1,
        score: 100,
    };
    u1.age = 1;
}

#[test]
fn def_tuple_struct() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    println!("{:?}", _black.0);
}

#[test] // Unit Struct has no data
fn def_unit_struct() {
    struct AlwaysEqual;

    let _subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {
    //     fn do_something(&self) {
    //         println!("I'm doing something");
    //     }
    // }
}


#[test]
fn move_struct() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Struct {
        a: i32,
        b: String,
    }
    fn make() -> Struct {
        let s = Struct {
            a: 3,
            b: String::from("hello"),
        };
        s // moved self
    }
    let s1 = make();
    let _s2 = s1; //moved
}

#[test]
fn borrow_struct_prop() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，1. move: person.name 的所有权被转移给新的变量 `name`
    // 2. borrow: 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}
