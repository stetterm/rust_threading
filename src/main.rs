use std::thread;
use std::thread::JoinHandle;

fn main() {
    let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..10 {
        join_handles.push(thread::spawn(move || {
            println!("hello from thread #{}", i);
        }));
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
    println!("Goodbye");
}
