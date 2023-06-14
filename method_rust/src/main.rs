fn main() {
    // Method Syntax in Rust
    // Unlike functions, methods are defined within the context of a struct/enum/trait object,
    // and their first parameter is always self,
    // which represents the instance of the struct the method is being called on.
    // 联想 C++ 的类的成员函数和普通函数的区别, 如何使得函数操作封装的数据
    // self 语法有点类似 Python 中实例化也是 self
    println!("----------------------------");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
        // height: 0,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width_height() {
        println!(
            "The rectangle has a nonzero width/height; it is ({}, {})",
            rect1.width, rect1.height
        );
    }

    // C++ 中 object->something() is similar to (*object).something() 的使用范式
    // Rust 自动实现 automatic referencing and dereferencing.

    // Methods with More Parameters
    println!("----------------------------");
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // Associated Functions
    // Rust 如何实现类似 C++ 中类的静态方法 static member function, 不需要 self
    // 就是依赖于(绑定)特定的数据类型, 但是不依赖于实例化对象
    // 又有一点类似 C++ 中类的友元函数的感觉, Rust 'TMD' 真神奇 ^_^
    println!("----------------------------");
    let sq = Rectangle::square(32);
    println!("The area of square is {}", sq.area());

    // Multiple impl Blocks
    // Rust 允许针对一个自定义封装的数据类型有多个 impl 进行关联,也可以写在一个 impl 块里面
    // 感觉没有必要, 根据 C++ 类的实现, 方法全部写在一个 impl 块里面不方便吗？
    // 也可能有其他新的应用, 期待后面的阐述!
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 方法的实现与自定义的某中数据封装形式进行绑定
// 从而实现方法可以操纵封装的数据
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 思考如何实现 C++ 那种数据是 private, 而方法是 public
    // 类似 getter and setter 方法
    fn width_height(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    // 有一种实现 C++ 中运算符(>=/>)重载的感觉 ^_^
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
