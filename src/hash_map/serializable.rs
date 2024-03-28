use std::hash::Hash;

use super::collectible::IsCollectibleItem;
use super::LinkedHashMap;
use super::LinkedHashSet;

impl<K, V> serde::Serialize for LinkedHashMap<K, V>
where
    K: serde::Serialize + Clone + Eq + Hash,
    V: serde::Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<K> serde::Serialize for LinkedHashSet<K>
where
    K: serde::Serialize + Clone + Eq + Hash,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de, K, V> serde::Deserialize<'de> for LinkedHashMap<K, V>
where
    K: serde::Deserialize<'de> + Clone + Eq + Hash,
    V: serde::Deserialize<'de> + IsCollectibleItem<Key = K, Value = V>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = Vec::<(K, V)>::deserialize(deserializer)?;
        Ok(data.into_iter().collect())
    }
}

impl<'de, K> serde::Deserialize<'de> for LinkedHashSet<K>
where
    K: serde::Deserialize<'de> + Clone + Eq + Hash + IsCollectibleItem<Key = K, Value = ()>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = Vec::<K>::deserialize(deserializer)?;
        Ok(data.into_iter().collect())
    }
}

mod value {
    use std::hash::Hash;

    use super::super::collectible::value::CollectibleValue;

    impl<K, V> serde::Serialize for CollectibleValue<K, V>
    where
        K: serde::Serialize + Clone + Eq + Hash,
        V: serde::Serialize + Clone,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value: &V = &*self;
            value.serialize(serializer)
        }
    }

    impl<'de, K, V> serde::Deserialize<'de> for CollectibleValue<K, V>
    where
        K: serde::Deserialize<'de> + Clone + Eq + Hash,
        V: serde::Deserialize<'de> + Clone,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = V::deserialize(deserializer)?;
            Ok(CollectibleValue::from(value))
        }
    }
}

mod key {
    use std::hash::Hash;

    use super::super::collectible::key::CollectibleKey;

    impl<K> serde::Serialize for CollectibleKey<K>
    where
        K: serde::Serialize + Clone + Eq + Hash,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let key: &K = &*self;
            key.serialize(serializer)
        }
    }

    impl<'de, K> serde::Deserialize<'de> for CollectibleKey<K>
    where
        K: serde::Deserialize<'de> + Clone + Eq + Hash,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let key = K::deserialize(deserializer)?;
            Ok(CollectibleKey::from(key))
        }
    }
}
