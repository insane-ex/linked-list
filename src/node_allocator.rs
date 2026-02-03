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
