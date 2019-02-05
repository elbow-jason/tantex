use std::sync::RwLock;

pub struct Wrapper<T> {
    pub lock: RwLock<T>,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Wrapper<T> {
        Wrapper {
            lock: RwLock::new(value),
        }
    }
}
