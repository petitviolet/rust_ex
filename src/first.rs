use crate::SortOrder;

pub fn sort<T: Clone + PartialOrd>(x: &[T], order: &SortOrder) -> Result<Vec<T>, String> {
  if x.len().is_power_of_two() {
    let mut vec = x.to_vec();
    _sort(&mut vec, &order);
    return Ok(vec);
  } else {
    return Err(format!("Length of x({}) is invalid", x.len()));
  }
}

fn _sort<T: PartialOrd>(x: &mut [T], order: &SortOrder) -> () {
  if x.len() > 1 {
    let mid_point = mid_point(x);
    _sort(&mut x[..mid_point], &SortOrder::Ascending);
    _sort(&mut x[mid_point..], &SortOrder::Descending);
    sub_sort(x, order);
  }
}

fn sub_sort<T: PartialOrd>(x: &mut [T], order: &SortOrder) {
  if x.len() > 1 {
    compare_and_swap(x, order);
    let mid_point = mid_point(x);
    sub_sort(&mut x[..mid_point], order);
    sub_sort(&mut x[mid_point..], order);
  }
}

fn compare_and_swap<T>(x: &mut [T], order: &SortOrder) where T: PartialOrd {
  let mid_point = mid_point(x);
  for i in 0..mid_point {
    match ((x[i] > x[mid_point + i]), order) {
      (true, SortOrder::Ascending) | (false, SortOrder::Descending) => x.swap(i, mid_point + i),
      _ => (),
    }
  }
}

fn mid_point<A>(x: &[A]) -> usize {
  return x.len() / 2;
}

#[cfg(test)]
mod tests {
  use super::sort;
  use super::SortOrder;

  fn assert_sort_result<T: std::fmt::Debug + PartialEq>(result: Result<Vec<T>, String>, expected: Vec<T>) {
    match result {
      Ok(actual) => {
        assert_eq!(actual, expected);
      },
      Err(err) =>
        assert!(false, "should not get Err({})", err),
    }
  }
  #[test]
  fn sort_u32_ascending() {
    let x = vec![10, 39, 11, 20, 4, 330, 21, 110];
    assert_sort_result(sort(&x, &SortOrder::Ascending), vec![4, 10, 11, 20, 21, 39, 110, 330]);
  }

  #[test]
  fn sort_u32_decending() {
    let x = vec![10, 39, 11, 20, 4, 330, 21, 110];
    assert_sort_result(sort(&x, &SortOrder::Descending), vec![330, 110, 39, 21, 20, 11, 10, 4]);
  }

  #[test]
  fn sort_str_decending() {
    let x = vec!["a", "c", "f", "e", "g", "b", "d", "h"];
    assert_sort_result(sort(&x, &SortOrder::Descending), vec!["h", "g", "f", "e", "d", "c", "b", "a"]);
  }
}