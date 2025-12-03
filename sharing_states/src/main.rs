// SHARING STATES IN RUST

// A data  wrap in a mutex can only be access by a single thread at any given time


use std::fmt::format;
use std::thread;
use std::sync::Mutex;
use std::rc::Rc;
use std::sync::Arc;


struct File {
    text: Vec<String>,

}
fn main() {
    let m = Mutex::new(5);
    {
        let mut  num = m.lock().unwrap();
        *num = 10;
    }
    let lock_m =m.lock().unwrap();
    println!("M is {:?}", *lock_m);

    drop(lock_m);

    let lock_m1 = m.lock().unwrap();
    println!("This code is Blocked");


let file = Arc::new( Mutex::new(File{text: vec![]}));

let mut thread_vec = vec![];

for i in 0..10  {
    let file = Arc::clone(&file);
    let handle = thread::spawn( move|| {

        let mut file_lock = file.lock().unwrap();
        file_lock.text.push(format!("Hello from  thread {i}"));
    });

    thread_vec.push(handle);
    
};

for handle in thread_vec{
    handle.join().unwrap();
}
let file_lock = file.lock().unwrap();
for t in &file_lock.text{
    println!("{t}");
}

}   
