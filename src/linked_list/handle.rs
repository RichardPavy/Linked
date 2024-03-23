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
        let prev_weak = with_value(&self.node.prev, F::PointerWeak::clone);
        let next_weak = with_value(&self.node.next, F::PointerWeak::clone);
        match (F::upgrade(&prev_weak), F::upgrade(&next_weak)) {
            (Some(prev_strong), Some(next_strong)) => {
                // prev <-> self <-> next <-> prev
                // prev          <-> next <-> prev
                if F::ptr_eq_weak(
                    &with_value(&self.list.node, F::PointerWeak::clone),
                    &F::downgrade(&self.node),
                ) {
                    self.list.node.set(next_weak.clone());
                }
                prev_strong.next.set(next_weak);
                next_strong.prev.set(prev_weak);
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
