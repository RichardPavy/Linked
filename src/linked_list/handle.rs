use std::ops::Deref;
use std::rc::Rc;

use super::implem::LinkedListImpl;
use super::node_factory::NodeFactory;
use super::with_value;

#[must_use]
pub struct Handle<F: NodeFactory> {
    pub(super) list: Rc<LinkedListImpl<F>>,
    pub(super) node: F::PointerStrong,
}

impl<F: NodeFactory> Handle<F> {
    pub fn value(&self) -> &F::Value {
        &self.node.value
    }
}

impl<F: NodeFactory> Drop for Handle<F> {
    fn drop(&mut self) {
        match (
            with_value(&self.node.prev, |prev| F::upgrade(prev)),
            with_value(&self.node.next, |next| F::upgrade(next)),
        ) {
            (Some(ref prev), Some(ref next)) => {
                // prev <-> self <-> next <-> prev
                // prev          <-> next <-> prev
                if F::ptr_eq(
                    &with_value(&self.list.node, |node| node.clone()),
                    &F::downgrade(&self.node),
                ) {
                    self.list.node.set(F::downgrade(next));
                }
                prev.next.set(F::downgrade(&next));
                next.prev.set(F::downgrade(&prev));
            }
            (None, None) => {}
            _ => unreachable!(),
        }
    }
}

impl<V: std::fmt::Debug, F: NodeFactory<Value = V>> std::fmt::Debug for Handle<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Handle").field((&self.node).deref()).finish()
    }
}
