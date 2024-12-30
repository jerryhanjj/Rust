use std::{fs::File, io::ErrorKind};

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

    
}
