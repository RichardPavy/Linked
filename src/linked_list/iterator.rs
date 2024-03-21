use std::rc::Rc;

use super::node::Node;
use super::with_value;

pub(super) struct NodeIterator<T> {
    pub next: Option<Rc<Node<T>>>,
    pub stop: Option<Rc<Node<T>>>,
}

impl<V: Clone> Iterator for NodeIterator<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = &self.next else {
            return None;
        };
        let end = self
            .stop
            .as_ref()
            .map(|stop| Rc::ptr_eq(next, stop))
            .unwrap_or(true);

        let result = Some(next.value.clone());

        if end {
            self.next = None;
        } else {
            let next = with_value(&next.next, |next| next.upgrade());
            self.next = next;
        }

        return result;
    }
}
