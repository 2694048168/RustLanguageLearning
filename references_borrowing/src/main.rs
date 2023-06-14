fn main() {
    // References and Borrowing
    println!("-----------------------");
    let str = String::from("Wei Li");
    let len = calculate_length(&str);
    println!("The length of '{}' is {}.", str, len);

    println!("The String is valid after reference: {str}.");

    // Rust 中的 reference 和 C++ 中 reference 类似, 又有点不一样 ^_^
    // Rust called the action of creating a reference borrowing
    // change(&str);
    let mut str_mut = String::from("li wei");
    change(&mut str_mut);
    println!("The String is valid after mutable reference: {str_mut}.");

    // Rust 如何体现安全和避免同时修改同一个数据
    println!("-----------------------");
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("{r1}");

    {
        let r1 = &mut s;
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{r2}");

    // 不要忘记了 Rust 可以 Shadowing ^_^, 神奇的特性
    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
                 // println!("{}, {}, and {}", r1, r2, r3);
    println!("{} and {}", r1, r2);

    {
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    // Rust Dangling References
    println!("-----------------------");
    // Rust 编译器会保证不会出现 C++ 中悬挂指针的问题
    // let reference_to_nothing = dangle();
    // println!("{reference_to_nothing}");

    // Rust 编译过程和 C++ 有所不同, 当我们只注释掉这个有 error 的函数调用,
    // 在 C++ 编译中是可以通过的, 但是 Rust 会继续报错, 因为函数声明和实现是有 error 的
    // 我想这也是为什么 Rust 中函数的声明不需要一定在调用之前的理由
    
    let reference_to_something = no_dangle();
    println!("{reference_to_something}");
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

// reference borrowing
// fn change(some_string: &String) {
// Mutable References
fn change(some_string: &mut String) {
    some_string.push_str(", hello Rust");
}

// 没有转移所有权, heap data 被 drop 后, 无法返回了(borrowing 操作)
// fn dangle() -> &String {
//     let s = String::from("hello, Rust Dangling References");

//     &s
// }

// 直接转移所有权, heap data 被 move
fn no_dangle() -> String {
    let s = String::from("hello, Rust Dangling References");

    s
}
