// the main function, which is the entry point of many programs.
fn main() {
    println!("Hello, world!");
    // call function.
    print_function();

    // parameters of function in the signature.
    // the 'concrete values passed' in when you call a function.
    // parameter || argument
    // 和 C++ 中一样, 函数形参和实参, 默认为值传递(value copy)
    print_integer(42);

    print_key_value(0, 'L');

    // Statements and Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // Functions with Return Values
    let num = secret_number();
    println!("The value of num is: {num}");

    let x = plus_one(21);
    println!("The value of X is: {x}");

    println!("-----------------------------");
    // Comments in Rust
    // simple single line comment
    // 注释和 C++ 一致, 建议只使用 '//' 方式
}

// Rust doesn’t care where you define your functions,
// only that they’re defined somewhere in a scope that can be seen by the caller.
// 这和 C++ 函数的声明有差异, C++ 必须要在调用的前部声明才能使用.
fn print_function() {
    println!("the print function.");
}

// parameters of function in the signature.
fn print_integer(integer: i32) {
    println!("the value of integer: {integer}");
}

// parameter list
fn print_key_value(key: u32, value: char) {
    println!("the value of key {key} is {value}");
}

// Functions with Return Values
fn secret_number() -> i32 {
    42
}

// Rust 的函数返回值, 与 C++ 有所不同, 以最后一个表达式的值作为返回值
fn plus_one(x: i32) -> i32 {
    // x + 1; /* error, statement */
    x + 1 /* Expression result as return value  */
}
