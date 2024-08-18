use std::{cell::RefCell, rc::Rc};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let head = Rc::new(RefCell::new(head));
    let mut j = head.clone();
    let mut i = head.clone();
    let mut p = i.clone();

    while let Some(node) = j.borrow().into() {
        println!("j is {:?}", j);
        println!("i is {:?}", i);
        println!("p is {:?}", p);
        // let node = node.unwrap();
        j = if node.as_ref().unwrap().next.as_ref().is_some() {
            let node = node.as_ref().unwrap().next.unwrap();
            if node.next.is_some() {
                p = i;
                i = Rc::new(RefCell::new(i.borrow().unwrap().next));
                Rc::new(RefCell::new(node.next))
            } else {
                Rc::new(RefCell::new(Some(node)))
            }
        } else {
            Rc::new(RefCell::new(None))
        };
    }
    // o = i.take();
    // **p.next.as_mut().unwrap() = *Box::new(*i.as_mut().unwrap().next.unwrap());
    Some(Box::new(ListNode::new(7)))
}

fn main() {
    println!("Hello, world!");
}
