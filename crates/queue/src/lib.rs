use std::collections::VecDeque;

pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.items.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_is_true_for_new_queue() {
        let q = Queue::<i32>::new();
        assert!(q.is_empty());
    }

    #[test]
    fn len_is_zero_for_new_queue() {
        let q = Queue::<i32>::new();
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn peek_front_returns_none_for_new_queue() {
        let q = Queue::<i32>::new();
        assert_eq!(q.peek_front(), None);
    }

    #[test]
    fn dequeue_returns_none_for_new_queue() {
        let mut q = Queue::<i32>::new();
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn enqueue_makes_is_empty_false() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        assert!(!q.is_empty());
    }

    #[test]
    fn enqueue_increases_len() {
        let mut q = Queue::<i32>::new();
        q.enqueue(10);
        assert_eq!(q.len(), 1);
        q.enqueue(20);
        assert_eq!(q.len(), 2);
        q.enqueue(30);
        assert_eq!(q.len(), 3);
    }

    #[test]
    fn peek_front_returns_first_enqueued_element() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.peek_front(), Some(&1));
    }

    #[test]
    fn peek_front_does_not_remove_element() {
        let mut q = Queue::<i32>::new();
        q.enqueue(42);
        assert_eq!(q.peek_front(), Some(&42));
        assert_eq!(q.peek_front(), Some(&42));
        assert_eq!(q.len(), 1);
        assert!(!q.is_empty());
    }

    #[test]
    fn dequeue_returns_first_enqueued_element_fifo() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn dequeue_decreases_len() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.len(), 3);

        q.dequeue();
        assert_eq!(q.len(), 2);
        q.dequeue();
        assert_eq!(q.len(), 1);
        q.dequeue();
        assert_eq!(q.len(), 0);
        assert!(q.is_empty());
    }

    #[test]
    fn dequeue_until_empty_then_peek_is_none() {
        let mut q = Queue::<i32>::new();
        q.enqueue(7);
        q.enqueue(8);

        assert_eq!(q.dequeue(), Some(7));
        assert_eq!(q.dequeue(), Some(8));

        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek_front(), None);
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn alternating_enqueue_and_dequeue_maintains_invariants() {
        let mut q = Queue::<i32>::new();

        q.enqueue(1);
        assert_eq!(q.peek_front(), Some(&1));
        assert_eq!(q.len(), 1);

        assert_eq!(q.dequeue(), Some(1));
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);

        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.peek_front(), Some(&2));
        assert_eq!(q.len(), 2);

        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.peek_front(), Some(&3));
        assert_eq!(q.len(), 1);

        assert_eq!(q.dequeue(), Some(3));
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek_front(), None);
    }

    #[test]
    fn clear_empties_queue_and_resets_state() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        q.clear();

        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek_front(), None);
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn clear_after_operations_resets_state() {
        let mut q = Queue::<i32>::new();

        q.enqueue(1);
        q.enqueue(2);
        assert_eq!(q.dequeue(), Some(1));
        q.enqueue(3);
        assert_eq!(q.peek_front(), Some(&2));
        assert_eq!(q.len(), 2);

        q.clear();

        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        assert_eq!(q.peek_front(), None);

        q.enqueue(99);
        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        assert_eq!(q.peek_front(), Some(&99));
        assert_eq!(q.dequeue(), Some(99));
        assert!(q.is_empty());
    }

    #[test]
    fn clear_sets_len_to_zero() {
        let mut q = Queue::<i32>::new();
        q.enqueue(1);
        q.enqueue(2);
        q.clear();
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn fifo_handles_duplicate_values() {
        let mut q = Queue::<i32>::new();
        q.enqueue(5);
        q.enqueue(5);
        q.enqueue(5);
        assert_eq!(q.dequeue(), Some(5));
        assert_eq!(q.dequeue(), Some(5));
        assert_eq!(q.dequeue(), Some(5));
        assert_eq!(q.dequeue(), None);
    }
}
