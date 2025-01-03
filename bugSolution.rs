use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = x.clone();
    let z = x.clone();

    let handle1 = std::thread::spawn(move || {
        let mut x = x.lock().unwrap();
        *x = 6;
    });

    let handle2 = std::thread::spawn(move || {
        let mut x = y.lock().unwrap();
        *x = 7;
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{}", *x.lock().unwrap());
}
