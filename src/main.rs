use std::cell::RefCell;
use std::iter::{Skip, Peekable};
use std::iter::Take;
use std::rc::Rc;

type NodePt = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    data: i32,
    next_node: Option<NodePt>,
}

impl Node {
    fn new(data: i32) -> Self {
        Self {
            data,
            next_node: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Option<NodePt>,
    tail: Option<NodePt>,
    len: usize,
}

impl<'a> IntoIterator for &'a LinkedList {
    type Item = NodePt;
    type IntoIter = LinkedListIterator;

    fn into_iter(self) -> Self::IntoIter {
        let current = Some(self.head.as_ref().unwrap().clone());
        LinkedListIterator {
            current,
        }
    }
}

struct LinkedListIterator {
    current: Option<NodePt>,
}

impl Iterator for LinkedListIterator {
    type Item = NodePt;
    fn next(&mut self) -> Option<NodePt> {
        let (current_node, next_node) = match self.current {
            Some(ref node) => {
                let next = &RefCell::borrow(node).next_node;
                let next_node = match next {
                    Some(next_node) => {
                        Some(next_node.clone())
                    }
                    None => None
                };
                (Some(node.clone()), next_node)
            }
            None => (None, None)
        };
        self.current = next_node;
        current_node
    }
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn push(&mut self, data: i32) {
        self.len += 1;
        let node = Rc::new(RefCell::new(Node::new(data)));
        match &self.head {
            Some(_) => {
                RefCell::borrow_mut(self.tail.as_ref().unwrap()).next_node = Some(node.clone());
                self.tail = Some(node.clone());
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
        }
    }

    fn sublist_peekable_iter(&self, skip_num: usize, take_num: usize) -> Peekable<Take<Skip<LinkedListIterator>>> {
        self.into_iter().skip(skip_num).take(take_num).peekable()
    }

    fn get_node(&self, node_idx: usize) -> Option<NodePt> {
        self.into_iter().skip(node_idx).take(1).next()
    }

    fn print_list(&self) {
        print!("index: ");
        for (k, _) in self.into_iter().enumerate() {
            print!("{:>5}", k);
        }
        println!();
        print!("value: ");
        for (_, v) in self.into_iter().enumerate() {
            print!("{:>5}", v.borrow().data);
        }
        println!();
    }
}

fn link_node(head: &mut Option<NodePt>, tail: &mut Option<NodePt>, node: NodePt) {
    match head {
        Some(_head) => {
            RefCell::borrow_mut(&tail.as_ref().unwrap()).next_node = Some(node.clone());
            *tail = Some(node.clone());
            RefCell::borrow_mut(&node).next_node = None;
        }
        None => {
            *head = Some(node.clone());
            *tail = Some(node.clone());
        }
    }
}

fn merge_to_list(linked_list: &mut LinkedList,
                 merge_start_node: Option<NodePt>, merge_end_node: Option<NodePt>,
                 head: Option<NodePt>, tail: Option<NodePt>) {
    match merge_start_node {
        Some(merge_start_node) => {
            RefCell::borrow_mut(&merge_start_node).next_node = Some(head.unwrap().clone());
        }
        None => {
            linked_list.head = Some(head.unwrap().clone());
        }
    };
    match merge_end_node {
        Some(merge_end_node) => {
            let tail_mut = &mut tail.unwrap();
            RefCell::borrow_mut(tail_mut).next_node = Some(merge_end_node.clone());
        }
        None => {
            linked_list.tail = Some(tail.unwrap().clone());
        }
    };
}

fn get_merge_nodes(linked_list: &LinkedList, start: usize, end: usize) -> (Option<NodePt>, Option<NodePt>) {
    let mut merge_start_node: Option<NodePt> = None;
    let mut merge_end_node: Option<NodePt> = None;
    if start > 0 {
        merge_start_node = Some(linked_list.get_node(start - 1).as_ref().unwrap().clone());
    }
    if end < linked_list.len {
        merge_end_node = linked_list.get_node(end).clone();
    }
    (merge_start_node, merge_end_node)
}

fn merge(linked_list: &mut LinkedList, start: usize, mid: usize, end: usize) {
    let (merge_start_node, merge_end_node) = get_merge_nodes(linked_list, start, end);
    let mut merge_end = false;
    let mut head: Option<NodePt> = None;
    let mut tail: Option<NodePt> = None;
    let mut iter_left = linked_list.sublist_peekable_iter(start, mid - start);
    let mut iter_right = linked_list.sublist_peekable_iter(mid, end - mid);
    while !merge_end {
        let right_value = if let Some(il) = iter_right.peek() {
            Some(RefCell::borrow(&il).data)
        } else { None };
        let left_value = if let Some(ir) = iter_left.peek() {
            Some(RefCell::borrow(&ir).data)
        } else { None };
        match (left_value, right_value) {
            (None, None) => {
                merge_end = true
            }
            (Some(left_value), Some(right_value)) => {
                if left_value < right_value {
                    link_node(&mut head, &mut tail, iter_left.next().unwrap());
                } else {
                    link_node(&mut head, &mut tail, iter_right.next().unwrap());
                };
            }
            (Some(_left_value), None) => {
                link_node(&mut head, &mut tail, iter_left.next().unwrap());
            }
            (None, Some(_right_value)) => {
                link_node(&mut head, &mut tail, iter_right.next().unwrap());
            }
        }
    }
    merge_to_list(linked_list, merge_start_node, merge_end_node, head, tail);
}

fn merge_sort(linked_list: &mut LinkedList, start: usize, mid: usize, end: usize) {
    if start >= mid {
        return;
    }
    merge_sort(linked_list, start, (mid - start) / 2, mid);
    merge_sort(linked_list, mid, mid + (end - mid) / 2, end);
    merge(linked_list, start, mid, end);
}

fn main() {
    let mut linked_list = LinkedList::new();
    linked_list.push(15);
    linked_list.push(10);
    linked_list.push(5);
    linked_list.push(20);
    linked_list.push(3);
    linked_list.push(2);
    println!("before sort:");
    linked_list.print_list();
    let linked_list_len = linked_list.len;
    merge_sort(&mut linked_list, 0, linked_list_len / 2, linked_list_len);
    println!("after sort:");
    linked_list.print_list();
}
