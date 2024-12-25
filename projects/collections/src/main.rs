use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3];

    // vector 可以通过放入其中的值来推断类型
    let mut v1 = Vec::new();
    v1.push(5);
    v1.push(6);

    // 通过索引获取 vector 中的值
    let third = &v[2];
    println!("The third element is {third}");

    // 通过 get 方法获取 vector 中的项
    let third = v.get(3);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // 我们在 third 处通过get方法获取了 v 的不可变引用，下方 push 时出错
    // 再一次验证 作用域内  不能同时存在可变和不可变引用
    // v.push(4);

    /*
    为什么第一个元素的引用会关心 vector 结尾的变化？
    不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，
    在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
    */

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 使用枚举来存储多种类型到vec中
    // 如果不能确切无遗地知道运行时会储存进 vector 的所有类型，枚举技术就行不通了
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 新建空字符串
    let s = String::new();

    // 从字符串字面值创建string
    let data = "initial contents";
    let s = data.to_string();
    // let s = "initial contents".to_string();
    println!("s from &str = {s}");
    // 等价于
    let s = String::from("initial contents");
    println!("s from string::from = {s}");

    // 更新字符串
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("s push_str ={s}");

    let mut s = String::from("foo");
    s.push('l');
    println!("s push = {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 from s1 + s2 = {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s from s1 + s2 + s3 = {s}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    /*
    另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对。
    collect 方法可以将数据收集进一系列的集合类型，包括 HashMap
    */
    let teams = vec![String::from("Red"), String::from("Orange")];
    let initial_scores = vec![10, 50];

    // zip 方法创建一个元组的 vector，其中 “Blue” 与 10 组成一个元组，依此类推。
    // 然后 collect 方法将这个元组的 vector 转换成一个 HashMap
    // 这里要注意使用了 into_iter 获得了集合的所有权，因为在后面 entry 插入时，我们使用了 scores 的可变引用，而 iter 方法返回的是不可变引用
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    // 因为使用了 into_iter ，vec 的所有权产生了转移
    // 再次使用 teams initial_scores 会报错
    // 如果使用 iter 则不影响
    // println!("{:?}", teams);
    // println!("{:?}", initial_scores);

    // 通过 get 方法并提供对应的键来从哈希 map 中获取值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(v) => println!("{}: {}", team_name, v),
        None => println!("{}: Not found", team_name),
    }

    // 使用 for 循环遍历哈希 map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 只在键没有对应值时插入
    // blue 没有值，所以会插入
    scores.entry(String::from("Blue")).or_insert(20);
    // red 有值，所以不会插入
    scores.entry(String::from("Red")).or_insert(15);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // entry 的 or_insert 方法在键对应的值存在时返回这个值的可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
