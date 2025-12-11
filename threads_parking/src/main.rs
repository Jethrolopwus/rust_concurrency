
//  ===========================================
//  Thread Parking
// ================================

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


fn main() {
    let data = Arc::new(Mutex::new(4));

    let data_clone = data.clone();


   let thread_1 = thread::spawn(move|| {
    println!("Thread_1: Iam Doing Some Work!");
    println!("Thread_1: Iam Doing Some More Work!");

    // thread::sleep(Duration::from_secs(2));

    println!("Thread 1: Parked");

    // thread::park();
    thread::park_timeout(Duration::from_secs(4));

    println!("thread_1 is printing the updated data");
    println!("thread_1 data: {:?}", *data.lock().unwrap());


   });
   let thread_2 = thread::spawn(move|| {
    println!("Thread_2 : Iam working on updating the data");
    thread::sleep(Duration::from_secs(1));
    *data_clone.lock().unwrap() = 10;
    println!("Thread_2 data updated completed!")
   });
   thread_2.join();
//    thread_1.thread().unpark();
   thread_1.join();

}
