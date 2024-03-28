use std::ops::Deref;
use std::rc::Rc;

use super::implem::LinkedListImpl;
use super::node_factory::NodeFactory;
use super::with_value;

#[must_use]
pub struct Handle<F: NodeFactory> {
    pub(super) list: Rc<LinkedListImpl<F>>,
    pub(super) node: F::Handle,
}

impl<F: NodeFactory> Handle<F> {
    pub fn value(&self) -> &F::Value {
        &self.node.value
    }
}

impl<F: NodeFactory> Drop for Handle<F> {
    fn drop(&mut self) {
        let prev_ptr = with_value(&self.node.prev, F::Pointer::clone);
        let node_ptr = F::downgrade(&self.node);
        if F::ptr_eq_ptr(&prev_ptr, &node_ptr) {
            self.list.node.set(F::Pointer::default());
            return;
        }
        let next_ptr = with_value(&self.node.next, F::Pointer::clone);
        match (F::to_ref(&prev_ptr), F::to_ref(&next_ptr)) {
            (Some(prev_ref), Some(next_ref)) => {
                // prev <-> self <-> next <-> prev
                // prev          <-> next <-> prev
                if F::ptr_eq_ptr(&with_value(&self.list.node, F::Pointer::clone), &node_ptr) {
                    self.list.node.set(next_ptr.clone());
                }
                prev_ref.next.set(next_ptr);
                next_ref.prev.set(prev_ptr);
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
