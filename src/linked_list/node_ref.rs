use std::ops::Deref;

use super::node_factory::NodeFactory;

pub struct NodeRef<F: NodeFactory>(F::PointerStrong);

impl<F: NodeFactory> NodeRef<F> {
    pub fn of(value: F::PointerStrong) -> Self {
        Self(value)
    }
}

impl<F: NodeFactory> Deref for NodeRef<F> {
    type Target = F::Value;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<V: std::fmt::Debug, F: NodeFactory<Value = V>> std::fmt::Debug for NodeRef<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<F: NodeFactory> AsRef<F::Value> for NodeRef<F> {
    fn as_ref(&self) -> &F::Value {
        &self.0.value
    }
}
