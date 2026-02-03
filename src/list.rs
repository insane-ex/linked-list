use std::ptr::{self};

use super::{
    node::{Link, Node},
    node_allocator::{allocate_node, deallocate_node},
};

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.next = self.head;

        let new_node_ptr = allocate_node(new_node);

        if let Some(mut old_head) = self.head {
            unsafe { old_head.as_mut().previous = Some(new_node_ptr) };
        } else {
            self.tail = Some(new_node_ptr);
        }

        self.head = Some(new_node_ptr);
        self.size += 1;
    }

    pub fn push_back(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.previous = self.tail;

        let new_node_ptr = allocate_node(new_node);

        if let Some(mut old_tail) = self.tail {
            unsafe { old_tail.as_mut().next = Some(new_node_ptr) };
        } else {
            self.head = Some(new_node_ptr);
        }

        self.tail = Some(new_node_ptr);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head?;

        self.head = unsafe { old_head.as_ref().next };

        if let Some(mut new_head) = self.head {
            unsafe { new_head.as_mut().previous = None };
        } else {
            self.tail = None;
        }

        let popped_element = unsafe { ptr::read(&raw const old_head.as_ref().element) };

        unsafe { deallocate_node(old_head) };

        self.size -= 1;

        Some(popped_element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let old_tail = self.tail?;

        self.tail = unsafe { old_tail.as_ref().previous };

        if let Some(mut new_tail) = self.tail {
            unsafe { new_tail.as_mut().next = None };
        } else {
            self.head = None;
        }

        let popped_element = unsafe { ptr::read(&raw const old_tail.as_ref().element) };

        unsafe { deallocate_node(old_tail) };

        self.size -= 1;

        Some(popped_element)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[must_use]
    pub fn front(&self) -> Option<&T> {
        self.head.map(|head| unsafe { &head.as_ref().element })
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|mut head| unsafe { &mut head.as_mut().element })
    }

    #[must_use]
    pub fn back(&self) -> Option<&T> {
        self.tail.map(|tail| unsafe { &tail.as_ref().element })
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail
            .map(|mut tail| unsafe { &mut tail.as_mut().element })
    }

    pub fn contains(&self, element: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current_node = self.head;

        while let Some(node) = current_node {
            let node_ref = unsafe { node.as_ref() };

            if node_ref.element == *element {
                return true;
            }

            current_node = node_ref.next;
        }

        false
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkedList, Node};

    fn raw_head<T>(list: &LinkedList<T>) -> &Node<T> {
        unsafe { list.head.expect("head should be Some").as_ref() }
    }

    fn raw_tail<T>(list: &LinkedList<T>) -> &Node<T> {
        unsafe { list.tail.expect("tail should be Some").as_ref() }
    }

    #[test]
    fn test_create_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_push_front_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        let head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_none());
        assert_eq!(head_ref.element, 1);

        let tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_none());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 1);

        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_push_front_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        let mut head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_none());
        assert_eq!(head_ref.element, 1);

        let mut tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_none());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 1);

        list.push_front(2);

        head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_some());
        assert_eq!(head_ref.element, 2);

        tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_some());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 1);

        assert_eq!(list.size, 2);
    }

    #[test]
    fn test_push_back_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);

        let head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_none());
        assert_eq!(head_ref.element, 1);

        let tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_none());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 1);

        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_push_back_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);

        let mut head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_none());
        assert_eq!(head_ref.element, 1);

        let mut tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_none());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 1);

        list.push_back(2);

        head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_some());
        assert_eq!(head_ref.element, 1);

        tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_some());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 2);

        assert_eq!(list.size, 2);
    }

    #[test]
    fn test_pop_front_empty_list() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.pop_front().is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_front_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        let popped_element = list.pop_front();

        assert!(popped_element.is_some());
        assert_eq!(popped_element.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_front_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);
        list.push_front(2);

        let first_pop = list.pop_front();

        assert!(first_pop.is_some());
        assert_eq!(first_pop.unwrap(), 2);

        let head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_none());
        assert_eq!(head_ref.element, 1);

        let tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_none());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 1);

        let second_pop = list.pop_front();

        assert!(second_pop.is_some());
        assert_eq!(second_pop.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_back_empty_list() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.pop_back().is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_back_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        let popped_element = list.pop_back();

        assert!(popped_element.is_some());
        assert_eq!(popped_element.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_back_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);

        let first_pop = list.pop_back();

        assert!(first_pop.is_some());
        assert_eq!(first_pop.unwrap(), 2);

        let head_ref = raw_head(&list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_none());
        assert_eq!(head_ref.element, 1);

        let tail_ref = raw_tail(&list);

        assert!(tail_ref.previous.is_none());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 1);

        let second_pop = list.pop_back();

        assert!(second_pop.is_some());
        assert_eq!(second_pop.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_is_empty_on_empty_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.is_empty());
    }

    #[test]
    fn test_is_empty_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        assert!(!list.is_empty());
    }

    #[test]
    fn test_len_on_empty_list() {
        let list = LinkedList::<i32>::new();

        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_len_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_contains_returns_false() {
        let list = LinkedList::<i32>::new();

        assert!(!list.contains(&1));
    }

    #[test]
    fn test_contains_returns_true() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        assert!(list.contains(&1));
    }

    #[test]
    fn test_front_on_empty_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.front().is_none());
    }

    #[test]
    fn test_front_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(10);

        assert_eq!(list.front(), Some(&10));
    }

    #[test]
    fn test_front_mut_on_empty_list() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.front_mut().is_none());
    }

    #[test]
    fn test_front_mut_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(10);

        if let Some(element) = list.front_mut() {
            *element = 20;
        }

        assert_eq!(list.front(), Some(&20));
    }

    #[test]
    fn test_back_on_empty_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.back().is_none());
    }

    #[test]
    fn test_back_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(10);
        list.push_back(30);

        assert_eq!(list.back(), Some(&30));
    }

    #[test]
    fn test_back_mut_on_empty_list() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.back_mut().is_none());
    }

    #[test]
    fn test_back_mut_on_non_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(10);
        list.push_back(30);

        if let Some(element) = list.back_mut() {
            *element = 50;
        }

        assert_eq!(list.back(), Some(&50));
    }
}
