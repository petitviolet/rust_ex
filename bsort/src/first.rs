use crate::SortOrder;
use std::cmp::Ordering;

pub fn sort<T: Clone + Ord>(x: &[T], order: &SortOrder) -> Result<Vec<T>, String> {
  if x.len().is_power_of_two() {
    let mut vec = x.to_vec();
    match *order {
      SortOrder::Ascending => _sort(&mut vec, false, &|a, b| a.cmp(b)),
      SortOrder::Descending => _sort(&mut vec, true, &|a, b| a.cmp(b)),
    };
    return Ok(vec);
  } else {
    return Err(format!("Length of x({}) is invalid", x.len()));
  }
}
pub fn sort_by<T: Clone, F>(x: &[T], comparator: &F) -> Result<Vec<T>, String>
  where F: Fn(&T, &T) -> Ordering
{
  if x.len().is_power_of_two() {
    let mut vec = x.to_vec();
    _sort(&mut vec, false, comparator);
    return Ok(vec);
  } else {
    return Err(format!("Length of x({}) is invalid", x.len()));
  }
}

fn _sort<T, F>(x: &mut [T], reverse: bool, comparator: &F) -> ()
  where F: Fn(&T, &T) -> Ordering
{
  if x.len() > 1 {
    let mid_point = mid_point(x);
    _sort(&mut x[..mid_point], false, comparator);
    _sort(&mut x[mid_point..], true, comparator);
    sub_sort(x, reverse, comparator);
  }
}

fn sub_sort<T, F>(x: &mut [T], reverse: bool, comparator: &F)
  where F: Fn(&T, &T) -> Ordering
{
  if x.len() > 1 {
    compare_and_swap(x, reverse, comparator);
    let mid_point = mid_point(x);
    sub_sort(&mut x[..mid_point], reverse, comparator);
    sub_sort(&mut x[mid_point..], reverse, comparator);
  }
}

fn compare_and_swap<T, F>(x: &mut [T], reverse: bool, comparator: &F)
  where F: Fn(&T, &T) -> Ordering
{
  let mid_point = mid_point(x);
  let ordering = if reverse { Ordering::Less } else { Ordering::Greater };

  for i in 0..mid_point {
    if comparator(&x[i], &x[mid_point + i]) == ordering {
      x.swap(i, mid_point + i);
    }
  }
}

fn mid_point<A>(x: &[A]) -> usize {
  return x.len() / 2;
}

#[cfg(test)]
mod tests {
  use super::sort;
  use super::sort_by;
  use super::SortOrder;
  use crate::utils::{is_sorted, new_u32_vec};

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

  #[derive(PartialEq, Debug, Clone)]
  struct User {
    name: String,
    age: u8,
  }
  impl User {
    fn new(name: &str, age: u8) -> Self {
      Self {
        name: name.to_string(),
        age: age,
      }
    }
  }
  #[test]
  fn sort_users_descending() {
    let alice = User::new("alice", 10);
    let bob = User::new("bob", 40);
    let charlie = User::new("charlie", 20);
    let dave = User::new("dave", 50);
    let users = vec![&alice, &dave, &charlie, &bob];
    assert_sort_result(sort_by(&users, &|u1, u2| u1.age.cmp(&u2.age)), vec![&alice, &charlie, &bob, &dave]);
  }

  #[test]
  fn sort_huge_number_arrays() {
    let arr = new_u32_vec(65536);
    match sort(&arr, &SortOrder::Ascending) {
      Ok(res) =>
        assert!(is_sorted(&res, &SortOrder::Ascending)),
      Err(err) =>
        assert!(false, "should not get Err({})", err),
    }

    match sort(&arr, &SortOrder::Descending) {
      Ok(res) =>
        assert!(is_sorted(&res, &SortOrder::Descending)),
      Err(err) =>
        assert!(false, "should not get Err({})", err),
    }
  }
}
