use std::cell::Cell;
use std::rc::Rc;

use scopeguard::guard;

pub use self::handle::Handle;
use self::implem::LinkedListImpl;
pub use self::node_factory::RcNodeFactory;
use self::node_ref::NodeRef;

mod handle;
mod implem;
mod iterator;
mod node;
mod node_factory;
mod node_ref;

pub struct LinkedList<V> {
    list: Rc<LinkedListImpl<RcNodeFactory<V>>>,
}

impl<V> LinkedList<V> {
    pub fn new() -> Self {
        Self {
            list: Rc::new(LinkedListImpl::default()),
        }
    }
}

impl<V> LinkedList<V> {
    pub fn push_back(&self, value: V) -> Handle<RcNodeFactory<V>> {
        self.list.push_back(value)
    }

    pub fn prev(&self) -> Self {
        LinkedList {
            list: self.list.prev(),
        }
    }

    pub fn next(&self) -> Self {
        LinkedList {
            list: self.list.next(),
        }
    }
}

impl<V> LinkedList<V> {
    pub fn iter(&self) -> impl Iterator<Item = NodeRef<RcNodeFactory<V>>> {
        self.list.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = V>
    where
        V: Clone,
    {
        self.list.iter().map(|v| v.clone())
    }

    pub fn current(&self) -> Option<NodeRef<RcNodeFactory<V>>> {
        self.list.current()
    }
}

impl<V: std::fmt::Debug> std::fmt::Debug for LinkedList<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<V> Default for LinkedList<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Clone for LinkedList<V> {
    fn clone(&self) -> Self {
        Self {
            list: Rc::clone(&self.list),
        }
    }
}

impl<V: PartialEq> PartialEq for LinkedList<V> {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.iter();
        let mut b = other.iter();
        loop {
            match (a.next(), b.next()) {
                (None, None) => return true,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(a), Some(b)) => {
                    if V::ne(&a, &b) {
                        return false;
                    }
                }
            }
        }
    }
}

impl<V: Clone + Eq> Eq for LinkedList<V> {}

fn with_value<T: Default, R, F: Fn(&T) -> R>(cell: &Cell<T>, f: F) -> R {
    let value = guard(cell.take(), move |v| cell.set(v));
    f(&value)
}

#[cfg(test)]
mod tests {
    use tests::node_ref::NodeValueRefOption;

    use super::*;

    #[test]
    fn push() {
        let list = LinkedList::<String>::new();
        assert_eq!(Vec::<String>::default(), list.values().collect::<Vec<_>>());

        let _a = list.push_back("a".into());
        assert_eq!(vec!["a"], list.values().collect::<Vec<_>>());

        let _b = list.push_back("b".into());
        assert_eq!(vec!["a", "b"], list.values().collect::<Vec<_>>());

        let _c = list.push_back("c".into());
        assert_eq!(vec!["a", "b", "c"], list.values().collect::<Vec<_>>());
    }

    #[test]
    fn forward() {
        let list = LinkedList::<String>::new();
        let a = list.push_back("a".into());
        let b = list.push_back("b".into());
        let c = list.push_back("c".into());

        drop(a);
        assert_eq!(vec!["b", "c"], list.values().collect::<Vec<_>>());

        drop(b);
        assert_eq!(vec!["c"], list.values().collect::<Vec<_>>());

        drop(c);
        assert_eq!(Vec::<String>::new(), list.values().collect::<Vec<_>>());
    }

    #[test]
    fn backward() {
        let list = LinkedList::<String>::new();
        let a = list.push_back("a".into());
        let b = list.push_back("b".into());
        let c = list.push_back("c".into());

        drop(c);
        assert_eq!(vec!["a", "b"], list.values().collect::<Vec<_>>());

        drop(b);
        assert_eq!(vec!["a"], list.values().collect::<Vec<_>>());

        drop(a);
        assert_eq!(Vec::<String>::default(), list.values().collect::<Vec<_>>());
    }

    #[test]
    fn middle1() {
        let list = LinkedList::<String>::new();
        let a = list.push_back("a".into());
        let b = list.push_back("b".into());
        let c = list.push_back("c".into());

        drop(b);
        assert_eq!(vec!["a", "c"], list.values().collect::<Vec<_>>());

        drop(c);
        assert_eq!(vec!["a"], list.values().collect::<Vec<_>>());

        drop(a);
        assert_eq!(Vec::<String>::default(), list.values().collect::<Vec<_>>());
    }

    #[test]
    fn middle2() {
        let list = LinkedList::<String>::new();
        let a = list.push_back("a".into());
        let b = list.push_back("b".into());
        let c = list.push_back("c".into());

        drop(b);
        assert_eq!(vec!["a", "c"], list.values().collect::<Vec<_>>());

        drop(a);
        assert_eq!(vec!["c"], list.values().collect::<Vec<_>>());

        drop(c);
        assert_eq!(Vec::<String>::default(), list.values().collect::<Vec<_>>());
    }

    #[test]
    fn equality() {
        let list1: LinkedList<String> = LinkedList::<String>::new();
        let a1 = list1.push_back("a".into());
        let b1 = list1.push_back("b".into());
        let c1 = list1.push_back("c".into());

        let list2: LinkedList<String> = LinkedList::<String>::new();
        let a2 = list2.push_back("a".into());
        let b2 = list2.push_back("b".into());
        let c2 = list2.push_back("c".into());

        assert_eq!(list1, list2);

        drop(c1);
        assert_ne!(list1, list2);

        drop(c2);
        assert_eq!(list1, list2);

        let c1 = list1.push_back("c".into());
        let c2 = list2.push_back("c".into());
        assert_eq!(list1, list2);

        drop(c1);
        drop(b2);
        assert_ne!(list1, list2);

        drop(a1);
        drop(b1);
        drop(a2);
        drop(c2);
        assert_eq!(list1, list2);
    }

    #[test]
    fn drop_struct() {
        use std::sync::atomic::AtomicI32;
        use std::sync::atomic::Ordering::SeqCst;

        static COUNT: AtomicI32 = AtomicI32::new(0);

        struct DropStruct {
            name: &'static str,
        }

        impl DropStruct {
            fn new(name: &'static str) -> Self {
                COUNT.fetch_add(1, SeqCst);
                Self { name }
            }
        }

        impl Drop for DropStruct {
            fn drop(&mut self) {
                COUNT.fetch_sub(1, SeqCst);
            }
        }

        let list = LinkedList::<Rc<DropStruct>>::new();
        let to_vec = {
            let list = list.clone();
            move || list.iter().map(|x| x.name).collect::<Vec<_>>()
        };
        let a = list.push_back(DropStruct::new("a").into());
        assert_eq!(1, COUNT.load(SeqCst));
        assert_eq!(vec!["a"], to_vec());

        drop(a);
        assert_eq!(0, COUNT.load(SeqCst));
        assert_eq!(Vec::<String>::default(), to_vec());

        let b = list.push_back(DropStruct::new("b").into());
        assert_eq!(1, COUNT.load(SeqCst));
        assert_eq!(vec!["b"], to_vec());
        let c = list.push_back(DropStruct::new("c").into());
        assert_eq!(2, COUNT.load(SeqCst));
        assert_eq!(vec!["b", "c"], to_vec());
        let d = list.push_back(DropStruct::new("d").into());
        assert_eq!(3, COUNT.load(SeqCst));
        assert_eq!(vec!["b", "c", "d"], to_vec());

        drop(d);
        assert_eq!(2, COUNT.load(SeqCst));
        assert_eq!(vec!["b", "c"], to_vec());
        drop(b);
        assert_eq!(1, COUNT.load(SeqCst));
        assert_eq!(vec!["c"], to_vec());
        drop(c);
        assert_eq!(0, COUNT.load(SeqCst));
        assert_eq!(Vec::<String>::default(), to_vec());
    }

    #[test]
    fn debug() {
        let list = LinkedList::<String>::new();

        let a = list.push_back("a".into());
        assert_eq!(
            r#"Handle(Node { prev: "a", value: "a", next: "a" })"#,
            format!("{a:?}")
        );

        let b = list.push_back("b".into());
        assert_eq!(
            r#"Handle(Node { prev: "a", value: "b", next: "a" })"#,
            format!("{b:?}")
        );

        let c = list.push_back("c".into());
        let _d = list.push_back("d".into());
        assert_eq!(
            r#"Handle(Node { prev: "b", value: "c", next: "d" })"#,
            format!("{c:?}")
        );

        assert_eq!(r#"["a", "b", "c", "d"]"#, format!("{list:?}"));
    }

    #[test]
    fn prev() {
        let list = LinkedList::<String>::new();
        let _a = list.push_back("a".into());
        let _b = list.push_back("b".into());
        let _c = list.push_back("c".into());
        let _d = list.push_back("d".into());

        assert_eq!(r#"["d", "a", "b", "c"]"#, format!("{:?}", list.prev()));
    }

    #[test]
    fn next() {
        let list = LinkedList::<String>::new();
        let _a = list.push_back("a".into());
        let _b = list.push_back("b".into());
        let _c = list.push_back("c".into());
        let _d = list.push_back("d".into());

        assert_eq!(r#"["b", "c", "d", "a"]"#, format!("{:?}", list.next()));
    }

    #[test]
    fn current() {
        let list = LinkedList::<String>::new();
        let _a = list.push_back("a".into());
        let _b = list.push_back("b".into());
        let _c = list.push_back("c".into());
        let _d = list.push_back("d".into());

        assert_eq!(Some(&"a".to_string()), list.current().as_ref().map_ref());
        assert_eq!(
            Some(&"d".to_string()),
            list.prev().current().as_ref().map_ref()
        );
        assert_eq!(
            Some(&"b".to_string()),
            list.next().current().as_ref().map_ref()
        );
    }
}
