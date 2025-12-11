// // =============================================
// //    BARRIERS 
// // =============================================

use std::sync::{Arc, Barrier, Mutex};
use std::thread;

fn main() {
    let mut _threads_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    let tasks = Arc::new(Mutex::new(vec![]));

    let barrier = Arc::new(Barrier::new(5));

    for i in 0..5 {
        let tasks = tasks.clone();
        let barrier = barrier.clone();

        let handle = thread::spawn(move || {
            // task 1
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, completed task 1"));

           
            barrier.wait();

            // task 2
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, completed task 2"));
        });

        _threads_vec.push(handle);
    }

    for handle in _threads_vec {
        handle.join().unwrap();
    }

    let task_lock = tasks.lock().unwrap();
    for contents in task_lock.iter() {
        println!("{contents}");
    }
}
