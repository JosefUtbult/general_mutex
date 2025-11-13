// Handler that can take an arbitrary template mutex that implements the general mutex trait
#[cfg(feature = "std-mutex")]
struct MyStruct<MutexType>
where
    MutexType: general_mutex::Mutex<Data = usize>,
{
    lock: MutexType,
}

#[cfg(feature = "std-mutex")]
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

#[cfg(feature = "std-mutex")]
fn main() {
    use general_mutex::std_mutex;

    // Create a new struct with the templated mutex being an std mutex
    let my_struct: MyStruct<std_mutex::Mutex<_>> = MyStruct::new();
    my_struct.increment_counter();
    let counter = my_struct.get_counter();
    std::println!("Counter is now {}", counter);
}

// Dummy main for incorrect use
#[cfg(not(feature = "std-mutex"))]
fn main() {
    std::println!("This example only works with the \"std-mutex\" feature. Here's how to run it:");
    std::println!("cargo run --example std_mutex --features \"std-mutex\"");
}
