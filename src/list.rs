use std::{
    fmt::{self, Display},
    mem::{self},
    ptr::{self, NonNull},
};

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

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    pub const fn reverse(&mut self) {
        if self.is_empty() || self.len() == 1 {
            return;
        }

        let mut current_node = self.head;

        while let Some(mut node) = current_node {
            let next_node = unsafe { node.as_ref().next };

            unsafe {
                node.as_mut().next = node.as_mut().previous;
                node.as_mut().previous = next_node;
            }

            current_node = next_node;
        }

        mem::swap(&mut self.head, &mut self.tail);
    }

    #[must_use]
    pub fn split(self) -> (Self, Self)
    where
        T: Clone,
    {
        let mid = self.len().div_ceil(2);
        let mut index: usize = 0;
        let mut current_node = self.head;
        let mut first_list = Self::new();
        let mut second_list = Self::new();

        while let Some(node) = current_node {
            let node_ref = unsafe { node.as_ref() };

            if index < mid {
                first_list.push_back(node_ref.element.clone());
            } else {
                second_list.push_back(node_ref.element.clone());
            }

            current_node = node_ref.next;
            index += 1;
        }

        (first_list, second_list)
    }

    fn remove_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            let previous_node = node.as_ref().previous;
            let next_node = node.as_ref().next;

            if let Some(mut node) = previous_node {
                node.as_mut().next = next_node;
            } else {
                self.head = next_node;
            }

            if let Some(mut node) = next_node {
                node.as_mut().previous = previous_node;
            } else {
                self.tail = previous_node;
            }

            deallocate_node(node);
        }

        self.size -= 1;
    }

    pub fn retain<F>(&mut self, predicate: F)
    where
        F: Fn(&T) -> bool,
    {
        let mut current_node = self.head;

        while let Some(node) = current_node {
            let next_node = unsafe { node.as_ref().next };

            if !predicate(unsafe { &node.as_ref().element }) {
                self.remove_node(node);
            }

            current_node = next_node;
        }
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "[]");
        }

        write!(f, "[")?;

        let mut current_node = self.head;

        while let Some(node) = current_node {
            let node_ref = unsafe { node.as_ref() };

            if node_ref.next.is_some() {
                write!(f, "{} <-> ", node_ref.element)?;
            } else {
                write!(f, "{}", node_ref.element)?;
            }

            current_node = node_ref.next;
        }

        write!(f, "]")
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
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

    #[test]
    fn test_list_clear() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);
        list.clear();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_empty_list_display_output() {
        let list = LinkedList::<i32>::new();

        assert_eq!(format!("{list}"), "[]");
    }

    #[test]
    fn test_non_empty_list_display_output() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        assert_eq!(format!("{list}"), "[1]");

        list.push_front(2);

        assert_eq!(format!("{list}"), "[2 <-> 1]");
    }

    #[test]
    fn test_reverse_non_empty_list() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.reverse();

        assert_eq!(format!("{list}"), "[1 <-> 2 <-> 3]");

        list.reverse();

        assert_eq!(format!("{list}"), "[3 <-> 2 <-> 1]");
    }

    #[test]
    fn test_split_list_with_even_size() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        let (first_list, second_list) = list.split();

        let mut head_ref = raw_head(&first_list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_some());
        assert_eq!(head_ref.element, 1);

        assert_eq!(first_list.len(), 2);

        let mut tail_ref = raw_tail(&first_list);

        assert!(tail_ref.previous.is_some());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 2);

        head_ref = raw_head(&second_list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_some());
        assert_eq!(head_ref.element, 3);

        tail_ref = raw_tail(&second_list);

        assert!(tail_ref.previous.is_some());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 4);

        assert_eq!(second_list.len(), 2);
    }

    #[test]
    fn test_split_list_with_odd_size() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let (first_list, second_list) = list.split();

        assert_eq!(first_list.len(), 2);

        let mut head_ref = raw_head(&first_list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_some());
        assert_eq!(head_ref.element, 1);

        assert_eq!(first_list.len(), 2);

        let mut tail_ref = raw_tail(&first_list);

        assert!(tail_ref.previous.is_some());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 2);

        head_ref = raw_head(&second_list);

        assert!(head_ref.previous.is_none());
        assert!(head_ref.next.is_none());
        assert_eq!(head_ref.element, 3);

        tail_ref = raw_tail(&second_list);

        assert!(tail_ref.previous.is_none());
        assert!(tail_ref.next.is_none());
        assert_eq!(tail_ref.element, 3);

        assert_eq!(second_list.len(), 1);
    }

    #[test]
    fn test_list_retain_even_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        list.retain(|x| x % 2 == 0);

        assert_eq!(format!("{list}"), "[2 <-> 4 <-> 6]");
    }

    #[test]
    fn test_list_retain_odd_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        list.retain(|x| x % 2 == 1);

        assert_eq!(format!("{list}"), "[1 <-> 3 <-> 5]");
    }
}
