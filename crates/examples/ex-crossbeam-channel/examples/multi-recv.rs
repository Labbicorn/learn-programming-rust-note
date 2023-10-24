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
