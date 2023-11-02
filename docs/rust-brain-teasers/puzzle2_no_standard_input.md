# No-Standard Input （非标准输入）

```rust
use std::io::stdin;

fn main() {
    println!("What is 3 + 2 ? Type you answer and press enter.");
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Unable to read standard input");

    // println!("input = {:?}", input);
    if input == "5" {
        println!("Correct");
    } else {
        pritnln!("Incorrect");
    }
}
```

这个运行的结果会打印"Incorrect"不知道大家有没有猜出来？

## 为什么呢？

因为我们输入的时候，会在输入的内容后面加上一个换行符，所以我们输入的内容是`5\n`，而不是`5`，所以我们的判断是错误的。


我们可以打印input的内容查看，显示的内容是`5\n`.

Rust的标准输入系统包括代表回车， \r表示回车，\n表示换行。我们可以使用trime函数来清理非打印字符。

下面是修改后的代码

```rust
use std::io::stdin;

fn main() {
    println!("What is 3 + 2 ? Type you answer and press enter.");
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Unable to read standard input");

    // println!("input = {:?}", input);
    if input.trime() == "5" {
        println!("Correct");
    } else {
        pritnln!("Incorrect");
    }
}
```

## 不要相信输入

一个好的经验法则是永远不要相信输入；但是，当输入是必要的时候，可以做一些事情来尽量减少错误。

当处理字符串时，使用trim来删除空格。

当比较字符串时，使用to_lowercase或者to_uppercase来将字符串转换为相同的大小写。

当解析复杂的字符串时，使用正则表达式来提取字符串的各个部分。

## 其他

- String - trim函数
- regex crate
