use core::sync::atomic::{AtomicBool, Ordering::SeqCst};

use crate::Mutex;

/// Generates standard tests for a mutex type
#[macro_export]
macro_rules! setup_tests {
    ($mutex_type:ty) => {
        #[test]
        fn create() {
            $crate::tests::test_create::<$mutex_type>();
        }

        #[test]
        fn lock() {
            $crate::tests::test_lock::<$mutex_type>();
        }

        #[test]
        fn lock_mut() {
            $crate::tests::test_lock_mut::<$mutex_type>();
        }
    };
}

/// Generates recursive failure tests for a mutex type
#[macro_export]
macro_rules! setup_reqursive_tests {
    ($mutex_type:ty) => {
        #[test]
        #[should_panic]
        fn recursive_lock() {
            $crate::tests::test_recursive_lock::<$mutex_type>();
        }

        #[test]
        #[should_panic]
        fn recursive_lock_mut() {
            $crate::tests::test_recursive_lock_mut::<$mutex_type>();
        }
    };
}

pub fn test_create<MutexType>()
where
    MutexType: Mutex<Data = u8>,
{
    let _ = MutexType::new(0);
}

pub fn test_lock<MutexType>()
where
    MutexType: Mutex<Data = &'static AtomicBool>,
{
    static FLAG: AtomicBool = AtomicBool::new(false);
    let mutex = MutexType::new(&FLAG);

    let res = mutex.lock(|flag| {
        flag.store(true, SeqCst);
        255
    });

    assert!(FLAG.load(SeqCst));
    assert_eq!(res, 255);
}

pub fn test_lock_mut<MutexType>()
where
    MutexType: Mutex<Data = &'static AtomicBool>,
{
    static FLAG: AtomicBool = AtomicBool::new(false);
    let mutex = MutexType::new(&FLAG);

    let res = mutex.lock_mut(|flag| {
        flag.store(true, SeqCst);
        255
    });

    assert!(FLAG.load(SeqCst));
    assert_eq!(res, 255);
}

pub fn test_recursive_lock<MutexType>()
where
    MutexType: Mutex<Data = u8>,
{
    let mutex = MutexType::new(0);

    mutex.lock(|_| {
        mutex.lock(|_| {});
    });
}

pub fn test_recursive_lock_mut<MutexType>()
where
    MutexType: Mutex<Data = u8>,
{
    let mutex = MutexType::new(0);

    mutex.lock_mut(|_| {
        mutex.lock_mut(|_| {});
    });
}
