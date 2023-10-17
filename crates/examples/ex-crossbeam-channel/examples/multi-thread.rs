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
