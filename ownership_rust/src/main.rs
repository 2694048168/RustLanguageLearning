// Ownership is a set of rules that govern how a Rust program manages memory.
// JAVA and C-sharp and Python 通过垃圾回收机制 (garbage collection, GC).
// C and C++ 需要 programmer explicitly allocate and free the memory.
// Rust memory is managed through a system of ownership with a set of rules that the compiler checks.
// C++ 也有 ownership 的概念, e.g. std::thread and std::move
fn main() {
    println!("Hello, world!");
    // the memory layout: stack and heap in Rust/C++/C
    // 万变不离其宗, a systems programming language like Rust/C++/C
    // 官方的这一段关于 stack and heap 的解释, 简单易懂
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    println!("The Stack and the Heap is best important for systems programming language");
    println!("-------------------------------");

    // Ownership Rules
    // 1. Rust 中每一个值都有一个所有者;
    // 2. 每一个值每一次只能属于一个所有者;
    // 3. 当所有者超出范围(scope), 则值被释放掉(dropped);

    // Variable Scope
    // Rust 这一点和 C++ 和 Python 的变量的作用域类似
    {
        let str = "hello"; // s is valid from this point forward
        println!("the str variable is valid: {str}");

        // do stuff with s
    } // this scope is now over, and s is no longer valid
      // println!("the str variable is NOT valid: {str}");
    println!("the str variable is NOT valid");

    // The String Type
    let var_str = String::from("string literal");
    println!("the string literal IS: {var_str}");

    // don't forget the Shadowing in Rust.
    let mut var_str = String::from("hello");
    // push_str() appends a literal to a String
    var_str.push_str(", world");
    println!("the string literal IS: {var_str}");

    // let mut var_literal = "hello";
    // var_literal = "world";

    // Memory and Allocation
    println!("-------------------------------");
    // Rust takes memory is automatically returned once the variable that owns it goes out of scope.
    // 这和 C++ 中明确的申请和释放内存范式不同; 也和 Python 的 GC 机制不同
    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("the string in heap valid: {s}");

        // do stuff with s
    } // this scope is now over, and s is no longer valid
      // println!("the string literal is NOT valid: {s}");
    println!("the string literal is NOT valid");
    // Rust 自动调用 drop 函数释放内存资源
    // 有点类似 C++ 中的 Resource Acquisition Is Initialization (RAII) patterns

    // Variables and Data Interacting with Move -----
    let x = 5;
    let y = x; // copy the value of x, and bind to y because of stack memory.
    println!("the VALUE of y: {y}");

    let s1 = String::from("Wei Li");
    println!(
        "the len of s1: {0}, and the capacity: {1}",
        s1.len(),
        s1.capacity()
    );
    let s2 = s1; // ? what happen! s1 move into s2 for heap data to avoid 'double free error'
                 // 类似 C++ 的移动语义 std::move
                 // println!("the VALUE of s1: {s1}");
    println!(
        "the len of s2: {0}, and the capacity: {1}",
        s2.len(),
        s2.capacity()
    );

    // Variables and Data Interacting with Clone -----
    // clone 有点借鉴 OpenCV 中 cv::Mat class 的感觉 ^_^, 深度拷贝 data on heap
    let s1 = String::from("Li Wei");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy -----
    println!("-------------------------------");
    // Rust has a special annotation called the Copy trait
    // that we can place on types that are stored on the stack.
    let var1 = 42;
    let var2 = var1;
    println!(
        "the valid value of var1 is {0}, and var2 is {1}",
        var1, var2
    );

    // -------- Ownership and Functions --------
    println!("-------------------------------");
    // Passing a variable to a function will move or copy, just as assignment does.
    // 该变量是否涉及 heap data ---> move or copy
    let var_ownership = String::from("weili_yzzcq@163.com");
    let var_copy = 42;

    takes_ownership(var_ownership);
    makes_copy(var_copy);

    // println!("the heap data is move: {var_ownership}");
    println!("the stack data is copy: {var_copy}");

    // -------- Return Values and Scope --------
    println!("-------------------------------");
    let str1 = gives_ownership();
    println!(
        "the len : {0}, and the capacity: {1}, and the value of str1: {2}",
        str1.len(),
        str1.capacity(),
        str1
    );

    let str2 = String::from("wei li");
    let str3 = takes_and_gives_back(str2);
    // println!("the value of str2: {str2}");
    println!("the value of str3: {str3}");
    
    // Rust does let us return multiple values using a tuple
    // 有点类似 Python 返回值所支持的感觉, C++ 不支持返回多个值!
    let var_return_multi = String::from("weili_yzzcq@163.com");
    let (value, length) = calculate_length(var_return_multi);
    println!("the value of String: {}, and the length is {}", value, length);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called.
  //  The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours wei li"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
