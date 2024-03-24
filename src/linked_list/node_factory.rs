use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

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
pub struct RawRef<V>(*const Node<RcNodeFactory<V>>);

impl<V> Clone for RawRef<V> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<V> Deref for RawRef<V> {
    type Target = Node<RcNodeFactory<V>>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<V> NodeFactory for RcNodeFactory<V> {
    type Value = V;
    type Reference = RawRef<V>;
    type Pointer = Option<*const Node<Self>>;
    type Handle = Rc<Node<Self>>;

    fn of(value: Self::Value) -> Self::Handle {
        Rc::new(Node {
            prev: Default::default(),
            value: value,
            next: Default::default(),
        })
    }

    fn to_ref(pointer: &Self::Pointer) -> Option<Self::Reference> {
        pointer.map(|p| RawRef(p))
    }

    fn to_ptr(pointer: &Self::Reference) -> Self::Pointer {
        if pointer.0.is_null() {
            None
        } else {
            Some(pointer.0)
        }
    }

    fn downgrade(pointer: &Self::Handle) -> Self::Pointer {
        Some(pointer.as_ref())
    }

    fn ptr_eq_ref(a: &Self::Reference, b: &Self::Reference) -> bool {
        a.0 == b.0
    }

    fn ptr_eq_ptr(a: &Self::Pointer, b: &Self::Pointer) -> bool {
        a == b
    }
}
