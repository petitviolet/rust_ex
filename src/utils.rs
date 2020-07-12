use crate::SortOrder;
use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
  let mut rng = Pcg64Mcg::from_seed([0; 16]);
  return rng.sample_iter(&Standard).take(n).collect();
}

pub fn is_sorted<T: Ord>(x: &[T], order: &SortOrder) -> bool {
  let cmp: Box<dyn Fn(&[T]) -> bool> = match order {
    &SortOrder::Ascending => Box::new(|pair: &[T]| pair[0] <= pair[1]),
    &SortOrder::Descending => Box::new(|pair: &[T]| pair[0] >= pair[1]),
  };
  // let cmp: fn(&[T]) -> bool = match order {
  //   &SortOrder::Ascending => |pair: &[T]| pair[0] <= pair[1],
  //   &SortOrder::Descending => |pair: &[T]| pair[0] >= pair[1],
  // };
  return x.windows(2).all(cmp);
}
