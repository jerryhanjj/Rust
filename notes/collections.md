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

