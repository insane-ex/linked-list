use std::fmt;

use super::{LinkedList, node::Node};

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
    use super::utils::{assert_empty_list, assert_node, new_list, raw_head, raw_tail};

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
