use crate::{Mutex as MutexTrait, setup_tests};
use core::cell::RefCell;

use std::sync::Mutex as StdMutex;
pub struct Mutex<Data>(StdMutex<RefCell<Data>>);

impl<Data> MutexTrait for Mutex<Data> {
    type Data = Data;

    fn new(data: Self::Data) -> Self {
        Self(StdMutex::new(RefCell::new(data)))
    }

    fn lock<R>(&self, f: impl FnOnce(&Data) -> R) -> R {
        #[cfg(test)]
        let lock = self.0.try_lock().unwrap();
        #[cfg(not(test))]
        let lock = self.0.lock().unwrap();

        f(&lock.borrow())
    }

    fn lock_mut<R>(&self, f: impl FnOnce(&mut Data) -> R) -> R {
        #[cfg(test)]
        let lock = self.0.try_lock().unwrap();
        #[cfg(not(test))]
        let lock = self.0.lock().unwrap();

        f(&mut lock.borrow_mut())
    }
}

setup_tests!(Mutex<_>);
