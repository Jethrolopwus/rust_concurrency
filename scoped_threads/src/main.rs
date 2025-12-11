//=================================================
//   SCOPED THREADS
// ================================================


use std::thread;

fn main() {
   let mut vec = vec![1,2,3];
//    thread::spawn(move|| {
//     println!("{:?}", vec);
//    });
//    println!("vec: {:?}", vec);

thread::scope(|some_scope| {
    some_scope.spawn(|| {
        println!("Thread inside the scope");
        println!("The Vec: {:?}", vec);
    });
    some_scope.spawn(   || {
        println!("Another Thread inside the scope");
        // vec.push(6);
        println!("vec: {:?}", vec)
    });
});

    println!("The scope finished");

    vec.push(6);
    println!("vec: {:?}", vec);

}
