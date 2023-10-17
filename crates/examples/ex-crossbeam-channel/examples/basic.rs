use crossbeam_channel as channel;

fn main() {
    let (sender, receiver) = channel::unbounded();

    sender.send("Hello, world!").unwrap();

    let message = receiver.recv().unwrap();
    println!("{}", message); // 输出 "Hello, world!"
}
