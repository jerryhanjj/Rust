fn main() {
    println!("Hello, world!");

    another_function();
    another_function_1(5);
    print_labeled_measurement(5, 'h');

    /*
    语句没有返回值，表达式有值
    函数调用是一个表达式，宏调用是一个表达式，用大括号创建的一个新的块作用域也是一个表达式
    */
    let y = {
        let x = 3;
        // 表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_1(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
