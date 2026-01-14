pub struct Stack {
    items: Vec<i32>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn push(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&i32> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_stack_pop_are_none() {
        let mut s = Stack::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn empty_stack_peak_are_none() {
        let s = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn len_tracks_number_of_elements() {
        let mut s = Stack::new();
        s.push(34);
        s.push(9);
        s.push(314);
        s.push(1981);
        s.push(13);
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn is_empty_is_true_for_new_stack() {
        let s = Stack::new();
        assert!(s.is_empty());
    }

    #[test]
    fn is_empty_is_false_after_push() {
        let mut s = Stack::new();
        s.push(5);
        assert!(!s.is_empty());
    }

    #[test]
    fn peek_does_not_remove_element() {
        let mut s = Stack::new();
        s.push(10);
        assert_eq!(s.peek(), Some(&10));
        assert_eq!(s.peek(), Some(&10));
        assert_eq!(s.len(), 1);
        assert_eq!(s.pop(), Some(10));
    }

    #[test]
    fn peek_returns_top_element() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        assert_eq!(s.peek(), Some(&4));
    }

    #[test]
    fn lifo_order() {
        let mut s = Stack::new();
        s.push(4);
        s.push(100);
        s.push(25);
        assert_eq!(s.pop(), Some(25));
        assert_eq!(s.pop(), Some(100));
        assert_eq!(s.pop(), Some(4));
    }
}
