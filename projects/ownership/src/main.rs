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

    let s1 = give_ownership(); // give_ownership 返回值 移动 给 s1
    println!("s1 = {s1}");

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3= {s3}");

    // 使用元组返回多个值
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("Tuple The length of '{s2}' is {len}.");

    // 引用
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("Reference The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);
    println!("mut val reference s = {s}");

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 = {r1}");
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;
    println!("r2 = {r2}");

    let mut s = String::from("hello");
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{r1} and {r2}");
    // 此位置之后 r1 和 r2 不再使用
    let r3 = &mut s; // 没问题
    println!("{r3}");
    // println!("{r2}"); 如果此时再使用 r2 ，那么声明 r3 编译不通过

    let first_word_index = first_word(&s);
    println!("first_word_index = {first_word_index}");

    let s = String::from("hello world");

    println!("slice first world = {}", first_word_slice(&s));

    // slice 中的索引是指字节序的索引，不是字符序的索引，注意在处理多字节字符时的不同
    let hello = &s[0..5]; // s[..5]
    let world = &s[6..11]; // s[6..]
    println!("slice, {hello}, {world}");

    let first_word = first_word_char(&s);
    println!("first_word_index = {first_word}");

    let first_word = first_word_safe(&s);
    println!("The first word safely slice = {first_word}");

    // 关于参数使用 &str 带来的一些通用接口的好处
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word_safe(&my_string[0..6]);
    let word = first_word_safe(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word_safe(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word_safe(&my_string_literal[0..6]);
    let word = first_word_safe(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word_safe(my_string_literal);
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    // 转换成字节数组会导致在处理多字节字符（如中文，UTF-8编码）时产生截断
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 如果找到空格，返回索引
            return i;
        }
    }

    println!("Not find space in string!");
    // 如果没有找到，返回字符串长度
    s.len()
}

/*
注意：
s.char_indices() 和 s.chars().enumerate() 都用来迭代，但是有区别

- s.char_indices()
char_indices() 方法返回一个迭代器，该迭代器产生一个元组 (usize, char)，其中 usize 是字符在字符串中的字节索引，char 是字符本身。
这个方法直接提供了字符的字节索引，这对于创建字符串切片非常有用，因为字符串切片是基于字节索引的。

- s.chars().enumerate()
chars() 方法返回一个迭代器，该迭代器产生字符串中的每个字符。
enumerate() 方法可以应用于任何迭代器，包括 chars() 返回的迭代器。
它返回一个迭代器，该迭代器产生一个元组 (usize, T)，其中 usize 是元素的索引（从0开始），T 是迭代器中的元素类型，在这个例子中是 char。
这里的索引是基于字符的枚举索引，而不是字节索引。
这意味着如果字符串包含多字节字符，enumerate() 方法返回的索引将不会与字符在字符串中的字节位置相对应。
*/

// 更安全将字符串切片，同时入参使用 &str 使函数更加通用，传入 string 和 &str都可
fn first_word_safe(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            // 确保索引位于字符边界上
            if s.is_char_boundary(i) {
                return &s[0..i];
            }
        }
    }

    &s[..]
}

// 入参只能是 &string
fn first_word_char(s: &String) -> usize {
    // 使用chars方法来迭代每个unicode字符而不是字节，不会产生对多字节字符的截断
    for (i, item) in s.chars().enumerate() {
        if item.is_whitespace() {
            return i;
        }
    }
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 移动 给 函数 give_ownership
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
} // some_string 离开作用域，调用drop，释放内存

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
