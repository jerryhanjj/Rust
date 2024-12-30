use std::f32::consts::E;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let greeting_file_result = File::open("hello");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // 通过使用 unwarp 调用 panic
    let greeting_file = File::open("hello.txt").unwrap();

    // 使用 expect 定制错误信息
    let greeting_file = File::open("hello.txt").expect("error msg");

    // 传播错误
    // 选择让调用者知道这个错误并决定该如何处理
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // 使用 ? 运算符
    fn read_username_from_file_1() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    /*
    match 表达式与 ? 运算符所做的有一点不同：
    ? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
    当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。
    这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败

    File::open 调用结尾的 ? 会将 Ok 中的值返回给变量 username_file。
    如果发生了错误，? 运算符会使整个函数提前返回并将任何 Err 值返回给调用代码。
    同理也适用于 read_to_string 调用结尾的 ?
    */

    // ? 运算符链式使用
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // 再继续简化
    fn read_username_from_file_3() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}
