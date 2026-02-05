use std::ptr::NonNull;

pub type Link<T> = Option<NonNull<Node<T>>>;

pub struct Node<T> {
    pub previous: Link<T>,
    pub next: Link<T>,
    pub element: T,
}

impl<T> Node<T> {
    pub const fn new(element: T) -> Self {
        Self {
            previous: None,
            next: None,
            element,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn create_new_node() {
        let node = Node::new(1);

        assert!(node.previous.is_none());
        assert!(node.next.is_none());
        assert_eq!(node.element, 1);
    }
}
