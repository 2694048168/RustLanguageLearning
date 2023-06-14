fn main() {
    // An Example Program Using Structs
    println!("-----------------------------");

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_func(width1, height1)
    );

    // Refactoring with Tuples
    println!("-----------------------------");
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Refactoring with Structs: Adding More Meaning
    println!("-----------------------------");
    let rectangle_shape = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rectangle_shape)
    );

    // Adding Useful Functionality with Derived Traits
    println!("-----------------------------");
    // 类似需要 C++ 针对 << 输出流运算符进行重载的功能
    // println!("the instance object of struct: {rectangle_shape}");

    // the specifier :? inside the curly brackets tells println!
    // we want to use an output format called Debug trait
    println!("the instance object of struct: {:?}", rectangle_shape);
    println!("the instance object of struct: {:#?}", rectangle_shape);

    dbg!(rectangle_shape);

    // ? Another way to print out a value using the Debug format is to use the dbg! macro,
    // which takes ownership of an expression,
    // prints the file and line number of where that dbg! macro call occurs
    // in your code along with the resultant value of that expression,
    // and returns ownership of the value.
    // ! Note: Calling the dbg! macro prints to the standard error console stream (stderr),
    // ! as opposed to println!, which prints to the standard output console stream (stdout).

    println!("-----------------------------");
    let scale = 4;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area_func(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
