use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

use super::node::Node;

pub trait NodeFactory: Sized {
    type Value;
    type Reference: Clone + Deref<Target = Node<Self>>; // Rc<Node<V>>
    type Pointer: Clone + Default; // Weak<Node<V>>
    type Handle: Deref<Target = Node<Self>>;

    fn of(value: Self::Value) -> Self::Handle;
    fn to_ref(pointer: &Self::Pointer) -> Option<Self::Reference>;
    fn to_ptr(pointer: &Self::Reference) -> Self::Pointer;
    fn downgrade(pointer: &Self::Handle) -> Self::Pointer;
    fn ptr_eq_ref(a: &Self::Reference, b: &Self::Reference) -> bool;
    fn ptr_eq_ptr(a: &Self::Pointer, b: &Self::Pointer) -> bool;
}

pub struct RcNodeFactory<V>(PhantomData<V>);

impl<V> NodeFactory for RcNodeFactory<V> {
    type Value = V;
    type Reference = Rc<Node<Self>>;
    type Pointer = Weak<Node<Self>>;
    type Handle = Self::Reference;

    fn of(value: Self::Value) -> Self::Reference {
        Rc::new(Node {
            prev: Default::default(),
            value: value,
            next: Default::default(),
        })
    }

    fn to_ref(pointer: &Self::Pointer) -> Option<Self::Reference> {
        pointer.upgrade()
    }

    fn to_ptr(pointer: &Self::Reference) -> Self::Pointer {
        Rc::downgrade(&pointer)
    }

    fn downgrade(pointer: &Self::Handle) -> Self::Pointer {
        Rc::downgrade(&pointer)
    }

    fn ptr_eq_ref(a: &Self::Reference, b: &Self::Reference) -> bool {
        Rc::ptr_eq(a, b)
    }

    fn ptr_eq_ptr(a: &Self::Pointer, b: &Self::Pointer) -> bool {
        Weak::ptr_eq(a, b)
    }
}
