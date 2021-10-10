use crate::semaphore_share_result::SemaphoreShareResult;

// FYI you're supposed to use this with a mutex
#[derive(Clone)]
pub struct SemaphoreShare<T> {
    storage: Vec<T>,
    has_reported_over: bool,
}

impl<T> SemaphoreShare<T> {
    pub fn new() -> Self {
        Self {
            storage: vec![],
            has_reported_over: false,
        }
    }

    pub fn append(&mut self, items: &mut Vec<T>) {
        self.storage.append(items)
    }

    pub fn push(&mut self, item: T) {
        self.storage.push(item);
    }

    pub fn get(&mut self) -> SemaphoreShareResult<T> {
        match self.storage.pop() {
            Some(item) => SemaphoreShareResult::Green(item),
            None => match self.has_reported_over {
                true => SemaphoreShareResult::Red,
                false => SemaphoreShareResult::Yellow,
            },
        }
    }

    pub fn set_has_reported_over(&mut self, has_reported_over: bool) {
        self.has_reported_over = has_reported_over;
    }
}
