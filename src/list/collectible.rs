use std::cell::Cell;
use std::ops::Deref;
use std::ops::DerefMut;

use super::Handle;
use super::LinkedList;
use super::RcNodeFactory;

pub trait IsCollectibleItem: Sized {
    type Value;
    fn handle_cell(&self) -> &Cell<Option<Handle<RcNodeFactory<Self>>>>;
}

type CollectibleHandle<V> = Cell<Option<Handle<RcNodeFactory<V>>>>;

pub struct CollectibleValue<V> {
    handle_cell: CollectibleHandle<Self>,
    value: V,
}

impl<V> IsCollectibleItem for CollectibleValue<V> {
    type Value = V;
    fn handle_cell(&self) -> &CollectibleHandle<Self> {
        &self.handle_cell
    }
}

impl<V> Default for CollectibleValue<V>
where
    V: Default,
{
    fn default() -> Self {
        Self {
            handle_cell: Cell::default(),
            value: V::default(),
        }
    }
}

impl<V> From<V> for CollectibleValue<V> {
    fn from(value: V) -> Self {
        Self {
            handle_cell: Cell::default(),
            value,
        }
    }
}

impl<V> Deref for CollectibleValue<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> DerefMut for CollectibleValue<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<V> std::fmt::Debug for CollectibleValue<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CollectibleValue")
            .field(&self.value)
            .finish()
    }
}

impl<V> PartialEq for CollectibleValue<V>
where
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V> Eq for CollectibleValue<V> where V: Eq {}

impl<V> Extend<V> for LinkedList<V>
where
    V: IsCollectibleItem,
{
    #[inline]
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        for v in iter {
            let handle = Some(self.push_back(v));
            let handle_cell = {
                let handle_ptr = std::ptr::from_ref(&handle);
                let handle_ref = unsafe { &*handle_ptr };
                let value = handle_ref.as_ref().unwrap().value();
                value.handle_cell()
            };

            // Note: `handle_cell` shoud 'borrow' `handle`, except we 'broke' the borrow rule
            // by casting to a pointer. The risk is this *moves* `handle`, which could also move
            // `handle_cell` before `set` is called, and result in UB. This is safe because `handle`
            // doesn't contain `value` directly, there is a layer of indirection so moving the
            // `handle` does not affect `handle_cell`.
            handle_cell.set(handle);
        }
    }
}

impl<V> FromIterator<V> for LinkedList<V>
where
    V: IsCollectibleItem,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

#[cfg(test)]
mod tests {
    use super::super::LinkedList;
    use super::CollectibleValue;

    #[test]
    fn collect() {
        let values = [
            CollectibleValue::from("a".to_string()),
            "b".to_owned().into(),
            "c".to_owned().into(),
        ]
        .into_iter()
        .collect::<LinkedList<_>>();
        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            values.iter().map(|v| v.to_string()).collect::<Vec<_>>()
        );
    }
}
