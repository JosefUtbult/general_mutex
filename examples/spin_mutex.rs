// Handler that can take an arbitrary template mutex that implements the general mutex trait
#[cfg(feature = "spin")]
struct MyStruct<MutexType>
where
    MutexType: general_mutex::Mutex<Data = usize>,
{
    lock: MutexType,
}

#[cfg(feature = "spin")]
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

#[cfg(feature = "spin")]
fn main() {
    use general_mutex::spin;

    // Create a new struct with the templated mutex being a spin mutex
    let my_struct: MyStruct<spin::Mutex<_>> = MyStruct::new();
    my_struct.increment_counter();
    let counter = my_struct.get_counter();
    std::println!("Counter is now {}", counter);
}

// Dummy main for incorrect use
#[cfg(not(feature = "spin"))]
fn main() {
    std::println!("This example only works with the \"spin\" feature. Here's how to run it:");
    std::println!("cargo run --example spin_mutex --features \"spin\"");
}
