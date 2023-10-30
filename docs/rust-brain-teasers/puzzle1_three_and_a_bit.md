# Three and a Bit

```rust
const THREE_AND_A_BIT: f32 = 3.4028236;

fn main() {
    println!("{}", THREE_AND_A_BIT);
}
```

想当然，我们猜测他的输出是3.4028236, 但是实际上是3.40282367，结果相差0.0000001。

同其他语言一样的是Rust中的32位浮点数，也是使用IEEE-754标准表示的。

一个浮点数的内存布局如下：

```
|------------- 32 bits ---------|
|1 bit |    8 bit  |   23 bits  |
| ---- | ----------| -----------|
| sign |  exponent |  mantissa  |
```


f32 = sign(-1 or 1) x 2 ^{epxponent - 127} x 1.mantissa

Rust 计算出表示3.4028236最有效的方法是使用指数2和尾数1.7014118432998657。

实际上，7014118在32位二进制中不能完美表示。需要注意的是，数字1.被假定存在IEEE-754标准中，但实际上并没有存储。最接近的表示法是7014118432998657.

3.4028237 = 1 x 2 ^ {128 - 127} x 1.7014118432998657
3.4028237 = 3.4028236865997314

有的时候，解决浮点数精度错误，通过使用一个更大的浮点数类型。例如： f64。
如果64位不够，f128 crate 提供128位浮点数。 但是这个不能解决根本问题。
有一些类型根本无法表示。例如，pi，总是使用近似值来表示。

如果真的需要一个完美的表示，可以使用rug，可以提供任意精度。

在性能和准确性之间存在一个权衡。因此，需要花点时间考虑你试图用程序解决的问题。并根据需要和程序运行速度之间的平衡来选择数值精度。浮点数是由CPU直接支持的。而且运算速度非常快。即使使用浮点数f32也可以比f64更快。因为32位版本使用的内存更少。定点和任意精度库可以很快，但比使用内置浮点数支持要慢。需要由你决定你的程序的精度、性能权衡需求。

## other

- IEE-754 floating point stanadard
- Rug - arbitrary precision numbers crate
- f128 crate
- fixed crate