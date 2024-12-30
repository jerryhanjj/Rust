use std::f32::consts::E;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // let greeting_file_result = File::open("hello");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     // Err(error) => panic!("Problem opening the file: {error:?}"),
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     },
    // };

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
}
