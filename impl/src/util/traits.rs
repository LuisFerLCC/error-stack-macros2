pub(crate) trait IteratorExt<T>:
    Sized + Iterator<Item = syn::Result<T>>
{
    fn collect_vec_and_combine_syn_errors(mut self) -> syn::Result<Vec<T>> {
        let mut vec = Vec::new();

        while let Some(res) = self.next() {
            match res {
                Ok(item) => {
                    vec.push(item);
                }

                Err(mut err) => {
                    while let Some(Err(err2)) = self.next() {
                        err.combine(err2);
                    }

                    return Err(err);
                }
            }
        }

        Ok(vec)
    }
}

impl<I, T> IteratorExt<T> for I where I: Sized + Iterator<Item = syn::Result<T>> {}
