#[derive(Debug)]
pub struct Myvec<T> {
  items: Box<[T]>,
  pub len: usize,
}

impl<T: Default> Myvec<T> {
  pub fn new(capacity: usize) -> Self {
    Self {
      items: Self::allocate_in_heap(capacity),
      len: 0,
    }
  }
  pub fn empty() -> Self {
    Self::new(0)
  }

  fn allocate_in_heap(capacity: usize) -> Box<[T]> {
    std::iter::repeat_with(Default::default)
      .take(capacity)
      .collect::<Vec<T>>()
      .into_boxed_slice()
  }

  pub fn capacity(&self) -> usize { 
    self.items.len()
  }

  pub fn append(&mut self, item: T) -> () {
    if self.len >= self.capacity() {
      self.ensure_capacity();
    }
    self.items[self.len] = item;
    self.len += 1;
  }

  fn ensure_capacity(&mut self) -> () { 
    if self.capacity() == 0 {
      self.items = Self::allocate_in_heap(1);
    } else {
      let new_items = Self::allocate_in_heap(self.capacity() * 2);
      // I think this is not thread-safe
      let old_items = std::mem::replace(&mut self.items, new_items);
      // into_vec/into_vec convert type without copying data
      for (i, item) in old_items.into_vec().into_iter().enumerate() {
        self.items[i] = item;
      }
    }
  }

  pub fn fetch(&self, index: usize) -> Option<&T> {
    if index < self.len {
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
  pub fn pop(&mut self) -> Option<T> { 
    if self.len == 0 { None }
    else {
      let last_item = std::mem::replace(&mut self.items[self.len - 1], Default::default());
      self.len -= 1;
      Some(last_item)
    }
  }
}

pub fn play_myvec() -> () {
  let mut myvec: Myvec<i32> = Myvec::empty();
  println!("myvec: {:?}", myvec);
  let myvec2 = myvec.append(10);
  println!("myvec: {:?}", myvec);
  println!("myvec2: {:?}", myvec2);
  (0..5).for_each(|i| myvec.append(i));
  println!("myvecs: {:?}", myvec);

  let mut appended = (0..5).fold(Myvec::empty(), |mut acc, i| {
    acc.append(format!("item-{}", i));
    acc
  });
  println!("appended: {:?}", appended);
  let item_2 = appended.fetch(2);
  println!("appended.fetch(2): {:?}", item_2);
  // appended.append("item-new".to_string()); // mutable borrow occurs here
  println!("appended.fetch(2): {:?}", item_2);

  println!("before pop: {:?}", appended);
  println!("pop: {:?}", appended.pop());
  println!("after pop: {:?}", appended);
  println!(
    "appended.fetch_or(10, \"foo\"): {:?}",
    appended.fetch_or(10, &"foo".to_string())
  );
}
