struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        Self {
            name,
            payload,
        }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

// 只有当泛型是String时才有这个方法
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn serialize_payload<T>(payload: T) -> String {
    "placeholder".to_owned()
}

fn main() {
    // let cmd1 = BrowserCommand {
    //     name: "navigate".to_owned(),
    //     payload: "http://www.google.com",
    // };

    // let cmd2 = BrowserCommand {
    //     name: "zoom".to_owned(),
    //     payload: 200,
    // };

    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(), 
        "http://www.google.com".to_owned(),
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200,
    );

    // cmd2没有print_payload方法
    cmd1.print_payload();

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    let ph1 = serialize_payload(p1);
    println!("placeholder: {}", ph1);
    let ph2 = serialize_payload(p2);
    println!("placeholder: {}", ph2);
}
