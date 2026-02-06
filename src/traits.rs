use std::fmt::{self, Display};

use super::{
    LinkedList,
    list::{ListIntoIter, ListIter, ListIterMut},
};

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
