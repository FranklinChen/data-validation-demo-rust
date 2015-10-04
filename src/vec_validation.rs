// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

/// Convenient type synonym for a common pattern of usage.
/// More generically, `Vec` could be replaced by a type implementing
/// the trait [`Add`](http://doc.rust-lang.org/1.0.0-beta/std/ops/trait.Add.html).
pub type VecValidation<T, E> = Result<T, Vec<E>>;

#[inline]
pub fn single<T, E>(result: Result<T, E>) -> VecValidation<T, E> {
  result.map_err(|e| vec![e])
}

/// Combine 2 successful results with `f`, but accumulate failures.
/// Any single failure causes the whole result to be the collection
/// of all failures seen from start to finish.
///
/// Generalization to 3 results or a sequence of results is
/// straightforward.
pub fn combine2<T1, // success type for result1
                T2, // success type for result2
                Tn, // success type for final result
                E,  // error type
                F: FnOnce(T1, T2) -> Tn
               >(
  result1: VecValidation<T1, E>,
  result2: VecValidation<T2, E>,
  combine_success: F) -> VecValidation<Tn, E>
{
  match (result1, result2) {
    (Ok(v),       Ok(t))   => Ok(combine_success(v, t)),
    (Ok(..),      Err(e2)) => Err(e2),
    (Err(e1),     Ok(..))  => Err(e1),
    (Err(mut e1), Err(e2)) => Err({ e1.extend(e2.into_iter()); e1 })
  }
}
