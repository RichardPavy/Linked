use std::ops::Deref;

use super::collectible::CollectibleValue;
use super::collectible::IsCollectibleItem;
use super::node_ref::NodeRef;
use super::LinkedList;

impl<V> serde::Serialize for LinkedList<V>
where
    V: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data: Vec<_> = self.iter().collect();
        serializer.collect_seq(data.iter().map(NodeRef::deref))
    }
}

impl<'de, V> serde::Deserialize<'de> for LinkedList<V>
where
    V: serde::Deserialize<'de> + IsCollectibleItem<Value = V>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = Vec::<V>::deserialize(deserializer)?;
        Ok(data.into_iter().map(V::from).collect())
    }
}

impl<V> serde::Serialize for CollectibleValue<V>
where
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

impl<'de, V> serde::Deserialize<'de> for CollectibleValue<V>
where
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
