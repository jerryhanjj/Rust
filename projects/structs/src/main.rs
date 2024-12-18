#[derive(Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn print_user(user: &User) {
    println!();
    println!("user name = {}", user.username);
    println!("user email = {}", user.email);
    println!("user active = {}", user.active);
    println!("user sign_in_count = {}", user.sign_in_count);
    println!();
}

fn main() {
    let mut user1 = User {
        username: String::from("name"),
        email: String::from("xxx@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("username = {}", user1.username);
    user1.email = String::from("han.junjie@gmail.com");
    println!("user's email = {}", user1.email);

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    print_user(&user1);

    // 使用旧结构体创建新结构体
    // 方法1（普通）
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("xxxemial"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // print_user(&user2);
    // 方法2（结构体更新语法）
    let user2 = User {
        email: String::from("han.junjie@126.com"),
        // 如果 user1 中所有没有实现copy trait的字段都不赋值，那么 user1 可以继续使用
        // 此处 username 字段没有从 user1 中 转移（move），所以 user1 可以继续使用
        username: String::from("xxx"),
        ..user1 // .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值
    };
    print_user(&user2);
    // 如果 username 重新赋值，那么 user1 就可以再使用
    print_user(&user1);

    // 通过添加属性 #[derive(Clone)] 实现clone trait
    let user3 = user1.clone();
    print_user(&user3);

    /*
    元祖结构体：
    - 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
    - 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的
    */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(255, 228, 196);
    let origin = Point(0, 0, 0);

    println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);

    let Color(r, g, b) = black;
    println!("r: {}, g: {}, b: {}", r, g, b);
    let Point(x, y, z) = origin;
    println!("x: {}, y: {}, z: {}", x, y, z);

    /*
    类单元结构体：
    - 类单元结构体是一种没有任何字段的结构体。它们被称为类单元结构体，因为它们类似于 ()，即 unit 类型。
    - 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
    */
    struct EmptyStruct;
    #[allow(unused_variables)]
    let empty = EmptyStruct;
}

fn build_user(email: String, username: String) -> User {
    // 在函数内构造一个 User
    User {
        active: true,
        sign_in_count: 1,
        // 如果参数名和字段名相同，可以简写
        // username: username,
        // email: email,
        username,
        email,
    }
}
