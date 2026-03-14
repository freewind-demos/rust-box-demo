fn main() {
    println!("=== Rust Box 堆内存分配演示 ===\n");

    // 1. Box 的基本使用
    // Box<T> 在堆上存储数据，栈上存指针

    let b = Box::new(5);
    println!("Box<i32>: {}", b);

    let s = Box::new(String::from("Hello, Box!"));
    println!("Box<String>: {}", s);

    // 2. Box 用于递归类型
    println!("\n--- 递归类型 ---");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))));
    println!("链表: {:?}", list);

    // 3. Box 作为函数返回值
    let boxed = create_boxed();
    println!("返回的 Box: {}", boxed);

    // 4. Box::leak 泄漏堆内存
    println!("\n--- Box::leak ---");
    let leak = Box::new(String::from("永远不会释放"));
    let leaked: &'static str = Box::leak(leak);
    println!("泄漏的字符串: {}", leaked);
    // 字符串在整个程序运行期间都有效

    // 5. Box 解引用
    let x = Box::new(10);
    let y = *x; // 解引用获取值
    println!("解引用: {} -> {}", x, y);

    // 6. 可变 Box
    let mut mb = Box::new(5);
    *mb = 10;
    println!("可变 Box: {}", mb);

    // 7. Box 在 trait 对象中的应用
    println!("\n--- Trait 对象 ---");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];

    for shape in shapes {
        println!("面积: {:.2}", shape.area());
    }

    println!("\n=== 总结 ===");
    println!("Box<T> 在堆上分配数据");
    println!("用于递归类型（链表、树等）");
    println!("Box::leak 创建静态生命周期");
    println!("trait 对象必须使用 Box<dyn Trait>");
}

// 递归结构体：链表
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn create_boxed() -> Box<i32> {
    Box::new(42)
}

// trait 用于演示 trait 对象
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
