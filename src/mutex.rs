pub trait Mutex {
    type Data;

    fn new(data: Self::Data) -> Self;
    fn lock<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R;
    fn lock_mut<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
}
