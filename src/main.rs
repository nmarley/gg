use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector from spawned: {:?}", v);
    });

    // can't borrow moved value
    // println!("Here's a vector from main: {:?}", v);

    handle.join().unwrap();
}
