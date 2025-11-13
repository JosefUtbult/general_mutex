use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

// Handler that can take an arbitrary template mutex that implements the general mutex trait
struct MyStruct<MutexType>
where
    MutexType: general_mutex::Mutex<Data = usize>,
{
    lock: MutexType,
}

impl<MutexType> MyStruct<MutexType>
where
    MutexType: general_mutex::Mutex<Data = usize>,
{
    fn new() -> Self {
        Self {
            // Create the templated mutex object using the new trait function
            lock: MutexType::new(0),
        }
    }

    // Increment the counter. This can be a non-mutable function, as it uses a general mutex for
    // the mutability of the counter
    fn increment_counter(&self) {
        self.lock.lock_mut(|counter| {
            *counter += 1;
        });
    }

    // The general mutex also allows to access the internal data as a non-mut
    fn get_counter(&self) -> usize {
        self.lock.lock(|counter| *counter)
    }
}

// Custom mutex implementation that implements the general mutex trait
struct MyVerySafeMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> general_mutex::Mutex for MyVerySafeMutex<T> {
    type Data = T;

    fn new(data: Self::Data) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    fn lock<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R {
        // Make a (very bad) check for if the mutex already is acquired
        if self.locked.load(Ordering::SeqCst) {
            panic!("Tried to unlock mutex multiple times");
        }

        f(unsafe { &*self.data.get() })
    }

    fn lock_mut<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        // Make a (very bad) check for if the mutex already is acquired
        if self.locked.load(Ordering::SeqCst) {
            panic!("Tried to unlock mutex multiple times");
        }

        f(unsafe { &mut *self.data.get() })
    }
}

fn main() {
    // Create a new struct with the templated mutex being our custom mutex
    let my_struct: MyStruct<MyVerySafeMutex<_>> = MyStruct::new();
    my_struct.increment_counter();
    let counter = my_struct.get_counter();
    std::println!("Counter is now {}", counter);
}
