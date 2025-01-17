## 结构体更新语法

- 相当于`“=”`赋值，使用旧结构体中没有实现copy trait的类型的数据，会产生移动（move），就结构体失效
- 旧结构体中的所有没有实现copy trait的类型的数据，都重新赋值，那么旧结构体在更新语法后，依旧可以使用，因为堆上数据没有产生移动
- 通过添加属性 `#[derive(Clone)]` 实现clone trait，可以使用clone方法对结构体进行赋值

## 元祖结构体

- 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
- 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的

## 类单元结构体

- 类单元结构体是一种没有任何字段的结构体。它们被称为类单元结构体，因为它们类似于 ()，即 unit 类型。
- 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。

## 打印结构体实例

1. 结构体没有实现 `std::fmt::Display trait`，所以不能直接打印结构体实例
2. 以debug方式打印结构体实例，加上外部属性 `#[derive(Debug)]` 来派生 Debug trait

如何打印结构体实例？
1. 使用 {:?} 占位符
2. 使用 {:#?} 占位符
3. 使用 dbg! 宏
4. 使用自定义方法

## 关联函数和方法

- impl 块中的函数都是关联函数，一个结构体可以有多个 impl 块
- 所有的方法都是关联函数，但不是所有的关联函数都是方法
- 关联函数是不以 self 作为参数的函数，它们通常用于返回一个结构体的新实例
- 方法是以 self 作为参数的函数，它们允许对象实例调用
