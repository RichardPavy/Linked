use std::rc::Rc;

use super::implem::LinkedListImpl;
use super::node::Node;
use super::with_value;

#[must_use]
pub struct Handle<V> {
    pub(super) list: Rc<LinkedListImpl<V>>,
    pub(super) node: Rc<Node<V>>,
}

impl<V> Handle<V> {
    pub fn value(&self) -> &V {
        &self.node.value
    }
}

impl<V> Drop for Handle<V> {
    fn drop(&mut self) {
        match (
            with_value(&self.node.prev, |prev| prev.upgrade()),
            with_value(&self.node.next, |next| next.upgrade()),
        ) {
            (Some(ref prev), Some(ref next)) => {
                // prev <-> self <-> next <-> prev
                // prev          <-> next <-> prev
                if with_value(&self.list.node, |node| node.clone())
                    .ptr_eq(&Rc::downgrade(&self.node))
                {
                    self.list.node.set(Rc::downgrade(next));
                }
                prev.next.set(Rc::downgrade(&next));
                next.prev.set(Rc::downgrade(&prev));
            }
            (None, None) => {}
            _ => unreachable!(),
        }
    }
}

impl<V: Clone + std::fmt::Debug> std::fmt::Debug for Handle<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Handle").field(&self.node).finish()
    }
}
