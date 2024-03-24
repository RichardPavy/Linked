use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

use super::node::Node;

pub trait NodeFactory: Sized {
    type Value;
    type Reference: Clone + Deref<Target = Node<Self>>; // Rc<Node<V>>
    type ReferenceRaw: Clone + Default; // Weak<Node<V>>
    type ReferenceRaw2;
    type Reference2: Deref<Target = Node<Self>>;

    fn of(value: Self::Value) -> Self::Reference;
    fn to_ref(pointer: &Self::ReferenceRaw) -> Option<Self::Reference>;
    fn to_raw(pointer: &Self::Reference) -> Self::ReferenceRaw;
    fn ptr_eq_ref(a: &Self::Reference, b: &Self::Reference) -> bool;
    fn ptr_eq_raw(a: &Self::ReferenceRaw, b: &Self::ReferenceRaw) -> bool;
}

pub struct RcNodeFactory<V>(PhantomData<V>);

impl<V> NodeFactory for RcNodeFactory<V> {
    type Value = V;
    type Reference = Rc<Node<Self>>;
    type ReferenceRaw = Weak<Node<Self>>;
    type ReferenceRaw2 = Self::Reference;
    type Reference2 = Self::Reference;

    fn of(value: Self::Value) -> Self::Reference {
        Rc::new(Node {
            prev: Default::default(),
            value: value,
            next: Default::default(),
        })
    }

    fn to_ref(pointer: &Self::ReferenceRaw) -> Option<Self::Reference> {
        pointer.upgrade()
    }

    fn to_raw(pointer: &Self::Reference) -> Self::ReferenceRaw {
        Rc::downgrade(&pointer)
    }

    fn ptr_eq_ref(a: &Self::Reference, b: &Self::Reference) -> bool {
        Rc::ptr_eq(a, b)
    }

    fn ptr_eq_raw(a: &Self::ReferenceRaw, b: &Self::ReferenceRaw) -> bool {
        Weak::ptr_eq(a, b)
    }
}
