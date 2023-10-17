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
