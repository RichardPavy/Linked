use std::cell::RefCell;
use std::hash::Hash;
use std::rc::Rc;

use super::Handle;
use super::LinkedHashMap;
use super::LinkedHashSet;

pub trait IsCollectibleItem: Clone + Sized {
    type Key: Clone + Eq + Hash;
    type Value;
    fn register(self, handle: Handle<Self::Key, Self::Value>);
}

#[derive(Clone)]
pub struct CollectibleItemHandle<T: IsCollectibleItem>(
    Rc<RefCell<Option<Handle<T::Key, T::Value>>>>,
);

impl<T> Default for CollectibleItemHandle<T>
where
    T: IsCollectibleItem,
{
    fn default() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
}

pub mod value {
    use std::hash::Hash;
    use std::ops::Deref;
    use std::ops::DerefMut;

    use super::super::Handle;
    use super::CollectibleItemHandle;
    use super::IsCollectibleItem;

    #[derive(Clone)]
    pub struct CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone,
    {
        handle: CollectibleItemHandle<Self>,
        value: V,
    }

    impl<K, V> IsCollectibleItem for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone,
    {
        type Key = K;
        type Value = Self;

        fn register(self, handle: Handle<Self::Key, Self::Value>) {
            *self.handle.0.borrow_mut() = Some(handle);
        }
    }

    impl<K, V> Default for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone + Default,
    {
        fn default() -> Self {
            Self {
                handle: Default::default(),
                value: Default::default(),
            }
        }
    }

    impl<K, V> From<V> for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone,
    {
        fn from(value: V) -> Self {
            Self {
                handle: Default::default(),
                value,
            }
        }
    }

    impl<K, V> Deref for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone,
    {
        type Target = V;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<K, V> DerefMut for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }

    impl<K, V> std::fmt::Debug for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("CollectibleValue")
                .field("value", &self.value)
                .finish()
        }
    }

    impl<K, V> PartialEq for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone + PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl<K, V> Eq for CollectibleValue<K, V>
    where
        K: Clone + Eq + Hash,
        V: Clone + Eq,
    {
    }
}

pub mod key {
    use std::hash::Hash;
    use std::ops::Deref;
    use std::ops::DerefMut;

    use super::super::Handle;
    use super::CollectibleItemHandle;
    use super::IsCollectibleItem;

    #[derive(Clone)]
    pub struct CollectibleKey<K>
    where
        K: Clone + Eq + Hash,
    {
        handle: CollectibleItemHandle<Self>,
        key: K,
    }

    impl<K> IsCollectibleItem for CollectibleKey<K>
    where
        K: Clone + Eq + Hash,
    {
        type Key = Self;
        type Value = ();

        fn register(self, handle: Handle<Self::Key, Self::Value>) {
            *self.handle.0.borrow_mut() = Some(handle);
        }
    }

    impl<K> PartialEq for CollectibleKey<K>
    where
        K: Clone + Eq + Hash,
    {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl<K> Eq for CollectibleKey<K> where K: Clone + Eq + Hash {}

    impl<K> Hash for CollectibleKey<K>
    where
        K: Clone + Eq + Hash,
    {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.key.hash(state);
        }
    }

    impl<K> Default for CollectibleKey<K>
    where
        K: Clone + Eq + Hash + Default,
    {
        fn default() -> Self {
            Self {
                handle: Default::default(),
                key: Default::default(),
            }
        }
    }

    impl<K> From<K> for CollectibleKey<K>
    where
        K: Clone + Eq + Hash,
    {
        fn from(key: K) -> Self {
            Self {
                handle: Default::default(),
                key,
            }
        }
    }

    impl<K> Deref for CollectibleKey<K>
    where
        K: Clone + Eq + Hash,
    {
        type Target = K;

        fn deref(&self) -> &Self::Target {
            &self.key
        }
    }

    impl<K> DerefMut for CollectibleKey<K>
    where
        K: Clone + Eq + Hash,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.key
        }
    }

    impl<K> std::fmt::Debug for CollectibleKey<K>
    where
        K: Clone + Eq + Hash + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("CollectibleKey")
                .field("key", &self.key)
                .finish()
        }
    }
}

impl<K, V> Extend<(K, V)> for LinkedHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: IsCollectibleItem<Key = K, Value = V>,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            let insert_result = self.insert(k, v.clone());
            v.register(insert_result.handle);
        }
    }
}

impl<K, V> FromIterator<(K, V)> for LinkedHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: IsCollectibleItem<Key = K, Value = V>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> LinkedHashMap<K, V> {
        let mut map = LinkedHashMap::new();
        map.extend(iter);
        map
    }
}

impl<K> Extend<K> for LinkedHashSet<K>
where
    K: Clone + Eq + Hash + IsCollectibleItem<Key = K, Value = ()>,
{
    #[inline]
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for k in iter {
            let insert_result = self.insert(k.clone());
            k.register(insert_result.handle);
        }
    }
}

impl<K> FromIterator<K> for LinkedHashSet<K>
where
    K: Clone + Eq + Hash + IsCollectibleItem<Key = K, Value = ()>,
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> LinkedHashSet<K> {
        let mut map = LinkedHashSet::new();
        map.extend(iter);
        map
    }
}
