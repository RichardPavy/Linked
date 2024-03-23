use super::node::NodeFactory;
use super::with_value;

pub(super) struct NodeIterator<F: NodeFactory> {
    pub next: Option<F::StrongPointer>,
    pub stop: Option<F::StrongPointer>,
}

impl<V: Clone, F: NodeFactory<Value = V>> Iterator for NodeIterator<F> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = &self.next else {
            return None;
        };
        let end = self
            .stop
            .as_ref()
            .map(|stop| F::ptr_eq(&F::downgrade(&next), &F::downgrade(&stop)))
            .unwrap_or(true);

        let result = Some(next.value.clone());

        if end {
            self.next = None;
        } else {
            let next = with_value(&next.next, |next| F::upgrade(next));
            self.next = next;
        }

        return result;
    }
}
