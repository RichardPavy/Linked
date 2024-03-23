use super::node_factory::NodeFactory;
use super::node_ref::NodeRef;
use super::with_value;

pub(super) struct NodeIterator<F: NodeFactory> {
    pub next: Option<F::PointerStrong>,
    pub stop: Option<F::PointerStrong>,
}

impl<F: NodeFactory> Iterator for NodeIterator<F> {
    type Item = NodeRef<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = &self.next else {
            return None;
        };
        let end = self
            .stop
            .as_ref()
            .map(|stop| F::ptr_eq(&F::downgrade(&next), &F::downgrade(&stop)))
            .unwrap_or(true);

        let result = Some(NodeRef::of(next.clone()));

        if end {
            self.next = None;
        } else {
            let next = with_value(&next.next, |next| F::upgrade(next));
            self.next = next;
        }

        return result;
    }
}
