// The concepts of ownership, borrowing, and slices 
// ensure memory safety in Rust programs at compile time. 
// Rust 这三个概念保证在编译时期的内存安全和高效!
fn main() {
    // ------- The Slice Type -------
    println!("Hello, world!");
    println!("-----------------------");
    // Slices let you reference a contiguous sequence of elements in a collection
    //  rather than the whole collection.
    // ? 想想 Python 的 slices 操作, 无论是对字符串还是数组等内存连续存储的数据
    // A slice is a kind of reference, so it does not have ownership.

    let mut s = String::from("hello Rust Slice");
    let word_idx = first_word(&s);
    println!("{}", word_idx);
    // println!("{}", s[word_idx]); // ! compile error, why?
    // Because word isn’t connected to the state of s at all.

    s.clear(); // this empties the String, making it equal to ""

    // ------- String Slices -------
    println!("-----------------------");
    let str = String::from("hello Rust");
    let hello = &str[0..5];
    let rust = &str[6..10];
    println!("the first slice is: {hello}");
    println!("the second slice is: {rust}");

    // [starting_index..ending_index],
    // where starting_index is the first position in the slice
    // and ending_index is one more than the last position in the slice.
    // Rust slice 索引规则和 Python slice 的索引规则类似, 所有有一些缺省的默认使用方式

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("the SAME slice is: {slice}");
    let slice = &s[..2]; // same as the above line slice.
    println!("the SAME slice is: {slice}");

    let len = s.len();
    let slice = &s[3..len];
    println!("the SAME slice is: {slice}");
    let slice = &s[3..]; // same as the above line slice.
    println!("the SAME slice is: {slice}");

    let slice = &s[0..len];
    println!("the SAME slice is: {slice}");
    let slice = &s[..]; // same as the above line slice.
    println!("the SAME slice is: {slice}");

    // ------- Slices application -------
    println!("-----------------------");
    let mut str_slice = String::from("Rust, hello world");
    // let str_slice = String::from("Rust, hello world");
    let word = first_word_slice(&str_slice);
    println!("the first word is: {}", word);

    str_slice.clear();
    // println!("the first word is: {}", word);
    // 从这里可以看出 slice 是和其被 slice 的对象是绑定的,相关的, 符合想要的要求和设计
    // 我们应该仔细看看 Rust 的 slice 功能的内存布局设计

    // ------- String Literals as Slices -------
    let s: &str = "Hello, world!";
    // 字符串的字面值, 本质就是一个不可变的字符串切片,
    // &str is an immutable reference.
    println!("the String Literals is: {s}");

    // ------- String Slices as Parameters -------
    let str = String::from("hello RUST programming world");
    let word = second_word(&str); // from String to string slice default.
    println!("the second word is: {}", word);

    // ------- Other Slices -------
    println!("-----------------------");
    let arr = [1, 2, 3, 4, 5];
    let array_slice = &arr[1..3];
    assert_eq!(array_slice, &[2, 3]);
    // assert_eq!(array_slice, &[1, 2]);
}

// 数组索引方法返回索引
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("the idx and item value: {i}-{item}");

        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 利用 slice 切片索引方法
// Rust 好像不支持类似 C++ 那样的函数重载 ^_^
// fn first_word(s: &String) -> &str {
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Defining a function to take a string slice instead of a reference
// to a String makes our API more general
// and useful without losing any functionality
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut count = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count += 1;
            if count == 2 {
                return &s[0..i];
            }
        }
    }

    &s[..]
}
