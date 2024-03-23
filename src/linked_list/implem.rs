use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

use super::handle::Handle;
use super::iterator::NodeIterator;
use super::node_factory::NodeFactory;
use super::node_ref::NodeRef;
use super::with_value;

pub(super) struct LinkedListImpl<F: NodeFactory> {
    pub node: Cell<F::PointerWeak>,
    _phantom: PhantomData<F>,
}

impl<F: NodeFactory> LinkedListImpl<F> {
    pub fn push_back(self: &Rc<Self>, value: F::Value) -> Handle<F> {
        let new_strong = F::of(value);
        let new_weak = F::downgrade(&new_strong);
        let node_weak = with_value(&self.node, F::PointerWeak::clone);
        if let Some(node_strong) = F::upgrade(&node_weak) {
            // prev     <->     self <-> next <-> prev
            // prev <-> new <-> self <-> next <-> prev
            let prev_weak = with_value(&node_strong.prev, F::PointerWeak::clone);
            let prev_strong = F::upgrade(&prev_weak).unwrap();
            new_strong.prev.set(prev_weak);
            prev_strong.next.set(new_weak.clone());
            new_strong.next.set(node_weak);
            node_strong.prev.set(new_weak);
        } else {
            self.node.set(new_weak.clone());
            new_strong.prev.set(new_weak.clone());
            new_strong.next.set(new_weak);
        }
        Handle {
            list: self.clone(),
            node: new_strong,
        }
    }

    pub fn prev(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, F::PointerWeak::clone);
        if let Some(node) = F::upgrade(&node) {
            let prev = with_value(&node.prev, F::PointerWeak::clone);
            Rc::new(LinkedListImpl {
                node: Cell::new(prev),
                _phantom: PhantomData,
            })
        } else {
            self.clone()
        }
    }

    pub fn next(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, F::PointerWeak::clone);
        if let Some(node) = F::upgrade(&node) {
            let next = with_value(&node.next, F::PointerWeak::clone);
            Rc::new(LinkedListImpl {
                node: Cell::new(next),
                _phantom: PhantomData,
            })
        } else {
            self.clone()
        }
    }
}

impl<F: NodeFactory> LinkedListImpl<F> {
    pub fn iter(&self) -> impl Iterator<Item = NodeRef<F>> {
        let next = with_value(&self.node, F::upgrade);
        let stop = next
            .as_ref()
            .and_then(|next| with_value(&next.prev, F::upgrade));
        NodeIterator::<F> { next, stop }
    }

    pub fn current(&self) -> Option<NodeRef<F>> {
        with_value(&self.node, F::upgrade).map(NodeRef::of)
    }
}

impl<F: NodeFactory> Default for LinkedListImpl<F> {
    fn default() -> Self {
        Self {
            node: Default::default(),
            _phantom: PhantomData,
        }
    }
}
