use std::thread;
use std::thread::JoinHandle;

#[macro_export]
macro_rules! avec {
    () => { Vec::new() };
    ($($elem : expr),+) => {{
        let mut new = Vec::new();
        $(new.push($elem);)*
        new
    }};
    ($elem : expr ; $n : expr) => {{
        let mut new = Vec::new();
        for _ in 0..$n {
            new.push($elem);
        }
        new
    }};
}

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
    let x: Vec<i32> = avec![5];
    dbg!(&x);
    let x: Vec<i32> = avec![5; 5];
    dbg!(&x);
    let x = avec![1, 2, 3, 4, 5, 6];
    dbg!(&x);
}
