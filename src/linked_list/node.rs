use std::cell::Cell;
use std::rc::Weak;

use super::with_value;

pub(super) struct Node<V> {
    pub prev: Cell<Weak<Node<V>>>,
    pub value: Cell<Option<V>>,
    pub next: Cell<Weak<Node<V>>>,
}

impl<V> Default for Node<V> {
    fn default() -> Self {
        Self {
            prev: Default::default(),
            value: Default::default(),
            next: Default::default(),
        }
    }
}

impl<V: Clone + std::fmt::Debug> std::fmt::Debug for Node<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field(
                "prev",
                &with_value(&self.prev, |prev| {
                    with_value(&prev.upgrade().unwrap().value, |value| {
                        value.clone().unwrap()
                    })
                }),
            )
            .field(
                "value",
                &with_value(&self.value, |value| value.clone().unwrap()),
            )
            .field(
                "next",
                &with_value(&self.next, |next| {
                    with_value(&next.upgrade().unwrap().value, |value| {
                        value.clone().unwrap()
                    })
                }),
            )
            .finish()
    }
}
