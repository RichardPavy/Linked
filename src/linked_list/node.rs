use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

use super::with_value;

pub trait NodeFactory: Sized {
    type Value;
    type PointerStrong: Clone + Deref<Target = Node<Self>>; // Rc<Node<V>>
    type PointerWeak: Clone + Default; // Weak<Node<V>>
    type ReferenceRaw;
    type Reference: Deref<Target = Node<Self>>;

    fn of(value: Self::Value) -> Self::PointerStrong;
    fn upgrade(pointer: &Self::PointerWeak) -> Option<Self::PointerStrong>;
    fn downgrade(pointer: &Self::PointerStrong) -> Self::PointerWeak;
    fn ptr_eq(a: &Self::PointerWeak, b: &Self::PointerWeak) -> bool;
}

pub struct RcNodeFactory<V>(PhantomData<V>);

impl<V> NodeFactory for RcNodeFactory<V> {
    type Value = V;
    type PointerStrong = Rc<Node<Self>>;
    type PointerWeak = Weak<Node<Self>>;
    type ReferenceRaw = Self::PointerStrong;
    type Reference = Self::PointerStrong;

    fn of(value: Self::Value) -> Self::PointerStrong {
        Rc::new(Node {
            prev: Default::default(),
            value: value,
            next: Default::default(),
        })
    }

    fn upgrade(pointer: &Self::PointerWeak) -> Option<Self::PointerStrong> {
        pointer.upgrade()
    }

    fn downgrade(pointer: &Self::PointerStrong) -> Self::PointerWeak {
        Rc::downgrade(&pointer)
    }

    fn ptr_eq(a: &Self::PointerWeak, b: &Self::PointerWeak) -> bool {
        Weak::ptr_eq(a, b)
    }
}

pub struct Node<F: NodeFactory> {
    pub(super) prev: Cell<F::PointerWeak>,
    pub(super) value: F::Value,
    pub(super) next: Cell<F::PointerWeak>,
}

impl<V: std::fmt::Debug, F: NodeFactory<Value = V>> std::fmt::Debug for Node<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field(
                "prev",
                &with_value(&self.prev, |prev| F::upgrade(&prev).clone())
                    .unwrap()
                    .value,
            )
            .field("value", &self.value)
            .field(
                "next",
                &with_value(&self.next, |next| F::upgrade(&next).clone())
                    .unwrap()
                    .value,
            )
            .finish()
    }
}

pub struct NodeValueRef<F: NodeFactory>(F::PointerStrong);

impl<F: NodeFactory> NodeValueRef<F> {
    pub fn of(value: F::PointerStrong) -> Self {
        Self(value)
    }
}

impl<F: NodeFactory> Deref for NodeValueRef<F> {
    type Target = F::Value;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<V: std::fmt::Debug, F: NodeFactory<Value = V>> std::fmt::Debug for NodeValueRef<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<F: NodeFactory> AsRef<F::Value> for NodeValueRef<F> {
    fn as_ref(&self) -> &F::Value {
        &self.0.value
    }
}

pub trait NodeValueRefOption<V> {
    fn map_ref(&self) -> Option<&V>;
}

impl<F: NodeFactory> NodeValueRefOption<F::Value> for Option<&NodeValueRef<F>> {
    fn map_ref(&self) -> Option<&F::Value> {
        self.map(|n| n.as_ref())
    }
}
