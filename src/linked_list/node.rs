use std::cell::Cell;
use std::rc::Weak;

use super::with_value;

pub(super) struct Node<V> {
    pub prev: Cell<Weak<Node<V>>>,
    pub value: V,
    pub next: Cell<Weak<Node<V>>>,
}

impl<V> From<V> for Node<V> {
    fn from(value: V) -> Self {
        Self {
            prev: Default::default(),
            value: value,
            next: Default::default(),
        }
    }
}

impl<V: Clone + std::fmt::Debug> std::fmt::Debug for Node<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field(
                "prev",
                &with_value(&self.prev, |prev| prev.upgrade().unwrap().value.clone()),
            )
            .field("value", &self.value)
            .field(
                "next",
                &with_value(&self.next, |next| next.upgrade().unwrap().value.clone()),
            )
            .finish()
    }
}
