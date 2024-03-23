use std::cell::Cell;

use super::node_factory::NodeFactory;
use super::with_value;

pub struct Node<F: NodeFactory> {
    pub(super) prev: Cell<F::PointerWeak>,
    pub(super) value: F::Value,
    pub(super) next: Cell<F::PointerWeak>,
}

impl<V: std::fmt::Debug, F: NodeFactory<Value = V>> std::fmt::Debug for Node<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("prev", &with_value(&self.prev, F::upgrade).unwrap().value)
            .field("value", &self.value)
            .field("next", &with_value(&self.next, F::upgrade).unwrap().value)
            .finish()
    }
}
