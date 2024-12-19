## 枚举值

- 利用枚举可以将 **数据类型** 和 **值** 关联起来
- 每一个我们定义的枚举成员的名字变成了一个构建枚举的实例的函数，例如：`IpAddr::V4()` 是一个获取 `String` 参数并返回 `IpAddr` 类型实例的函数调用
- 用枚举替代结构体可以用枚举的每个成员可以处理不同类型和数量的数据
    ```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    ```
- 与结构体相似，可以在 `impl` 块中为枚举定义方法