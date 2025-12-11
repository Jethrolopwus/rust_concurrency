// =====================================================


// ASYNC AWAIT
// TOKIO
// =====================================================

use tokio::runtime::Handle;
use std::time::Duration;
use tokio::time::sleep;

 async fn  printing (i: i32) {
    println!("Task {i}");
 }

 #[tokio::main(flavor = "current_thread")]
 async  fn main() {
    let mut  handles  = vec![];
    for i  in 0..3 {
        let Handle = tokio::spawn(async move {
            println!("Task {i}, printing, first time");
            printing(i).await;
            println!("Task {i}, printing,  second time");
            printing(i).await;

        });
        handles.push(Handle);
    }
     for Handle in handles {
    Handle.await.unwrap();
     }
//    let x = printing();

//    println!("the future have not been pollled yet");
//       x.await;
//    drop(x);
   
}
