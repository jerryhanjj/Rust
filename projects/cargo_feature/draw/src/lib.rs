pub fn draw_line(x:i32, y:i32) {
    println!("draw's draw_line");
}

#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;

    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("{color}")
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use serde::{Deserialize, Serialize};
    use rgb::RGB;

    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
        pub color: RGB<u16>,
    }
}