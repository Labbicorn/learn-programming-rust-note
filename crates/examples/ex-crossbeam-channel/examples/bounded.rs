use crossbeam_channel as channel;

fn main() {
    let (sender, receiver) = channel::bounded(1); // 设置通道容量为 1

    sender.send("Message 1").unwrap();
    // sender.send("Message 2").unwrap(); // 这行会阻塞，因为通道已满

    let message = receiver.recv().unwrap();
    println!("{}", message); // 输出 "Message 1"
}
