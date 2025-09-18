use std::{collections::HashMap, hash::Hash};

pub(crate) trait IteratorExt<K, V>:
    Sized + Iterator<Item = syn::Result<(K, V)>>
where
    K: Eq + Hash,
{
    fn collect_hashmap_and_combine_syn_errors(
        mut self,
    ) -> syn::Result<HashMap<K, V>> {
        let mut map = HashMap::new();

        while let Some(res) = self.next() {
            match res {
                Ok((k, v)) => {
                    map.insert(k, v);
                }
                Err(mut err) => {
                    while let Some(Err(err2)) = self.next() {
                        err.combine(err2);
                    }

                    return Err(err);
                }
            }
        }

        Ok(map)
    }
}

impl<I, K, V> IteratorExt<K, V> for I
where
    I: Sized + Iterator<Item = syn::Result<(K, V)>>,
    K: Eq + Hash,
{
}
