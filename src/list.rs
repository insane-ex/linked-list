use std::{
    fmt::{self, Display},
    marker::PhantomData,
    mem,
    ptr::{self, NonNull},
};

use super::{
    node::{Link, Node},
    node_allocator::{allocate_node, deallocate_node},
};

pub struct ListIter<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current?;

        unsafe {
            let node_ref = node.as_ref();

            self.current = node_ref.next;

            Some(&node_ref.element)
        }
    }
}

pub struct ListIterMut<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.current?;

        unsafe {
            let node_ref = node.as_mut();

            self.current = node_ref.next;

            Some(&mut node_ref.element)
        }
    }
}

pub struct ListIntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter { list: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = ListIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

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

    #[must_use]
    pub const fn iter(&self) -> ListIter<'_, T> {
        ListIter {
            current: self.head,
            _marker: PhantomData,
        }
    }

    pub const fn iter_mut(&mut self) -> ListIterMut<'_, T> {
        ListIterMut {
            current: self.head,
            _marker: PhantomData,
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
    use super::{LinkedList, Node, fmt};

    mod utils {
        use super::{LinkedList, Node, fmt};

        pub fn new_list<T>() -> LinkedList<T> {
            LinkedList::<T>::new()
        }

        pub fn raw_head<T>(list: &LinkedList<T>) -> &Node<T> {
            unsafe { list.head.expect("head should be Some").as_ref() }
        }

        pub fn raw_tail<T>(list: &LinkedList<T>) -> &Node<T> {
            unsafe { list.tail.expect("tail should be Some").as_ref() }
        }

        pub fn assert_node<T>(node: &Node<T>, prev: bool, next: bool, element: &T)
        where
            T: fmt::Debug + PartialEq,
        {
            assert_eq!(node.previous.is_some(), prev);
            assert_eq!(node.next.is_some(), next);
            assert_eq!(node.element, *element);
        }

        pub fn assert_empty_list<T>(list: &LinkedList<T>) {
            assert!(list.head.is_none());
            assert!(list.tail.is_none());
            assert_eq!(list.size, 0);
        }
    }

    mod misc {
        use super::{
            LinkedList,
            utils::{assert_empty_list, assert_node, new_list, raw_head, raw_tail},
        };

        #[test]
        fn create_list() {
            let list = new_list::<i32>();

            assert_empty_list(&list);
        }

        #[test]
        fn create_list_using_default() {
            let list = LinkedList::<i32>::default();

            assert_empty_list(&list);
        }

        #[test]
        fn is_empty_returns_true_on_empty_list() {
            let list = new_list::<i32>();

            assert!(list.is_empty());
        }

        #[test]
        fn is_empty_returns_false_on_non_empty_list() {
            let mut list = new_list();

            list.push_front(1);

            assert!(!list.is_empty());
        }

        #[test]
        fn len_returns_zero_on_empty_list() {
            let list = new_list::<i32>();

            assert_eq!(list.len(), 0);
        }

        #[test]
        fn len_returns_grater_than_zero_on_non_empty_list() {
            let mut list = new_list();

            list.push_front(1);

            assert_eq!(list.len(), 1);
        }

        #[test]
        fn contains_returns_false() {
            let list = new_list();

            assert!(!list.contains(&1));
        }

        #[test]
        fn contains_returns_true() {
            let mut list = new_list();

            list.push_front(1);

            assert!(list.contains(&1));
        }

        #[test]
        fn contains_returns_true_on_second_node() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);

            assert!(list.contains(&2));
        }

        #[test]
        fn contains_returns_false_on_nonexistent_element() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            assert!(!list.contains(&4));
        }

        #[test]
        fn front_on_empty_list() {
            let list = new_list::<i32>();

            assert!(list.front().is_none());
        }

        #[test]
        fn front_on_non_empty_list() {
            let mut list = new_list();

            list.push_back(10);

            assert_eq!(list.front(), Some(&10));
        }

        #[test]
        fn front_mut_on_empty_list() {
            let mut list = new_list::<i32>();

            assert!(list.front_mut().is_none());
        }

        #[test]
        fn front_mut_on_non_empty_list() {
            let mut list = new_list();

            list.push_back(10);

            if let Some(element) = list.front_mut() {
                *element = 20;
            }

            assert_eq!(list.front(), Some(&20));
        }

        #[test]
        fn back_on_empty_list() {
            let list = new_list::<i32>();

            assert!(list.back().is_none());
        }

        #[test]
        fn back_on_non_empty_list() {
            let mut list = new_list();

            list.push_back(10);
            list.push_back(30);

            assert_eq!(list.back(), Some(&30));
        }

        #[test]
        fn back_mut_on_empty_list() {
            let mut list = new_list::<i32>();

            assert!(list.back_mut().is_none());
        }

        #[test]
        fn back_mut_on_non_empty_list() {
            let mut list = new_list();

            list.push_back(10);
            list.push_back(30);

            if let Some(element) = list.back_mut() {
                *element = 50;
            }

            assert_eq!(list.back(), Some(&50));
        }

        #[test]
        fn clear_list() {
            let mut list = new_list();

            list.push_front(1);
            list.clear();

            assert_empty_list(&list);
        }

        #[test]
        fn display_output_on_empty_list() {
            let list = new_list::<i32>();
            assert_eq!(format!("{list}"), "[]");
        }

        #[test]
        fn display_output_on_non_empty_list() {
            let mut list = new_list();

            list.push_front(1);

            assert_eq!(format!("{list}"), "[1]");

            list.push_front(2);

            assert_eq!(format!("{list}"), "[2 <-> 1]");
        }

        #[test]
        fn reverse_empty_list() {
            let mut list = new_list::<i32>();

            list.reverse();

            assert_eq!(format!("{list}"), "[]");
        }

        #[test]
        fn reverse_list_with_one_element() {
            let mut list = new_list::<i32>();

            list.push_front(1);
            list.reverse();

            assert_eq!(format!("{list}"), "[1]");
        }

        #[test]
        fn reverse_non_empty_list() {
            let mut list = new_list();

            list.push_front(1);
            list.push_front(2);
            list.push_front(3);
            list.reverse();

            assert_eq!(format!("{list}"), "[1 <-> 2 <-> 3]");

            list.reverse();

            assert_eq!(format!("{list}"), "[3 <-> 2 <-> 1]");
        }

        #[test]
        fn split_list_with_even_size() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);
            list.push_back(4);

            let (first_list, second_list) = list.split();

            let head_ref = raw_head(&first_list);
            let tail_ref = raw_tail(&first_list);

            assert_node(head_ref, false, true, &1);
            assert_node(tail_ref, true, false, &2);

            assert_eq!(first_list.len(), 2);

            let head_ref = raw_head(&second_list);
            let tail_ref = raw_tail(&second_list);

            assert_node(head_ref, false, true, &3);
            assert_node(tail_ref, true, false, &4);

            assert_eq!(second_list.len(), 2);
        }

        #[test]
        fn split_list_with_odd_size() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            let (first_list, second_list) = list.split();

            let head_ref = raw_head(&first_list);
            let tail_ref = raw_tail(&first_list);

            assert_node(head_ref, false, true, &1);
            assert_node(tail_ref, true, false, &2);

            assert_eq!(first_list.len(), 2);

            let head_ref = raw_head(&second_list);
            let tail_ref = raw_tail(&second_list);

            assert_node(head_ref, false, false, &3);
            assert_node(tail_ref, false, false, &3);

            assert_eq!(second_list.len(), 1);
        }

        #[test]
        fn retain_list_even_elements() {
            let mut list = new_list();

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
        fn retain_list_odd_elements() {
            let mut list = new_list();

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

    mod push {
        use super::utils::{assert_node, new_list, raw_head, raw_tail};

        #[test]
        fn push_front_one_element() {
            let mut list = new_list();

            list.push_front(1);

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, false, &1);
            assert_node(tail_ref, false, false, &1);

            assert_eq!(list.size, 1);
        }

        #[test]
        fn push_front_two_elements() {
            let mut list = new_list();

            list.push_front(1);

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, false, &1);
            assert_node(tail_ref, false, false, &1);

            list.push_front(2);

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, true, &2);
            assert_node(tail_ref, true, false, &1);

            assert_eq!(list.size, 2);
        }

        #[test]
        fn push_back_one_element() {
            let mut list = new_list();

            list.push_back(1);

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, false, &1);
            assert_node(tail_ref, false, false, &1);

            assert_eq!(list.size, 1);
        }

        #[test]
        fn push_back_two_elements() {
            let mut list = new_list();

            list.push_back(1);

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, false, &1);
            assert_node(tail_ref, false, false, &1);

            list.push_back(2);

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, true, &1);
            assert_node(tail_ref, true, false, &2);

            assert_eq!(list.size, 2);
        }
    }

    mod pop {
        use crate::list::tests::utils::assert_empty_list;

        use super::utils::{assert_node, new_list, raw_head, raw_tail};

        #[test]
        fn pop_front_empty_list() {
            let mut list = new_list::<i32>();

            assert!(list.pop_front().is_none());
            assert_eq!(list.size, 0);
        }

        #[test]
        fn pop_front_one_element() {
            let mut list = new_list();

            list.push_front(1);

            let popped_element = list.pop_front();

            assert_eq!(popped_element, Some(1));

            assert_empty_list(&list);
        }

        #[test]
        fn pop_front_two_elements() {
            let mut list = new_list();

            list.push_front(1);
            list.push_front(2);

            let first_pop = list.pop_front();

            assert_eq!(first_pop, Some(2));

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, false, &1);
            assert_node(tail_ref, false, false, &1);

            let second_pop = list.pop_front();

            assert_eq!(second_pop, Some(1));

            assert_empty_list(&list);
        }

        #[test]
        fn pop_back_empty_list() {
            let mut list = new_list::<i32>();

            assert!(list.pop_back().is_none());

            assert_eq!(list.size, 0);
        }

        #[test]
        fn pop_back_one_element() {
            let mut list = new_list();

            list.push_front(1);

            let popped_element = list.pop_back();

            assert_eq!(popped_element, Some(1));

            assert_empty_list(&list);
        }

        #[test]
        fn pop_back_two_elements() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);

            let first_pop = list.pop_back();

            assert_eq!(first_pop, Some(2));

            let head_ref = raw_head(&list);
            let tail_ref = raw_tail(&list);

            assert_node(head_ref, false, false, &1);
            assert_node(tail_ref, false, false, &1);

            let second_pop = list.pop_back();

            assert_eq!(second_pop, Some(1));

            assert_empty_list(&list);
        }
    }

    mod iter {
        use super::utils::new_list;

        #[test]
        fn iter_should_traverse_in_order() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            let collected: Vec<i32> = list.iter().copied().collect();

            assert_eq!(collected, vec![1, 2, 3]);
        }

        #[test]
        fn iter_empty_list() {
            let list = new_list::<i32>();

            assert!(list.iter().copied().next().is_none());
        }

        #[test]
        fn iter_mut_should_modify_elements() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            for x in &mut list {
                *x *= 2;
            }

            let collected: Vec<i32> = list.iter().copied().collect();

            assert_eq!(collected, vec![2, 4, 6]);
        }

        #[test]
        fn into_iter_should_consume_list() {
            let mut list = new_list();

            list.push_back(10);
            list.push_back(20);

            let collected: Vec<i32> = list.into_iter().collect();

            assert_eq!(collected, vec![10, 20]);
        }

        #[test]
        fn for_loop_on_ref_list() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);

            let mut sum = 0;

            for x in &list {
                sum += *x;
            }

            assert_eq!(sum, 3);
        }

        #[test]
        fn for_loop_on_mut_ref_list() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);

            for x in &mut list {
                *x += 10;
            }

            let collected: Vec<i32> = list.iter().copied().collect();

            assert_eq!(collected, vec![11, 12]);
        }

        #[test]
        fn for_loop_consuming_list() {
            let mut list = new_list();

            list.push_back(5);
            list.push_back(6);

            let mut v = vec![];

            for x in list {
                v.push(x);
            }

            assert_eq!(v, vec![5, 6]);
        }

        #[test]
        fn partial_iteration() {
            let mut list = new_list();

            list.push_back(1);
            list.push_back(2);
            list.push_back(3);

            let mut iter = list.iter();

            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
        }
    }
}
