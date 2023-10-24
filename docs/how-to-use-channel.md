# 如何理解以及使用Rust中的Channel

## 🤔 什么是Channel

Channel是Rust中的一个特殊的数据类型，它可以用来在多个线程之间传递消息。Channel由两部分组成，一个是发送端(Sender)，一个是接收端(Receiver)。发送端用来发送消息，接收端用来接收消息。Channel的发送端和接收端都可以有多个，这样就可以实现多个线程之间的通信。

## 🤔 Channel的类型

标准库`std::sync::mpsc::channel`中提供的Channel是多生产者单消费者的，也就是说，一个Channel可以有多个发送端，但是只能有一个接收端。`crossbeam-channel` crate提供的Channel是多生产者多消费者的，也就是说，一个Channel可以有多个发送端，也可以有多个接收端。

## Channel 的另一层理解

其实对于Channel的使用来说，还可以有另一种的理解方式，对于多个任务的处理可以拆分给多个线程，但是多个线程在处理完之后需要将处理后的输出传递给另一个线程，这里就需要用到了Channel，这里也有点类似Unix中的管道的思想。

下面是一个示例的代码:

```rust
fn run_pipelin(documents: Vec<Pathbuf>, output_dir: PathBuf) -> io::Result<()> {
    // start channel 5 stage
    let (texts, h1) = start_file_reader_thread(documents);
    let (pints, h2) = start_file_indexing_thread(texts);
    let (gallons, h3) = start_in_memory_merge_thread(pints);
    let (files, h4) = start_index_writer_thread(gallons, &output_dir);
    let result = merge_index_file(file, &output_dir);

    // wait this threads stop,
    let r1 = h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    let r4 = h4.join().unwrap();

    r1?;
    r4?;
    result
}
```

## 🤔 Channel的使用

### 创建Channel

创建Channel需要使用`std::sync::mpsc`中的`channel`函数，该函数返回一个元组，元组的第一个元素是发送端，第二个元素是接收端。

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

#### 使用crossbeam channel 创建Channel

也可以使用`crossbeam-channel` crate来创建Channel，该crate提供了更多的功能，比如多个接收端，多个发送端等。

```rust
use crossbeam_channel as channel;

fn main() {
    let (sender, receiver) = channel::unbounded();
}
```

### 发送消息

发送消息需要使用`Sender`的`send`方法，该方法接收一个泛型参数，该泛型参数就是发送的消息。

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send("hello").unwrap();
}
```

#### 使用crossbeam channel 发送消息

```rust
use crossbeam_channel as channel;

fn main() {
    let (sender, receiver) = channel::unbounded();

    sender.send("Hello, world!").unwrap();
}
```

### 接收消息

接收消息需要使用`Receiver`的`recv`方法，该方法返回一个`Result`枚举，`Ok`表示接收到消息，`Err`表示没有接收到消息。

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send("hello").unwrap();
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}
```

#### 使用Crossbeam channel接受消息

```rust
use crossbeam_channel as channel;

fn main() {
    let (sender, receiver) = channel::unbounded();

    sender.send("Hello, world!").unwrap();

    let message = receiver.recv().unwrap();
    println!("{}", message); // 输出 "Hello, world!"
}
```

### 多个发送端

多个发送端可以通过`clone`方法来创建，`clone`方法会创建一个新的发送端，新的发送端和原来的发送端共享接收端。

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    tx.send("hello").unwrap();
    tx1.send("world").unwrap();
    let msg = rx.recv().unwrap();
    println!("{}", msg);
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}
```

### 多个接收端

对于要是使用多个接收端的话，标准库中没有提供，只能单个接收端，要是想使用多个接收端需要使用`crossbeam-channel` crate，实现此功能。

```rust
use crossbeam_channel::unbounded;

fn main() {
    let (tx, rx) = unbounded();
    let tx1 = tx.clone();
    tx.send("hello").unwrap();
    tx1.send("world").unwrap();
    let msg = rx.recv().unwrap();
    println!("{}", msg);
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}
```

### 受限的Channel

Channel可以设置一个缓冲区，缓冲区的大小就是Channel可以存储的消息的数量，当缓冲区满了之后，发送端就会阻塞，直到缓冲区有空间了才会继续发送消息。当缓冲区为空的时候，接收端就会阻塞，直到缓冲区有消息了才会继续接收消息。

```rust
use crossbeam_channel as channel;

fn main() {
    let (sender, receiver) = channel::bounded(1); // 设置通道容量为 1

    sender.send("Message 1").unwrap();
    // sender.send("Message 2").unwrap(); // 这行会阻塞，因为通道已满

    let message = receiver.recv().unwrap();
    println!("{}", message); // 输出 "Message 1"
}
```

### 多个线程配合channel的使用

```rust
use crossbeam_channel as channel;
use std::thread;

fn main() {
    let (sender, receiver) = channel::unbounded();

    let handle = thread::spawn(move || {
        sender.send("Hello from another thread!").unwrap();
    });

    let message = receiver.recv().unwrap();
    println!("{}", message); // 输出 "Hello from another thread!"

    handle.join().unwrap();
}
```

1. 首先创建一个Channel，发送端和接收端都是`unbounded`的，也就是说，发送端和接收端都是无限的，可以发送无限多的消息。
2. 创建一个线程，线程中发送消息。
3. 主线程中接收消息。


### Crossbeam channel的Select的使用介绍

```rust
use crossbeam_channel as channel;
use crossbeam_channel::select;

fn main() {
    let (sender1, receiver1) = channel::unbounded();
    let (sender2, receiver2) = channel::unbounded();

    let mut counter = 0;
    sender1
        .send(format!("Message from channel1 {counter}"))
        .unwrap();
    sender2
        .send(format!("Message from channel2 {counter}"))
        .unwrap();

    loop {
        select! {
            recv(receiver1) -> message => {
                println!("Received: {:?}", message);
                counter += 1;
                sender2
                    .send(format!("Message from channel2 {counter}"))
                    .unwrap();
            },
            recv(receiver2) -> message => {
                println!("Received: {:?}", message);
                counter -= 1;
                sender1
                    .send(format!("Message from channel1 {counter}"))
                    .unwrap();
            }
        }
        println!("The Counter is {counter}");
    }
}
```

1. 首先创建两个Channel，一个是`sender1`和`receiver1`，另一个是`sender2`和`receiver2`。
2. 创建一个计数器`counter`，并且向`sender1`和`sender2`发送消息。
3. 创建一个循环，循环中使用`select`宏，该宏接收一个`match`表达式，`match`表达式中使用`recv`方法来接收消息，`recv`方法返回一个`Result`枚举，`Ok`表示接收到消息，`Err`表示没有接收到消息。
4. `select`宏会阻塞，直到有一个`recv`方法接收到消息，就会执行对应的`match`分支。
