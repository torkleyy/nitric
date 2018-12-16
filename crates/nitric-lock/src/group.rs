use crate::mutex::new_mutex;
use crate::Mutex;

#[derive(Default)]
pub struct LockGroup {
    counter: usize,
    unique_id: Box<u8>,
}

impl LockGroup {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn id(&self) -> usize {
        self.unique_id.as_ref() as *const _ as usize
    }

    pub fn mutex<T>(&mut self, value: T) -> Mutex<T> {
        let r = new_mutex(value, self.counter);

        self.counter = self
            .counter
            .checked_add(1)
            .expect("Allocated more than `usize::MAX` locks");

        r
    }

    pub fn token(&mut self) -> LockToken {
        LockToken {
            _opaque: (),
        }
    }
}

pub struct LockToken {
    _opaque: (),
}
