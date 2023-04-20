use std::thread::spawn;
use std::{sync::atomic::AtomicBool, cell::UnsafeCell};
use std::sync::atomic::Ordering;

const UNLOCKED: bool = false;
const LOCKED: bool = true;

struct Mutex<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self { locked: AtomicBool::new(UNLOCKED), value: UnsafeCell::new(t) }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut  T) -> R) -> R {
        while self.locked.compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // keep memory hot, avoid expensive cas operation
            while self.locked.load(Ordering::Relaxed) {}
        }
        // use yield to mock thread sche.
        std::thread::yield_now();
        // not save to manipulate value
        let ret = f(unsafe {&mut *self.value.get()});
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handes: Vec<_> =  (0..100)
        .map(|_|  {
            spawn(move || {
                for _ in 0..1000 {
                    l.with_lock(|v| {
                        *v += 1;
                    });
                }
            })
        }).collect();
    for handle in handes {
        handle.join().unwrap();
    }
    assert_eq!(l.with_lock(|v| *v), 100 * 1000);
}
