use rayon::prelude::*;
use typed_index_collections::TiSlice;

pub trait ToParallelTypedIndexIterEnumerated<'a, K, V>
where
    K: Send,
    V: Sync,
    usize: Into<K>,
    V: 'a,
{
    fn par_iter_enumerated(&'a self) -> impl IndexedParallelIterator<Item = (K, &'a V)> + 'a;
}

pub trait ToParallelTypedIndexIterMutEnumerated<'a, K, V>
where
    K: Send,
    V: Send,
    usize: Into<K>,
    V: 'a,
{
    fn par_iter_mut_enumerated(
        &'a mut self,
    ) -> impl IndexedParallelIterator<Item = (K, &'a mut V)> + 'a;
}

impl<'a, K, V> ToParallelTypedIndexIterEnumerated<'a, K, V> for TiSlice<K, V>
where
    K: Send,
    V: Sync,
    usize: Into<K>,
    V: 'a,
{
    fn par_iter_enumerated(&'a self) -> impl IndexedParallelIterator<Item = (K, &'a V)> + 'a {
        self.raw.par_iter().enumerate().map(|(k, v)| (k.into(), v))
    }
}

impl<'a, K, V> ToParallelTypedIndexIterMutEnumerated<'a, K, V> for TiSlice<K, V>
where
    K: Send,
    V: Send,
    usize: Into<K>,
    V: 'a,
{
    fn par_iter_mut_enumerated(
        &'a mut self,
    ) -> impl IndexedParallelIterator<Item = (K, &'a mut V)> + 'a {
        let x: &'a mut [V] = &mut self.raw;
        x.into_par_iter().enumerate().map(|(k, v)| (k.into(), v))
    }
}
