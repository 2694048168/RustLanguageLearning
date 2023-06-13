fn main() {
    println!("Hello, world!");
    println!("------------------------------");
    // if Expressions
    let condition: u32 = 3;
    // let condition = condition * 3;

    if condition > 5 {
        println!("the condition is true.");
    } else {
        println!("the condition is false.");
    }

    // let num = 3;
    // let num = 1;
    let num = true;
    // 作为条件表达式的结果, Rust 和 C++ 有点不一致
    // 这样的设计似乎也是合理的, 作为条件表达式结果, 应该明确为 bool 类型
    if num {
        println!("the number is estimated true.");
    }

    let number = 2;
    // let number = false;
    // Rust 中条件表达式, 不建议像 C++ 和 Python 那样进行 '()' 进行评估,
    // ! 运算符的优先级问题？Rust 难道不存在这个问题吗？
    // if (number != 0) {
    if number != 0 {
        println!("number was something other than zero");
    }

    // ---------- Handling Multiple Conditions with else if ------------
    println!("------------------------------");
    // 牢记 Rust 中 shadowing 机制允许重复声明变量
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    // 类似 C++ 中 三目运算符
    let condition = true;
    // Remember that blocks of code evaluate to the last expression in them
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" };

    // 变量必须有一个单一的类型(两个分支具有相同的类型), Rust需要在编译时明确地知道数字变量的类型
    // 知道变量的类型可以让编译器在使用该变量的任何地方验证该类型是否有效。
    // 如果变量的类型只在运行时确定，Rust就无法做到这一点；
    // 如果编译器必须跟踪任何变量的多个假设类型，那么它将更加复杂，并且对代码的保证更少。
    // 这一点设计理念和 C++ 的设计理念不一致

    println!("The value of number is: {number}");

    // ---------- Repetition with Loops ------------
    println!("------------------------------");
    // Rust has three kinds of loops: loop, while, and for.
    // Repeating Code with loop
    loop {
        println!("无限循环");
        // comment the follow line, will be infinite loop.
        break;
    }
    // break and continue 的使用语义和 C++ 和 Python 中是一致的

    // Returning Values from Loops.
    // the break expression you use to stop the loop;
    // that value will be returned out of the loop so you can use it
    // Rust 中 break 表达式的值, 有点类似 C++ 和 Python 中的 return 的感觉
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 12;
        }
    };
    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    // C++ 中是通过 {} 方式来进行双层 for 循环的, 没有歧义
    // Python 中是通过 标准缩进 方式来进行双层 for 循环的, 没有歧义
    let mut count = 0;
    // set the break label, 有一种C/C++中 goto 的感觉
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break INTO label
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    println!("------------------------------");
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF");

    const ARRAY_NUMBER: [u32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", ARRAY_NUMBER[index]);
        index += 1;
    }

    // Looping Through a Collection with for
    println!("------------------------------");
    // 有一种 C++ 中 for-range 的感觉
    // 也有一种 Python 中 enumerate 的感觉
    for element in ARRAY_NUMBER {
        println!("the value is: {element}");
    }
    
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF");
    
    println!("------------------------------");
    // 类似于 C++ 中 STL 的容器的迭代器和算法
    for element in ARRAY_NUMBER.iter().rev() {
        println!("the value is: {element}");
    }
}
