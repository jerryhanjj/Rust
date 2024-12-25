## 集合
Rust 标准库中提供了一系列集合：
- Sequences: Vec, VecDeque, LinkedList
- Maps: HashMap, BTreeMap
- Sets: HashSet, BTreeSet
- Misc: BinaryHeap

## Vector

- `vector` 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。
- `vector` 只能储存相同类型的值
- 通过 **索引** 或者 **get** 方法获取 `vector` 中的项，如果索引越界，前者会导致程序 `panic` ，后者会返回一个 `None`
- 可以使用枚举来将不同类型的数据存储到 `vector` 中

## String
```
什么是字符串？
在开始深入这些方面之前，我们需要讨论一下术语 字符串 的具体意义。
Rust 的核心语言中只有一种字符串类型：字符串 slice str，它通常以被借用的形式出现，&str。第四章讲到了 字符串 slices：它们是一些对储存在别处的 UTF-8 编码字符串数据的引用。
举例来说，由于字符串字面值被储存在程序的二进制输出中，因此字符串字面值也是字符串 slices。

字符串（String）类型由 Rust 标准库提供，而不是编入核心语言，它是一种可增长、可变、可拥有、UTF-8 编码的字符串类型。
当 Rustaceans 提及 Rust 中的 "字符串 "时，他们可能指的是 String 或 string slice &str 类型，而不仅仅是其中一种类型。
虽然本节主要讨论 String，但这两种类型在 Rust 的标准库中都有大量使用，而且 String 和 字符串 slices 都是 UTF-8 编码的。
```

更新字符串：
- 使用 `push_str` 和 `push` 附加字符串
- 使用 `+` 运算符或 `format!` 宏拼接字符串

## Hashmap

### 创建hashmap
1. 通常方法，类似 vector
2. 使用迭代器，元组，zip，collection方法，注意 into_iter 和 iter 两个方法的差异

### 获取键值
1. 通过 get 方法获取键值，如果没有对应的键值，返回 None
2. 循环遍历 map


### 更新hashmap
1. 直接覆盖插入
2. 只在键没有对应值时插入键值对
3. 更新键值