# lifetime

什么时候我们会需要用到生命周期参数？
只要我们用到了引用，就需要考虑生命周期参数。
每个引用都有一个生命周期参数。

A 拥有数据的所有权，那么 A 就是出借方。
B 从 A 那里借用了数据，那么 B 就是借用方。
那么 A 的生命周期一定要比 B 的生命周期长。
也就是说 A 要活得比 B 更长。
也就是所有权出借方的生命周期要比所有权借用方的生命周期长。
为什么呢？假设 A 的生命周期比 B 的更短，那么 B 就会出现悬垂引用。

Rust 的编译器中有一个叫做生命周期检查器的工具，英文名叫做 borrow checker。
在大部分时候，它都可以自动推断出引用的生命周期参数。
不过在一些特殊的情况下，生命周期存在不确定的情况，
这个时候就需要我们自己手动标注生命周期参数，来告诉它我们希望用哪种情况。

我们手动标注的生命周期参数并不会改变引用的生命周期，
它只是用来帮助 borrow checker 检查我们的代码。

引用如果只是在一个函数体的内部使用，那么 borrow checker 是可以自己推断出
生命周期的。只有当引用用在了函数的参数、返回值的时候，borrow checker 就
做不到自动推断了，这个时候就需要手动标注生命周期参数。

```rust
'a 是出借方
'b 是借用方
'a 的生命周期 >= 'b 的生命周期
'a 是 'b 的子类型，和类继承类似
fn foo<'a: 'b>() {
}
```

返回值引用一定和某个参数引用有关系，否则的话返回值引用的就是函数体内的局部
变量，函数结束之后就会有悬垂指针，是不允许的。

## 生命周期自动推断

满足以下 3 条规则可以自动推断生命周期：

1. 函数的每个引用参数都有一个独立的生命周期标注。

```rust
fn foo<'a>(x: &'a i32);
fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
fn foo<'a, 'b, 'c>(x: &'a i32, y: &'b i32, z: &'c i32);
```

2. 如果刚好只有一个引用参数，那这个引用参数的生命周期标注直接
  应用在返回值的引用上。

```rust
fn foo<'a>(x: &'a i32) -> &'a i32
```

3. 如果方法的第一个参数是 `&self` 或 `&mut self`，那么直接把这个参数的
  生命周期标注应用在返回值的引用上。


```rust
应用 1 和 2 规则：
fn first_word(s: &str) -> &str
fn first_word<'a>(s: &'a str) -> &'a str
```

```rust
应用 1 规则：
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
可以看到 2 和 3 规则推断不出来返回值的生命周期标注，所以只好手动标注。
```

## 结构体生命周期标注

结构体成员变量用到的生命周期标注需要在 impl 和结构体名字后面加上。

```rust
impl<'a> ImportantExcerpt<'a> {}
```

## 静态生命周期标注

```rust
let s: &'static str = "I have a static lifetime.";
```

## 相关资源

https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing
https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

https://doc.rust-lang.org/nomicon/lifetimes.html
https://doc.rust-lang.org/nomicon/hrtb.html

