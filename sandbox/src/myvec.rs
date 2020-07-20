#[derive(Debug)]
pub struct Myvec<T> {
  items: Box<[T]>,
}

impl<T: Clone> Myvec<T> {
  pub fn new() -> Self {
    Self {
      items: vec![].into_boxed_slice(),
    }
  }
  fn from(items: &[T]) -> Self {
    Self {
      items: items.to_vec().into_boxed_slice(),
    }
  }
  pub fn size(&self) -> usize {
    self.items.len()
  }

  pub fn append(&self, item: T) -> Self {
    let items = [self.items.clone(), Box::new([item])].concat();
    Self::from(&items)
  }

  pub fn fetch(&self, index: usize) -> Option<&T> {
    if index < self.size() {
      Some(&self.items[index])
    } else {
      None
    }
  }
  pub fn fetch_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
    match self.fetch(index) {
      Some(item) => item,
      None => default,
    }
  }
}

pub fn play_myvec() -> () {
  let myvec: Myvec<i32> = Myvec::new();
  println!("myvec: {:?}", myvec);
  let myvec2 = myvec.append(10);
  println!("myvec: {:?}", myvec);
  println!("myvec2: {:?}", myvec2);
  let myvecs = (0..5).map(|i| myvec.append(i)).collect::<Vec<_>>();
  println!("myvecs: {:?}", myvecs);

  let appended = (0..5).fold(Myvec::new(), |acc, i| acc.append(format!("item-{}", i)));
  println!("appended: {:?}", appended);
  println!("appended.fetch(2): {:?}", appended.fetch(2));
  println!(
    "appended.fetch_or(10, \"foo\"): {:?}",
    appended.fetch_or(10, &"foo".to_string())
  );
}
