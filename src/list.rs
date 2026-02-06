use std::{
    marker::PhantomData,
    mem,
    ptr::{self, NonNull},
};

use super::{
    node::{Link, Node},
    node_allocator::{allocate_node, deallocate_node},
};

pub struct ListIter<'a, T> {
    pub(super) current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

pub struct ListIterMut<'a, T> {
    pub(super) current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a mut T>,
}

pub struct ListIntoIter<T> {
    pub(super) list: LinkedList<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub(super) head: Link<T>,
    pub(super) tail: Link<T>,
    pub(super) size: usize,
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
