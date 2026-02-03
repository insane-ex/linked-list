use std::{
    alloc::{Layout, alloc, dealloc},
    mem::{self},
    ptr::{self, NonNull},
};

use super::node::Node;

#[allow(unused)]
pub fn allocate_node<T>(node: Node<T>) -> NonNull<Node<T>> {
    assert!(mem::size_of::<Node<T>>() != 0);

    let layout = Layout::new::<Node<T>>();

    let raw_ptr = unsafe {
        let ptr = alloc(layout).cast::<Node<T>>();

        assert!(!ptr.is_null());

        ptr::write(ptr, node);

        ptr
    };

    unsafe { NonNull::new_unchecked(raw_ptr) }
}

#[allow(unused)]
pub unsafe fn deallocate_node<T>(node: NonNull<Node<T>>) {
    unsafe { dealloc(node.as_ptr().cast::<u8>(), Layout::new::<Node<T>>()) };
}

#[cfg(test)]
mod tests {
    use crate::node_allocator::deallocate_node;

    use super::{Node, allocate_node};

    #[test]
    fn test_allocate_node() {
        let node = Node::new(1);
        let node_ptr = allocate_node(node);

        unsafe {
            let node_ref = node_ptr.as_ref();

            assert!(node_ref.previous.is_none());
            assert!(node_ref.next.is_none());
            assert_eq!(node_ref.element, 1);

            deallocate_node(node_ptr);
        }
    }
}
