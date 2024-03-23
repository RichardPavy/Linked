use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

use super::handle::Handle;
use super::iterator::NodeIterator;
use super::node_factory::NodeFactory;
use super::node_ref::NodeRef;
use super::with_value;

pub(super) struct LinkedListImpl<F: NodeFactory> {
    pub node: Cell<<F as NodeFactory>::PointerWeak>,
    _phantom: PhantomData<F>,
}

impl<F: NodeFactory> LinkedListImpl<F> {
    pub fn push_back(self: &Rc<Self>, value: <F as NodeFactory>::Value) -> Handle<F> {
        let new = F::of(value);
        let node = with_value(&self.node, |node| node.clone());
        if let Some(node) = F::upgrade(&node) {
            // prev     <->     self <-> next <-> prev
            // prev <-> new <-> self <-> next <-> prev
            let prev = with_value(&node.prev, |prev| F::upgrade(prev)).unwrap();
            new.prev.set(F::downgrade(&prev));
            prev.next.set(F::downgrade(&new));
            new.next.set(F::downgrade(&node));
            node.prev.set(F::downgrade(&new));
        } else {
            self.node.set(F::downgrade(&new));
            new.prev.set(F::downgrade(&new));
            new.next.set(F::downgrade(&new));
        }
        Handle {
            list: self.clone(),
            node: new,
        }
    }

    pub fn prev(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, |node| node.clone());
        if let Some(node) = F::upgrade(&node) {
            let prev = with_value(&node.prev, |prev| prev.clone());
            Rc::new(LinkedListImpl {
                node: Cell::new(prev),
                _phantom: PhantomData,
            })
        } else {
            self.clone()
        }
    }

    pub fn next(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, |node| node.clone());
        if let Some(node) = F::upgrade(&node) {
            let next = with_value(&node.next, |next| next.clone());
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
        let next = with_value(&self.node, |node| F::upgrade(node));
        let stop = next
            .as_ref()
            .and_then(|next| with_value(&next.prev, |prev| F::upgrade(prev)));
        NodeIterator::<F> { next, stop }
    }

    pub fn current(&self) -> Option<NodeRef<F>> {
        with_value(&self.node, |node| F::upgrade(node).map(NodeRef::of))
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
