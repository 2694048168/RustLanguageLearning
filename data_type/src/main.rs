use std::mem::size_of;

fn main() {
    // -------- Data Types -------
    println!("--------------------------------------");
    // certain data type, which tells Rust what kind of data is being specified
    // so it knows how to work with that data.
    // data type subsets: scalar and compound.
    //  Rust is a statically typed language.
    // converted a 'String' to a numeric type using 'parse'
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Hello, the guess is {guess}");

    // -------- Scalar Types -------
    println!("--------------------------------------");
    // integers | floating-point numbers | Booleans | characters
    // Number literals can use '_' as visual separator to make number easier to read
    assert_eq!(1, size_of::<i8>());
    // assert_eq!(4, size_of::<u8>());
    assert_eq!(1, size_of::<u8>());

    assert_eq!(4, size_of::<i32>());
    assert_eq!(4, size_of::<u32>());

    // arch=x64, arch=x32 ---> 4 bytes
    assert_eq!(8, size_of::<usize>());
    assert_eq!(8, size_of::<isize>());

    const SIZE_I8: i8 = 127;
    const SIZE_U8: u8 = 127;
    println!("the sizeof of i8: {SIZE_I8}");
    println!("the sizeof of i8: {SIZE_U8}");

    // Floating-Point Types default=f64 because of Modern CPUs
    let var_double = 3.14;
    // let var_double: f64 = 3.14;
    let var_float: f32 = 3.14;
    // C++ 中 float 和 double 不相等! ^_^,
    // Rust 编译器好像会使得默认的 f64 转换为 f32, 由于下面的这条语句
    assert_eq!(var_double, var_float);

    // Numeric Operations similar as C++
    // + - * / %

    // Rust Booleans are one byte in size
    let condition = false;
    let is_condition: bool = true; /* explicit type annotation */
    if condition || is_condition {
        println!("the condition in Rust.");
    }

    // Rust’s char type is the language’s most primitive alphabetic type.
    // single quotes ---> char || double quotes ---> string
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z: char = 'ℤ'; /* explicit type annotation */
    let heart_eyed_cat = '😻';
    println!("the primitive alphabetic in Rust {c} {z} {heart_eyed_cat}");

    // -------- Compound Types -------
    println!("--------------------------------------");
    // Rust has two primitive compound types: tuples and arrays.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tuple_same = (6.18, 3.14, 2.0);
    println!("the same type tuple in Rust: {:#?}", tuple_same);
    println!("the same type tuple in Rust: {:?}", tuple_same);

    // destructuring : destructure a tuple value
    // 类似 Python 中的元组解包或元组拆包
    // C++17 中结构化绑定特性
    let (_, pi, _) = tuple_same;
    println!("the value of PI: {pi}");

    // Rust also can access a tuple element directly by using a period (.)
    let tuple_diff: (i32, f64, u8) = (500, 6.4, 1);
    println!("the different type tuple in Rust: {:?}", tuple_diff);
    let five_hundred = tuple_diff.0;
    let six_point_four = tuple_diff.1;
    let one = tuple_diff.2;
    println!("the tuple in Rust: {five_hundred} {six_point_four} {one}");

    // The tuple without any values has a special name, unit.
    // This value and its corresponding type are both written ()
    // and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.
    // Rust 中的 unit, 有点类似 Python 中 None 的感觉, 或者有一种 C++ 中 NULL 的感觉.
    let special_tuple = ();
    println!("the special tuple in Rust: {:?}", special_tuple);
    println!("the special tuple in Rust: {:#?}", special_tuple);

    // -------- The Array Type -------
    println!("--------------------------------------");
    // Unlike a tuple, every element of an array must have the same type.
    // arrays in Rust have a fixed length.
    // 思考一下 C++ 中数组一般都需要确定其大小, Array 与 Vector 的设计差异
    // want data allocated on the stack rather than the heap in Rust.
    let array_same = [1, 2, 3, 4, 5, 6];
    println!("the array in Rust: {:?}", array_same);
    // println!("the array in Rust: {:#?}", array_same);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // println!("the months in year: {months}");
    println!("the months in year: {:?}", months);

    // explicit the type and size of array.
    let array_integer: [i32; 6] = [0, 1, 2, 3, 4, 5];
    println!("the value of array: {:?}", array_integer);
    let array_float = [3.14; 3];
    // let array_float: [f32; 3] = [3.14; 3];
    println!("the value of array: {:?}", array_float);

    // An array is a single chunk of memory of a known,
    // fixed size that can be allocated on the stack.
    // can access elements of an array using indexing
    println!("the first element of array: {}", array_integer[0]);
    // println!("the first element of array: {}", array_float[3]);
    println!("the first element of array: {}", array_float[2]);

    // Rust 检查数组访问越界行为, 特别是运行时 runtime error ---> Rust will panic
    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
