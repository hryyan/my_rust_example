#[no_mangle]
use std::thread;

pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        std::thread::spawn(|| {
            let mut _x = 0;
            for _ in (0..5_000_001) {
                _x += 1
            }
        })
    }).collect();

    for h in handles {
        h.join().ok().expect("Could not join a thread!");
    }
}
