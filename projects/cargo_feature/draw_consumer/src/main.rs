// 如果设置了 default-features = false ，需要显式启用color才能使用use语句导入
use draw::color;

fn main() {
    println!("Hello, world!");
    draw::draw_line(32, 32);

    let color = color::RGB {
        r: 247,
        g: 76,
        b: 0,
    };

    color::draw_line(32, 32, &color);

    let square = draw::shapes::Rectangle {
        width: 32,
        height: 32,
        color,
    };

    println!("{square:?}");
}
