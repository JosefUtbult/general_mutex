use crate::{Mutex as MutexTrait, setup_tests};

use core::cell::RefCell;
use dep_critical_section::{Mutex as CRMutex, with as critical};

pub type Mutex<Data> = CRMutex<RefCell<Data>>;

impl<Data> MutexTrait for Mutex<Data> {
    type Data = Data;

    fn new(data: Self::Data) -> Self {
        CRMutex::new(RefCell::new(data))
    }

    fn lock<R>(&self, f: impl FnOnce(&Data) -> R) -> R {
        critical(|cs| f(&*self.borrow(cs).borrow_mut()))
    }

    fn lock_mut<R>(&self, f: impl FnOnce(&mut Data) -> R) -> R {
        use dep_critical_section::with as critical;
        critical(|cs| f(&mut *self.borrow(cs).borrow_mut()))
    }
}

setup_tests!(Mutex<_>);
