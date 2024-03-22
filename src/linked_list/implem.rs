use std::cell::Cell;
use std::rc::Rc;
use std::rc::Weak;

use super::handle::Handle;
use super::iterator::NodeIterator;
use super::node::Node;
use super::with_value;

pub(super) struct LinkedListImpl<V> {
    pub node: Cell<Weak<Node<V>>>,
}

impl<V> LinkedListImpl<V> {
    pub fn push_back(self: &Rc<Self>, value: V) -> Handle<V> {
        let new = Rc::new(Node::from(value));
        let node = with_value(&self.node, |node| node.clone());
        if let Some(node) = node.upgrade() {
            // prev     <->     self <-> next <-> prev
            // prev <-> new <-> self <-> next <-> prev
            let prev = with_value(&node.prev, |prev| prev.upgrade()).unwrap();
            new.prev.set(Rc::downgrade(&prev));
            prev.next.set(Rc::downgrade(&new));
            new.next.set(Rc::downgrade(&node));
            node.prev.set(Rc::downgrade(&new));
        } else {
            self.node.set(Rc::downgrade(&new));
            new.prev.set(Rc::downgrade(&new));
            new.next.set(Rc::downgrade(&new));
        }
        Handle {
            list: self.clone(),
            node: new,
        }
    }

    pub fn prev(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, |node| node.clone());
        if let Some(node) = node.upgrade() {
            let prev = with_value(&node.prev, |prev| prev.clone());
            Rc::new(LinkedListImpl {
                node: Cell::new(prev),
            })
        } else {
            self.clone()
        }
    }

    pub fn next(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, |node| node.clone());
        if let Some(node) = node.upgrade() {
            let next = with_value(&node.next, |next| next.clone());
            Rc::new(LinkedListImpl {
                node: Cell::new(next),
            })
        } else {
            self.clone()
        }
    }
}

impl<V: Clone> LinkedListImpl<V> {
    pub fn iter(&self) -> impl Iterator<Item = V> {
        let next = with_value(&self.node, |node| node.upgrade());
        let stop = next
            .as_ref()
            .and_then(|next| with_value(&next.prev, |prev| prev.upgrade()));
        NodeIterator { next, stop }
    }

    pub fn current(&self) -> Option<V> {
        let current = with_value(&self.node, |node| node.upgrade());
        current.map(|node| node.value.clone())
    }
}

impl<V> Default for LinkedListImpl<V> {
    fn default() -> Self {
        Self {
            node: Default::default(),
        }
    }
}
