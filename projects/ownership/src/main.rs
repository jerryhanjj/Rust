fn main() {
    let s = "hello, world";
    println!("{s}");

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    let x = 5;
    let y = x;
    // x 和 y 都在栈中，值为5，有两个5
    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1;
    // s2=s1 的过程相当于一次”浅拷贝”，只拷贝了指向内存的指针，没有拷贝内存的行为
    // “=” 操作之后，s1 就自动失效，只有 s2 指向 s1 曾经的内存，防止产生在离开作用域时“二次释放”的情况
    // 正因此，Rust 中这种操作被称为 移动（move）
    // println!("{s1}, world"); -- s1 已失效，无法通过编译
    println!("after move: {s2}, world");

    // x,y 都在栈上，这种类型的数据是拷贝而不是移动
    // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用

    let s = String::from("hello");
    take_ownership(s); // s 移动 到 take_ownership，不再有效

    let x = 5;
    makes_copy(x); // x 拷贝 到 makes_copy，没有失效
    println!("still use x = {x}");
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
} // some_string 离开作用域，调用drop，释放内存

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
