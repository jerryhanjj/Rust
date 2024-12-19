#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法
    fn width(&self) -> u32 {
        self.width
    }

    // 方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    /*
    关联函数和方法：
        - impl 块中的函数都是关联函数
        - 所有的方法都是关联函数，但不是所有的关联函数都是方法。
        - 关联函数是不以 self 作为参数的函数，它们通常用于返回一个结构体的新实例。
        - 方法是以 self 作为参数的函数，它们允许对象实例调用。
    */
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // 使用结构体重构
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_1(&rect1)
    );

    /*
    结构体没有实现 std::fmt::Display trait，所以不能直接打印结构体实例。
    以debug方式打印结构体实例，加上外部属性 #[derive(Debug)] 来派生 Debug trait
    如何打印结构体实例？
        1. 使用 {:?} 占位符
        2. 使用 {:#?} 占位符
        3. 使用 dbg! 宏
        4. 使用自定义方法
    */
    println!("rect1 is {:?}", rect1);

    // 使用dbg!宏打印结构体实例
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 40,
    };
    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    println!("rect2's width is {}", rect2.width());

    let rect3 = Rectangle {
        width: 10,
        height: 30,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("square = {:?}", square);
}

// 计算面积
fn area_1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
