
// ----------------------------------
//     THREAD
// ---------------------------------


// concurrency is about multiple task which start, run and complete in overlaping time period in no specific order
// parrallesm is about multiple task that literally run at the same time on hardware with multiple computing resources like multi core processors


use std::thread;
use std::time::Duration;





fn main() {
    println!("This will be printed");
    println!("This will also be printed");

    println!("The Concurrency will start after this line!");


//calling Join on the thread handle
    let t = thread::spawn(|| {
        println!("Hello 1 from the thread!");
        println!("Hello 2 from the thread!");
        println!("Hello 3 from the thread!");
        println!("Hello 4 from the thread!");
        println!("Hello 5 from the thread!");
        println!("Hello 6 from the thread!");
        println!("Hello 7 from the thread!");

    });
   // thread::sleep(Duration::from_millis(1));  //this allow the calling thread for some time
   
   println!("Hello 1 from the Main!");
   println!("Hello 2 from the Main!");
   t.join();
   
    

    // ================================
    // Ownership and thread
    // ================================

    let x = "Some strings".to_string();

    thread::spawn(move|| {
        let y = x;
        println!("{y}")
    });
    // println!("{x}")
}
