pub enum SemaphoreShareResult<T> {
    // Symbolizes that producer(s) know there are no items left and consumers should stop querying
    Red,
    // Symbolizes that there is an item that can be consumed right now,
    Green(T),
    // Symbolizes that there may not be an item right now but the consumer should check again later
    Yellow,
}
