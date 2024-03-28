use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

use super::handle::Handle;
use super::iterator::NodeIterator;
use super::node_factory::NodeFactory;
use super::node_ref::NodeRef;
use super::with_value;

pub(super) struct LinkedListImpl<F: NodeFactory> {
    pub node: Cell<F::Pointer>,
    _phantom: PhantomData<F>,
}

impl<F: NodeFactory> LinkedListImpl<F> {
    pub fn push_back(self: &Rc<Self>, value: F::Value) -> Handle<F> {
        let new_ref = F::of(value);
        let new_ptr = F::downgrade(&new_ref);
        let node_ptr = with_value(&self.node, F::Pointer::clone);
        if let Some(node_ref) = F::to_ref(&node_ptr) {
            // prev     <->     self <-> next <-> prev
            // prev <-> new <-> self <-> next <-> prev
            let prev_ptr = with_value(&node_ref.prev, F::Pointer::clone);
            let prev_ref = F::to_ref(&prev_ptr).unwrap();
            new_ref.prev.set(prev_ptr);
            prev_ref.next.set(new_ptr.clone());
            new_ref.next.set(node_ptr);
            node_ref.prev.set(new_ptr);
        } else {
            self.node.set(new_ptr.clone());
            new_ref.prev.set(new_ptr.clone());
            new_ref.next.set(new_ptr);
        }
        Handle {
            list: self.clone(),
            node: new_ref,
        }
    }

    pub fn prev(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, F::Pointer::clone);
        if let Some(node) = F::to_ref(&node) {
            let prev = with_value(&node.prev, F::Pointer::clone);
            Rc::new(LinkedListImpl {
                node: Cell::new(prev),
                _phantom: PhantomData,
            })
        } else {
            self.clone()
        }
    }

    pub fn next(self: &Rc<Self>) -> Rc<Self> {
        let node = with_value(&self.node, F::Pointer::clone);
        if let Some(node) = F::to_ref(&node) {
            let next = with_value(&node.next, F::Pointer::clone);
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
        let next = with_value(&self.node, F::to_ref);
        let stop = next
            .as_ref()
            .and_then(|next| with_value(&next.prev, F::to_ref));
        NodeIterator::<F> { next, stop }
    }

    pub fn current(&self) -> Option<NodeRef<F>> {
        with_value(&self.node, F::to_ref).map(NodeRef::of)
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
