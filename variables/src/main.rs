// Constants are valid for the entire time a program runs
// the global 'const' variable
const FISH_MEMORY_SECONDS: u32 = 7;

fn main() {
    // -------- Variables and Mutability -------
    println!("--------------------------------------");
    // by default, variables are immutable.
    // let var_x = 25;
    let mut var_x = 25;
    println!("Hello, the value of variable: {var_x}");

    var_x = 66;
    println!("the value of variable: {var_x}");

    // -------- Constants -------
    println!("--------------------------------------");
    // declare constants using the const keyword instead of the let keyword,
    // and the type of the value must be annotated.
    // constants may be set only to a constant expression,
    // not the result of a value that could only be computed at runtime.
    //  Rust’s naming convention for constants is to use all uppercase
    //  with underscores between words.
    // The compiler is able to evaluate a limited set of operations at compile time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("the local const variable: {THREE_HOURS_IN_SECONDS}");
    println!("the global const variable: {FISH_MEMORY_SECONDS}");

    // -------- Shadowing -------
    println!("--------------------------------------");
    // Rust can declare a new variable with the same name as a previous variable.
    // the latter variable overshadows the forward, 
    // taking any uses of the variable name to itself 
    // until either it itself is shadowed or the scope ends.
    let var_y = 24;
    println!("the value of variable: {var_y}");
    
    let var_y = var_y + 1;
    
    {
        let var_y = var_y * 4;
        println!("the value of variable: {var_y}");
    }
    println!("the value of variable: {var_y}");

    // Shadowing is different from marking a variable as 'mut'.
    let spaces = "    ";
    let spaces = spaces.len();
    println!("the number of space: {spaces}");
    
    // shadowing 能够使得变量自动进行类型转换, 而 mut 变量不能.
    // let mut num_space = "    ";
    // num_space = num_space.len();
    // let num_space = num_space.len();
    // println!("the number of space: {num_space}");
}
