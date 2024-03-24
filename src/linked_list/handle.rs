use std::ops::Deref;
use std::rc::Rc;

use super::implem::LinkedListImpl;
use super::node_factory::NodeFactory;
use super::with_value;

#[must_use]
pub struct Handle<F: NodeFactory> {
    pub(super) list: Rc<LinkedListImpl<F>>,
    pub(super) node: F::Reference,
}

impl<F: NodeFactory> Handle<F> {
    pub fn value(&self) -> &F::Value {
        &self.node.value
    }
}

impl<F: NodeFactory> Drop for Handle<F> {
    fn drop(&mut self) {
        let prev_raw = with_value(&self.node.prev, F::ReferenceRaw::clone);
        let next_raw = with_value(&self.node.next, F::ReferenceRaw::clone);
        match (F::to_ref(&prev_raw), F::to_ref(&next_raw)) {
            (Some(prev_ref), Some(next_ref)) => {
                // prev <-> self <-> next <-> prev
                // prev          <-> next <-> prev
                if F::ptr_eq_raw(
                    &with_value(&self.list.node, F::ReferenceRaw::clone),
                    &F::to_raw(&self.node),
                ) {
                    self.list.node.set(next_raw.clone());
                }
                prev_ref.next.set(next_raw);
                next_ref.prev.set(prev_raw);
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
