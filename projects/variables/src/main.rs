fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("=========================================");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("=========================================");

    // 可以明确指定类型，也可以自动推断
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}, x is {x}, z is {z}");

    // tup第一个索引是从0开始
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("tup.0 = {five_hundred}, tup.1 = {six_point_four}, tup.2 = {one}");

    println!("=========================================");

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // i32是每个元素的类型，5是元素的个数
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // 数组a包含5个元素，每个元素都被初始化为3，等同于let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    // rust中如果访问数组下标越界，会立即panic，不会允许访问
    // a[5] --- panic
}
