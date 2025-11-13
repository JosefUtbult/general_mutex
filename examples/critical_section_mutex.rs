// Handler that can take an arbitrary template mutex that implements the general mutex trait
#[cfg(feature = "critical-section-std")]
struct MyStruct<MutexType>
where
    MutexType: general_mutex::Mutex<Data = usize>,
{
    lock: MutexType,
}

#[cfg(feature = "critical-section-std")]
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

#[cfg(feature = "critical-section-std")]
fn main() {
    use general_mutex::critical_section;

    // Create a new struct with the templated mutex being a critical section mutex
    let my_struct: MyStruct<critical_section::Mutex<_>> = MyStruct::new();
    my_struct.increment_counter();
    let counter = my_struct.get_counter();
    std::println!("Counter is now {}", counter);
}

// Dummy main for incorrect use
#[cfg(not(feature = "critical-section-std"))]
fn main() {
    std::println!(
        "This example only works with the \"critical-section-std\" feature. Here's how to run it:"
    );
    std::println!("cargo run --example critical_section_mutex --features \"critical-section-std\"");
}
