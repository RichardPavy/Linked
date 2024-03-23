use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

use super::node::Node;

pub trait NodeFactory: Sized {
    type Value;
    type PointerStrong: Clone + Deref<Target = Node<Self>>; // Rc<Node<V>>
    type PointerWeak: Clone + Default; // Weak<Node<V>>
    type ReferenceRaw;
    type Reference: Deref<Target = Node<Self>>;

    fn of(value: Self::Value) -> Self::PointerStrong;
    fn upgrade(pointer: &Self::PointerWeak) -> Option<Self::PointerStrong>;
    fn downgrade(pointer: &Self::PointerStrong) -> Self::PointerWeak;
    fn ptr_eq_strong(a: &Self::PointerStrong, b: &Self::PointerStrong) -> bool;
    fn ptr_eq_weak(a: &Self::PointerWeak, b: &Self::PointerWeak) -> bool;
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

    fn ptr_eq_strong(a: &Self::PointerStrong, b: &Self::PointerStrong) -> bool {
        Rc::ptr_eq(a, b)
    }

    fn ptr_eq_weak(a: &Self::PointerWeak, b: &Self::PointerWeak) -> bool {
        Weak::ptr_eq(a, b)
    }
}
