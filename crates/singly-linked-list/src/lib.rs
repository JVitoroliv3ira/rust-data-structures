pub struct SinglyLinkedListNode {
    value: i32,
    next: Option<Box<SinglyLinkedListNode>>,
}

impl SinglyLinkedListNode {
    pub fn new(value: i32, next: Option<Box<SinglyLinkedListNode>>) -> Self {
        Self { value, next }
    }
}

pub struct SinglyLinkedList {
    head: Option<Box<SinglyLinkedListNode>>,
    counter: usize,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        Self {
            head: Option::None,
            counter: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.counter
    }

    pub fn is_empty(&self) -> bool {
        self.counter == 0
    }

    pub fn peek_front(&self) -> Option<&i32> {
        self.head.as_deref().map(|n| &n.value)
    }

    pub fn push_front(&mut self, value: i32) {
        let old_head = self.head.take();
        let node = SinglyLinkedListNode::new(value, old_head);

        self.head = Some(Box::new(node));
        self.counter += 1;
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|mut n| {
            self.head = n.next.take();
            self.counter -= 1;
            n.value
        })
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.counter = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_is_zero_for_new_list() {
        let l = SinglyLinkedList::new();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn is_empty_is_true_for_new_list() {
        let l = SinglyLinkedList::new();
        assert!(l.is_empty());
    }

    #[test]
    fn peek_front_returns_none_for_new_list() {
        let l = SinglyLinkedList::new();
        assert_eq!(l.peek_front(), Option::None);
    }

    #[test]
    fn push_front_increases_len() {
        let mut l = SinglyLinkedList::new();
        l.push_front(15);
        assert_eq!(l.len(), 1);
        l.push_front(7);
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn push_front_makes_is_empty_false() {
        let mut l = SinglyLinkedList::new();
        l.push_front(15);
        assert!(!l.is_empty())
    }

    #[test]
    fn peek_front_returns_head_of_list() {
        let mut l = SinglyLinkedList::new();
        l.push_front(5);
        assert_eq!(l.peek_front(), Some(&5));
    }

    #[test]
    fn peek_front_does_not_remove_element() {
        let mut l = SinglyLinkedList::new();
        l.push_front(5);
        assert_eq!(l.peek_front(), Some(&5));
        assert_eq!(l.peek_front(), Some(&5));
    }

    #[test]
    fn push_front_sets_head_as_new_value() {
        let mut l = SinglyLinkedList::new();
        l.push_front(8);
        assert_eq!(l.peek_front(), Some(&8));
        l.push_front(95);
        assert_eq!(l.peek_front(), Some(&95));
    }

    #[test]
    fn pop_front_decreases_len() {
        let mut l = SinglyLinkedList::new();
        l.push_front(4);
        l.push_front(9);
        assert_eq!(l.len(), 2);
        l.pop_front();
        assert_eq!(l.len(), 1);
        l.pop_front();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn pop_front_remove_element() {
        let mut l = SinglyLinkedList::new();
        l.push_front(9);
        l.push_front(7);
        assert_eq!(l.peek_front(), Some(&7));
        l.pop_front();
        assert_eq!(l.peek_front(), Some(&9));
    }

    #[test]
    fn pop_front_returns_last_pushed_front() {
        let mut l = SinglyLinkedList::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        assert_eq!(l.pop_front(), Some(3));
    }

    #[test]
    fn pop_front_util_empty_then_none() {
        let mut l = SinglyLinkedList::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        assert_eq!(l.pop_front(), Some(3));
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), Option::None);
    }

    #[test]
    fn clear_sets_len_to_zero() {
        let mut l = SinglyLinkedList::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        l.clear();

        assert_eq!(l.len(), 0);
    }

    #[test]
    fn clear_makes_is_empty_true() {
        let mut l = SinglyLinkedList::new();
        l.push_front(10);

        l.clear();

        assert!(l.is_empty());
    }

    #[test]
    fn clear_after_operations_resets_state() {
        let mut l = SinglyLinkedList::new();

        l.push_front(1);
        l.push_front(2);
        l.pop_front();
        l.push_front(3);

        l.clear();

        assert_eq!(l.len(), 0);
        assert!(l.is_empty());
        assert_eq!(l.peek_front(), None);
        assert_eq!(l.pop_front(), None);
    }

    #[test]
    fn alternating_push_front_and_pop_front_maintains_invariants() {
        let mut l = SinglyLinkedList::new();

        l.push_front(1);
        assert_eq!(l.peek_front(), Some(&1));
        assert_eq!(l.len(), 1);

        assert_eq!(l.pop_front(), Some(1));
        assert!(l.is_empty());
        assert_eq!(l.len(), 0);

        l.push_front(2);
        l.push_front(3);
        assert_eq!(l.peek_front(), Some(&3));
        assert_eq!(l.len(), 2);

        assert_eq!(l.pop_front(), Some(3));
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), None);

        assert!(l.is_empty());
        assert_eq!(l.len(), 0);
    }
}
