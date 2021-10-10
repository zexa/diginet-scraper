pub struct ScraperSettings {
    // TODO: Is there any way to make this assert that the value is at least 1 at compile time?
    threads: u64, // You never know how many cores will processors have in the future.
}

impl ScraperSettings {
    pub fn new(threads: u64) -> Self {
        Self { threads }
    }

    pub fn get_threads(&self) -> u64 {
        self.threads // This should copy
    }
}
