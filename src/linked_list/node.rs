use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

use super::with_value;

pub trait NodeFactory: Sized {
    type Value;
    type StrongPointer: Deref<Target = Node<Self>>; // Rc<Node<V>>
    type WeakPointer: Clone + Default; // Weak<Node<V>>

    fn of(value: Self::Value) -> Self::StrongPointer;
    fn upgrade(pointer: &Self::WeakPointer) -> Option<Self::StrongPointer>;
    fn downgrade(pointer: &Self::StrongPointer) -> Self::WeakPointer;
    fn ptr_eq(a: &Self::WeakPointer, b: &Self::WeakPointer) -> bool;
}

pub struct RcNodeFactory<V>(PhantomData<V>);

impl<V> NodeFactory for RcNodeFactory<V> {
    type Value = V;
    type StrongPointer = Rc<Node<Self>>;
    type WeakPointer = Weak<Node<Self>>;

    fn of(value: Self::Value) -> Self::StrongPointer {
        Rc::new(Node {
            prev: Default::default(),
            value: value,
            next: Default::default(),
        })
    }

    fn upgrade(pointer: &Self::WeakPointer) -> Option<Self::StrongPointer> {
        pointer.upgrade()
    }

    fn downgrade(pointer: &Self::StrongPointer) -> Self::WeakPointer {
        Rc::downgrade(&pointer)
    }

    fn ptr_eq(a: &Self::WeakPointer, b: &Self::WeakPointer) -> bool {
        Weak::ptr_eq(a, b)
    }
}

pub struct Node<F: NodeFactory> {
    pub prev: Cell<F::WeakPointer>,
    pub value: F::Value,
    pub next: Cell<F::WeakPointer>,
}

impl<V: Clone + std::fmt::Debug, F: NodeFactory<Value = V>> std::fmt::Debug for Node<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field(
                "prev",
                &with_value(&self.prev, |prev| F::upgrade(&prev).unwrap().value.clone()),
            )
            .field("value", &self.value)
            .field(
                "next",
                &with_value(&self.next, |next| F::upgrade(&next).unwrap().value.clone()),
            )
            .finish()
    }
}
