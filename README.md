# general_mutex

The `general_mutex` rust crate is a generalized mutex trait that can be used for templating. This allows a struct to have templated mutex types for its internal variables:

```rust
struct MyStruct<MutexType>
where
    MutexType: general_mutex::Mutex<Data = usize>,
{
    // Internal variable containing a templated mutex with a usize in it
    lock: MutexType,
}
```

A struct can then be created with any of the provided mutex types

```rust
// Using the type provided by the crate...
let my_struct1: MyStruct<spin::Mutex<_>> = MyStruct::new();

// ... Or just the mutex as-is
let my_struct2: MyStruct<spin::Mutex<core::cell::RefCell<_>>> = MyStruct::new();
```
