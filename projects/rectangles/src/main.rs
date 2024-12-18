#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
}

// 计算面积
fn area_1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
