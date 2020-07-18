fn main() {
  println!("{}", bang("Hello, world"));
  println!("{:?}", option::f());
  println!("{:?}", result::g(&["1", "2", "hoge"]));
  println!("{:?}", result::g(&["1", "2", "3"]));
  println!("{:?}", adt::person());
  println!("{}", adt::task(&adt::Task::InProgress(adt::person())));
}

use rand::Rng;
fn rand_int(max: Option<i32>) -> i32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(0, max.unwrap_or(100))
}

fn bang(s: &str) -> String {
  return format!("{}!!!", s);
}

mod option {
  use crate::rand_int;
  pub fn f() -> Option<i64> {
    let x = Some(rand_int(None))
      .map(|n| n * 2)
      .and_then(|i| if i > 10 { Some(i * 100) } else { None })
      .unwrap_or(10);
    return Option::from(x as i64);
  }
}

mod result {
  pub fn g(arr: &[&str]) -> Result<i64, std::num::ParseIntError> {
    let x = arr
      .iter()
      .map(|s| s.parse::<i64>())
      .fold(0, |acc, x| acc + x.unwrap_or(0));

    let mut y = 0;
    for s in arr.iter() {
      let i = s.parse::<i64>()?;
      y += i;
    }

    Ok(x + y)
  }
}

mod adt {
  use crate::rand_int;
  #[derive(Debug)]
  struct Name(String);
  enum Gender {
    Male = 1,
    Female = 2,
    Other = 3,
  }
  impl Gender {
    pub fn of(i: isize) -> Gender {
      match i {
        1 => Gender::Male,
        2 => Gender::Female,
        _ => Gender::Other,
      }
    }
  }
  impl std::fmt::Debug for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let s = match self {
        &Gender::Male => "male",
        &Gender::Female => "female",
        &Gender::Other => "other",
      };
      f.write_str(&format!("Gender({})", &s))
    }
  }
  #[derive(Debug)]
  pub struct Person {
    name: Name,
    gender: Gender,
  }
  impl Person {
    pub fn new(name: &str, gender: i32) -> Person {
      Person {
        name: Name(name.to_string()),
        gender: Gender::of(gender as isize),
      }
    }
  }
  pub fn person() -> Person {
    Person::new("alice", rand_int(Some(3)))
  }

  pub enum Task {
    Ready,
    InProgress(Person),
    Done { finished_by: Person },
  }
  pub fn task(t: &Task) -> String {
    let x = match t {
      Task::Ready => "ready".to_string(),
      Task::InProgress(person) => format!("in progress by {:?}", person),
      Task::Done { finished_by } => format!("done by {:?}", finished_by),
    };
    return x;
  }
}
