//=============================================
// message parsing through channesl 1 and 2


use std::{sync::mpsc, thread};

use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {

    let tx_clone = tx.clone();
    thread::spawn(move|| {
        // let mut i = "5".to_string();
        println!("Sending Value {i}");
        tx_clone.send(i).unwrap();
        // println!("the value is: {i}")
    });

    }
    drop(tx);

    // let received_val = rx.recv().unwrap();
    // println!("Received Value: {received_val}");

    //   let received_val = rx.recv().unwrap();
    // println!("Received Value: {received_val}");


    for received_messages in rx {
        println!("Received Value: {received_messages}");
    }


 let (tx, rx) = mpsc::channel();

    thread::spawn(move|| {
        let x = "Some Strings".to_string();
        println!("sending Value: {x}");
        // thread::sleep(Duration::from_secs(3));

        tx.send(x).unwrap();
    });

    // let recv_val = rx.recv().unwrap();
    // println!("Iam Blocked");

    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("The received_value is: {received_value} ");
                received_status = true;
            },
            Err(_) => println!(" I am  Doing some other stuffs "),
        }
    }
}


