#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {
    println!("ip_type: {:?}", ip_type);
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

#[derive(Debug)]
#[allow(dead_code)]
// 使用枚举可以每个成员可以处理不同类型和数量的数据
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
这个枚举有四个含有不同类型的成员：
- Quit 没有关联任何数据
- Move 包含一个匿名结构体
- Write 包含单独一个 String
- ChangeColor 包含三个 i32

如下这些结构体可以包含与之前枚举成员中相同的数据：
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to r: {}, g: {}, b: {}", r, g, b)
            }
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_value: u8) {}
fn reroll() {}

// 通配模式
fn wildcard(value: u8) {
    match value {
        3 => add_fancy_hat(),
        4 => remove_fancy_hat(),
        other => move_player(other),
    }
}

// _ 占位符模式
fn placeholder(value: u8) {
    match value {
        1 => add_fancy_hat(),
        2 => remove_fancy_hat(),
        _ => reroll(),
    }
}

// 如果只对某种匹配模式感兴趣，可以使用 if let 来简化代码
fn only_interest(value: Option<i32>) {
    if let Some(5) = value {
        println!("value = 5");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}, six: {:?}", four, six);

    route(four);
    route(six);

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);

    let m_write = Message::Write(String::from("hello"));
    let m_quit = Message::Quit;
    let m_move = Message::Move { x: 1, y: 2 };
    let m_change_color = Message::ChangeColor(255, 0, 0);
    m_write.call();
    m_quit.call();
    m_move.call();
    m_change_color.call();

    let coin = Coin::Penny;
    println!("Coin: {}", value_in_cents(coin));

    println!("Coin: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);

    let value = 5;
    wildcard(value);
    placeholder(value);

    only_interest(Some(5));
}
