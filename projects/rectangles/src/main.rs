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
}

// 计算面积
fn area_1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
