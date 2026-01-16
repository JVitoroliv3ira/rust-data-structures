pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
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
        let mut s = Stack::<i32>::new();
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn empty_stack_peak_are_none() {
        let s = Stack::<i32>::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn len_tracks_number_of_elements() {
        let mut s = Stack::<i32>::new();
        s.push(34);
        s.push(9);
        s.push(314);
        s.push(1981);
        s.push(13);
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn is_empty_is_true_for_new_stack() {
        let s = Stack::<i32>::new();
        assert!(s.is_empty());
    }

    #[test]
    fn is_empty_is_false_after_push() {
        let mut s = Stack::<i32>::new();
        s.push(5);
        assert!(!s.is_empty());
    }

    #[test]
    fn peek_does_not_remove_element() {
        let mut s = Stack::<i32>::new();
        s.push(10);
        assert_eq!(s.peek(), Some(&10));
        assert_eq!(s.peek(), Some(&10));
        assert_eq!(s.len(), 1);
        assert_eq!(s.pop(), Some(10));
    }

    #[test]
    fn peek_returns_top_element() {
        let mut s = Stack::<i32>::new();
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        assert_eq!(s.peek(), Some(&4));
    }

    #[test]
    fn lifo_order() {
        let mut s = Stack::<i32>::new();
        s.push(4);
        s.push(100);
        s.push(25);
        assert_eq!(s.pop(), Some(25));
        assert_eq!(s.pop(), Some(100));
        assert_eq!(s.pop(), Some(4));
    }

    #[test]
    fn default_creates_empty_stack() {
        let s: Stack<i32> = Default::default();
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn pop_decreases_len_and_updates_peek() {
        let mut s = Stack::<i32>::new();
        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(s.len(), 3);
        assert_eq!(s.peek(), Some(&3));

        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.len(), 2);
        assert_eq!(s.peek(), Some(&2));

        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.len(), 1);
        assert_eq!(s.peek(), Some(&1));

        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
        assert_eq!(s.peek(), None);

        // pop extra continua sendo None
        assert_eq!(s.pop(), None);
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn push_pop_interleaving_works() {
        let mut s = Stack::<i32>::new();

        s.push(10);
        s.push(20);
        assert_eq!(s.pop(), Some(20));

        s.push(30);
        assert_eq!(s.peek(), Some(&30));
        assert_eq!(s.pop(), Some(30));
        assert_eq!(s.pop(), Some(10));

        assert!(s.is_empty());
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn is_empty_toggles_correctly() {
        let mut s = Stack::<i32>::new();
        assert!(s.is_empty());

        s.push(1);
        assert!(!s.is_empty());

        s.push(2);
        assert!(!s.is_empty());

        s.pop();
        assert!(!s.is_empty());

        s.pop();
        assert!(s.is_empty());
    }

    #[test]
    fn works_with_non_copy_types() {
        let mut s = Stack::<String>::new();
        s.push("a".to_string());
        s.push("b".to_string());

        assert_eq!(s.len(), 2);
        assert_eq!(s.peek().map(|x| x.as_str()), Some("b"));

        let top = s.pop();
        assert_eq!(top.as_deref(), Some("b"));
        assert_eq!(s.peek().map(|x| x.as_str()), Some("a"));

        let next = s.pop();
        assert_eq!(next.as_deref(), Some("a"));
        assert!(s.is_empty());
    }

    #[test]
    fn peek_on_empty_after_clearing_by_pop() {
        let mut s = Stack::<i32>::new();
        s.push(42);
        assert_eq!(s.pop(), Some(42));
        assert_eq!(s.peek(), None);
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }
}
