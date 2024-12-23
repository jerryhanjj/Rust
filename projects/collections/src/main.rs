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

}
