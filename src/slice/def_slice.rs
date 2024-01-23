#[allow(unused_imports)]
use std::mem;

/*
1. array 是基础类型：固定长度、类型相同的元素集合，是栈上分配的
2. Vector:动态数组, 可以增长和缩短，是堆上分配的 
2. String:动态数组 
 */
#[allow(dead_code)]
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

#[test]
fn def_array() {
    let a = [9, 8, 7, 6, 5];
    let a1 = [1;5]; //repeated 5 times
    dbg!(a1);
    let _first = a[0]; // 获取a数组第一个元素    
}
#[test]
fn def_array_repeated_string() {
    // let a = ["Hi!".to_string(); 8]; //Err: `Copy` trait is required
    let array: [String; 8] = std::array::from_fn(|_i| String::from("Hi!"));
    println!("{:#?}", array);
}

#[test]
fn def_array_slice() {
    // 1. define array
    let a = [1, 2, 3, 4, 5];
    // Fixed-size array (type signature is superfluous).
    let _xs: [i32; 5] = [1, 2, 3, 4, 5];
    // All elements can be initialized to the same value:100.
    let ys = [100; 5];
    for num in ys.iter() {
        println!("array:{}", &num);
    }

    // 2. define slice
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("{}", slice == &[2, 3]);
}

#[test]
fn def_array_byte(){
    let _bs: &[u8; 21] = b"this is a byte string";
    let _bs2 = b"this is a byte string";
}
#[test]
fn read_array(){
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", ys.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs)); //20=5*4

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}


#[test] // runtime　error with index
fn read_array_index_out(){
    use std::io;
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
#[test] // runtime no　error with Option
fn read_array_index_option(){
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let _name1 = names.get(0).unwrap();
    let n2 = names.get(2);
    if let Some(name2) = n2 { //safe check
        println!("{}", name2);
    } else {
        println!("None");
    } 
}

/** ******* slice ********************/ 
#[test]
fn slice_array_len(){
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..3];
    
    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，3个字符为12字节。
    assert_eq!(std::mem::size_of_val(&arr), 12);
    // 一个切片引用占用了2个字大小的内存空间( 从现在开始，为了简洁性考虑，如无特殊原因，我们统一使用切片来特指切片引用 )。 该切片的第一个字是指向数据的指针，第二个字是切片的长度。字的大小取决于处理器架构，例如在 x86-64 上，字的大小是 64 位也就是 8 个字节
    assert_eq!(std::mem::size_of_val(&slice), 16);
}