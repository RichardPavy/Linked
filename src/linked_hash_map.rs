use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use super::linked_list;
use super::linked_list::LinkedList;
use crate::linked_list::RcNodeFactory;

pub mod collectible;
pub mod serializable;

pub struct LinkedHashMap<K, V>
where
    K: Clone + Eq + Hash,
{
    keys: LinkedList<K>,
    map: Rc<RefCell<HashMap<K, LinkedHashMapValue<K, V>>>>,
}

pub struct LinkedHashSet<K>
where
    K: Clone + Eq + Hash,
{
    linked_hash_map: LinkedHashMap<K, ()>,
}

struct LinkedHashMapValue<K, V>
where
    K: Clone + Eq + Hash,
{
    value: V,
    handle: std::rc::Weak<HandleImpl<K, V>>,
}

#[derive(Clone)]
pub struct Handle<K, V>(Rc<HandleImpl<K, V>>)
where
    K: Clone + Eq + Hash;

struct HandleImpl<K, V>
where
    K: Clone + Eq + Hash,
{
    linked_hash_map: LinkedHashMap<K, V>,
    key_handle: RefCell<Option<linked_list::Handle<RcNodeFactory<K>>>>,
}

impl<K, V> Drop for HandleImpl<K, V>
where
    K: Clone + Eq + Hash,
{
    fn drop(&mut self) {
        static EXPECT_MSG: &'static str =
            "Handles can never have an empty value. Using Option<V> so they are Default";
        let key = self.key_handle.take().map(|k| k.value().clone());
        let key = key.expect(EXPECT_MSG);
        let removed = self.linked_hash_map.map.borrow_mut().remove(&key);
        removed.expect(EXPECT_MSG);
    }
}

#[must_use]
pub struct InsertResult<K, V, P>
where
    K: Clone + Eq + Hash,
{
    pub previous: P,
    pub handle: Handle<K, V>,
}

impl<K, V, P> InsertResult<K, V, P>
where
    K: Clone + Eq + Hash,
{
    fn map<P2>(self, f: impl FnOnce(P) -> P2) -> InsertResult<K, V, P2> {
        InsertResult {
            previous: f(self.previous),
            handle: self.handle,
        }
    }
}

impl<K, V> LinkedHashMap<K, V>
where
    K: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            keys: Default::default(),
            map: Default::default(),
        }
    }

    pub fn insert(&self, key: K, value: V) -> InsertResult<K, V, Option<V>> {
        match self.map.borrow_mut().entry(key.clone()) {
            hash_map::Entry::Occupied(mut entry) => {
                let old = entry.get_mut();
                let key_handle = self.keys.push_back(key);
                let previous = std::mem::replace(&mut old.value, value);
                let handle = old.handle.upgrade().unwrap();
                *handle.key_handle.borrow_mut() = Some(key_handle);
                InsertResult {
                    previous: Some(previous),
                    handle: Handle(handle),
                }
            }
            hash_map::Entry::Vacant(entry) => {
                let key_handle = self.keys.push_back(key);
                let handle = Rc::new(HandleImpl {
                    linked_hash_map: self.clone(),
                    key_handle: RefCell::new(Some(key_handle)),
                });
                let value = LinkedHashMapValue {
                    value,
                    handle: Rc::downgrade(&handle),
                };
                entry.insert(value);
                InsertResult {
                    previous: None,
                    handle: Handle(handle),
                }
            }
        }
    }

    fn borrow_map(&self) -> std::cell::Ref<HashMap<K, LinkedHashMapValue<K, V>>> {
        RefCell::borrow(&self.map)
    }
}

impl<K, V> LinkedHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    pub fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.borrow_map().get(key).map(|entry| entry.value.clone())
    }

    pub fn keys(&self) -> impl Iterator<Item = K> {
        self.keys.iter().map(|x| (*x).clone())
    }

    pub fn values(&self) -> impl Iterator<Item = V> + '_ {
        let keys = self.keys();
        keys.map(|key| self.get(&key).unwrap())
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> + '_ {
        self.keys().map(|key| {
            let value = self.get(&key).unwrap();
            (key, value)
        })
    }
}

impl<K, V> Default for LinkedHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> LinkedHashSet<K>
where
    K: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            linked_hash_map: LinkedHashMap::new(),
        }
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.linked_hash_map.borrow_map().contains_key(key)
    }

    pub fn insert(&self, key: K) -> InsertResult<K, (), bool> {
        self.linked_hash_map.insert(key, ()).map(|p| p.is_some())
    }

    pub fn iter(&self) -> impl Iterator<Item = K> {
        self.linked_hash_map.keys()
    }
}

impl<K> Default for LinkedHashSet<K>
where
    K: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self {
            linked_hash_map: LinkedHashMap::default(),
        }
    }
}

impl<K, V> std::fmt::Debug for LinkedHashMap<K, V>
where
    K: std::fmt::Debug + Clone + Eq + Hash,
    V: std::fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K> std::fmt::Debug for LinkedHashSet<K>
where
    K: std::fmt::Debug + Clone + Eq + Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<K, V> Clone for LinkedHashMap<K, V>
where
    K: Clone + Eq + Hash,
{
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            map: self.map.clone(),
        }
    }
}

impl<K> Clone for LinkedHashSet<K>
where
    K: Clone + Eq + Hash,
{
    fn clone(&self) -> Self {
        Self {
            linked_hash_map: self.linked_hash_map.clone(),
        }
    }
}

impl<K> PartialEq for LinkedHashSet<K>
where
    K: Clone + Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        if self.linked_hash_map.borrow_map().len() != other.linked_hash_map.borrow_map().len() {
            return false;
        }

        for (a, b) in self.iter().zip(other.iter()) {
            if a != b {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {

    use super::collectible::key::CollectibleKey;
    use super::collectible::value::CollectibleValue;
    use super::LinkedHashMap;
    use super::LinkedHashSet;

    #[test]
    fn linked_hash_map() {
        let map: LinkedHashMap<String, i64> = LinkedHashMap::default();
        let a = map.insert("a".into(), 123);
        assert_eq!(None, a.previous);
        let b = map.insert("b".into(), 456);
        assert_eq!(None, b.previous);
        let c = map.insert("c".into(), 789);
        assert_eq!(None, c.previous);

        assert_eq!(
            vec![
                ("a".to_string(), 123),
                ("b".to_string(), 456),
                ("c".to_string(), 789),
            ],
            map.iter().collect::<Vec<_>>()
        );

        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            map.keys().collect::<Vec<_>>()
        );

        assert_eq!(vec![123, 456, 789], map.values().collect::<Vec<_>>());

        assert_eq!(Some(123), map.insert("a".into(), 124).previous);
        assert_eq!(
            vec![
                ("b".to_string(), 456),
                ("c".to_string(), 789),
                ("a".to_string(), 124),
            ],
            map.iter().collect::<Vec<_>>()
        );

        assert!(matches!(map.get("c"), Some(789)));
        drop(c);
        assert!(matches!(map.get("c"), None));
        assert_eq!(
            vec![("b".to_string(), 456), ("a".to_string(), 124)],
            map.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn linked_hash_set() {
        let set: LinkedHashSet<String> = LinkedHashSet::new();
        let a = set.insert("a".into());
        assert!(!a.previous);
        let b = set.insert("b".into());
        assert!(!b.previous);
        let c = set.insert("c".into());
        assert!(!c.previous);
        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            set.iter().collect::<Vec<_>>()
        );

        assert!(set.insert("a".into()).previous);
        assert_eq!(
            vec!["b".to_string(), "c".to_string(), "a".to_string()],
            set.iter().collect::<Vec<_>>()
        );

        drop(c);
        assert_eq!(
            vec!["b".to_string(), "a".to_string()],
            set.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn collect_set() {
        let set = [
            CollectibleKey::from("a".to_string()),
            "b".to_string().into(),
            "c".to_string().into(),
        ]
        .into_iter()
        .collect::<LinkedHashSet<_>>();
        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            set.iter().map(|k| k.to_string()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn collect_map() {
        let map = [
            ("a".to_string(), CollectibleValue::from(1)),
            ("b".to_string(), 2.into()),
            ("c".to_string(), 3.into()),
        ]
        .into_iter()
        .collect::<LinkedHashMap<_, _>>();
        assert_eq!(
            vec![
                ("a".to_string(), 1),
                ("b".to_string(), 2),
                ("c".to_string(), 3),
            ],
            map.iter().map(|(k, v)| (k, *v)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn get() {
        let map = [
            ("a".to_string(), CollectibleValue::from(1)),
            ("b".to_string(), 2.into()),
            ("c".to_string(), 3.into()),
        ]
        .into_iter()
        .collect::<LinkedHashMap<_, _>>();
        assert_eq!(Some(1), map.get("a").map(|v| *v));
        assert_eq!(None, map.get("aa").map(|v| *v));
    }

    #[test]
    fn contains() {
        let set = LinkedHashSet::<String>::new();
        let _a = set.insert("a".into()).handle;
        let b = set.insert("b".into()).handle;
        let _c = set.insert("c".into()).handle;
        assert_eq!(true, set.contains("a"));
        assert_eq!(true, set.contains("b"));
        assert_eq!(true, set.contains("c"));
        drop(b);
        assert_eq!(true, set.contains("a"));
        assert_eq!(false, set.contains("b"));
        assert_eq!(true, set.contains("c"));
        assert_eq!(false, set.contains("aa"));
    }

    #[test]
    fn serde_map() {
        let map: LinkedHashMap<String, CollectibleValue<String, i32>> =
            serde_json::from_str(r#"[["a",1],["b",2],["c",3]]"#).unwrap();
        assert_eq!(
            r#"[["a",1],["b",2],["c",3]]"#,
            serde_json::to_string(&map).unwrap()
        );
    }

    #[test]
    fn serde_set() {
        let set: LinkedHashSet<CollectibleKey<String>> =
            serde_json::from_str(r#"["a","b","c"]"#).unwrap();
        assert_eq!(r#"["a","b","c"]"#, serde_json::to_string(&set).unwrap());
    }

    #[test]
    fn eq() {
        let s1: LinkedHashSet<CollectibleKey<String>> =
            serde_json::from_str(r#"["a","b","c"]"#).unwrap();
        let s2: LinkedHashSet<CollectibleKey<String>> =
            serde_json::from_str(r#"["a","b","c"]"#).unwrap();
        assert_eq!(r#"["a","b","c"]"#, serde_json::to_string(&s1).unwrap());
        assert_eq!(r#"["a","b","c"]"#, serde_json::to_string(&s2).unwrap());
        assert_eq!(s1, s2);
        let _a = s1.insert("a".to_string().into());
        assert_eq!(r#"["b","c","a"]"#, serde_json::to_string(&s1).unwrap());
        assert_eq!(r#"["a","b","c"]"#, serde_json::to_string(&s2).unwrap());
        assert_ne!(s1, s2);
    }
}
