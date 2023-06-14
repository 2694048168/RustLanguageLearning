fn main() {
    // ------- Defining and Instantiating Structs -------
    println!("-----------------------");
    println!("struct in Rust");

    // instance struct in Rust
    let user = User {
        active: true,
        username: String::from("Wei Li"),
        email: String::from("weili_yzzcq@163.com"),
        sign_in_count: 1,
    };

    println!(
        "struct value member: {} {} {} {}",
        user.active, user.username, user.email, user.sign_in_count
    );

    let mut user_var = User {
        active: true,
        username: String::from("Wei Li"),
        email: String::from("weili_yzzcq@163.com"),
        sign_in_count: 1,
    };

    user_var.active = false;
    println!(
        "struct value member: {} {} {} {}",
        user_var.active, user_var.username, user_var.email, user_var.sign_in_count
    );

    let author = build_user("email@.com".to_string(), "author".to_string());
    println!(
        "struct value member: {} {} {} {}",
        author.active, author.username, author.email, author.sign_in_count
    );

    // Creating Instances from Other Instances with Struct Update Syntax
    // 类似 C++ 中赋值构造函数 assignment construction function
    let user2 = User {
        active: user.active,
        username: user.username,
        email: String::from("another@example.com"),
        sign_in_count: user.sign_in_count,
    };
    println!(
        "struct value member: {} {} {} {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    // 这样似乎会出现 Error, 因为 user 所有权被 move 了, 那么上面 user2 里面就不能使用 user.username
    // C++ 代码不会出现这种问题,即使存在数据被 move 了, 因为在前面被使用的, 但是 Rust 不行
    // 如同 C++ 函数需要前向声明才能调用, 而 Rust 不需要前向声明即可调用,
    // 这样有差异的本质原因, 可能是 Rust 和 C++ 在编译原理上设计是不同的.
    let user3 = User {
        email: String::from("another@example.com"),
        // ..user
        ..user2
    };
    println!(
        "struct value member: {} {} {} {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );

    // Using Tuple Structs Without Named Fields to Create Different Types
    println!("-----------------------");
    let black = Color(0, 0, 0);
    println!("black RGB: {}-{}-{}", black.0, black.1, black.2);

    let three_dim_coordinate = Point(3.1, 2.33, 0.0);
    println!(
        "the 3D space coordinate: ({}, {}, {})",
        three_dim_coordinate.0, three_dim_coordinate.1, three_dim_coordinate.2
    );

    // Unit-Like Structs Without Any Fields
    // unit-like structs because they behave similarly to ()
    // tuple in Rust 也有类似的 unit-tuple
    // We wouldn’t need any data to implement that behavior!
    // Rust 设计这种特殊的语法, 有点类似 Python 中 None 的使用方式
    let unit_struct = AlwaysEqual;
    println!("the unit-struct: {:?}", unit_struct);

    // Ownership of Struct Data
}

// define the Struct in Rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// pure function to init 'User' struct
// Using the Field Init Shorthand, C++ 中构造函数通过 this 指针解决该问题
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username,
        username,
        // email: email,
        email,
        sign_in_count: 1,
    }
    // Rust 中表达式作为返回值, 语句(statement)和表达式(expression)的差异
    // };
}

struct Color(i32, i32, i32);
struct Point(f32, f32, f32);

#[derive(Debug)]
struct AlwaysEqual;
