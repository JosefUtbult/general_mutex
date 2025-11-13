use crate::{Mutex as MutexTrait, setup_tests};
use core::cell::RefCell;
use dep_spin::Mutex as SpinMutex;

pub struct Mutex<Data>(SpinMutex<core::cell::RefCell<Data>>);

impl<Data> MutexTrait for Mutex<Data> {
    type Data = Data;

    fn new(data: Self::Data) -> Self {
        Self(SpinMutex::new(RefCell::new(data)))
    }

    fn lock<R>(&self, f: impl FnOnce(&Data) -> R) -> R {
        #[cfg(test)]
        let lock = self.0.try_lock().unwrap();
        #[cfg(not(test))]
        let lock = self.0.lock();

        f(&lock.borrow())
    }

    fn lock_mut<R>(&self, f: impl FnOnce(&mut Data) -> R) -> R {
        #[cfg(test)]
        let lock = self.0.try_lock().unwrap();
        #[cfg(not(test))]
        let lock = self.0.lock();

        f(&mut lock.borrow_mut())
    }
}

setup_tests!(Mutex<_>);
