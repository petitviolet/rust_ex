mod ex_trait;
mod myvec;

fn main() {
    println!("{}", bang("Hello, world"));
    pointer();
    println!("{:?}", option::f());
    println!("{:?}", result::g(&["1", "2", "hoge"]));
    println!("{:?}", result::g(&["1", "2", "3"]));
    println!("{:?}", adt::person());
    println!("{}", adt::task(&adt::Task::Default));
    println!(
        "{}",
        adt::task2(&adt::InProgress {
            person: adt::person()
        })
    );

    // need to `use` in order to call trait's functions
    use crate::adt::Distance;
    println!("{:?}", adt::Mile::new(100).convert());
    adt::do_mile_convert();

    closure::f();
    ownership::copy::f();
    ownership::mv::f();

    myvec::play_myvec();
    rc::f();
    refcell::f();
    arc::f();
    ex_trait::f();
    ex_trait::g();
}

use rand::Rng;
// static rng: rand::rngs::ThreadRng = rand::thread_rng();
fn rand_int(max: Option<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, max.unwrap_or(100))
}

static BANG: &str = "!!!";
fn bang(s: &str) -> String {
    return format!("{}{}", s, BANG);
}

fn pointer() -> () {
    let mut n1 = "hoge";
    println!("  n1: {:p}", n1);
    println!(" &n1: {:p}", &n1);
    println!("&&n1: {:p}", &&n1);
    println!("*&n1: {:p}", *&n1);

    let n1_ptr = &mut n1;
    println!("    n1_ptr: {:p}", n1_ptr);
    println!("   *n1_ptr: {:p}", *n1_ptr);
    println!("  &*n1_ptr: {:p}", &*n1_ptr);
    println!("   &n1_ptr: {:p}", &n1_ptr);
    println!("&&&&n1_ptr: {:p}", &&&&n1_ptr);
    println!("  *&n1_ptr: {:p}", *&n1_ptr);

    *n1_ptr = "foo";
    println!("do `*n1_ptr = \"foo\"`");
    println!("  n1_ptr: {:p}", n1_ptr);
    println!(" *n1_ptr: {:p}", *n1_ptr);
    println!("&*n1_ptr: {:p}", &*n1_ptr);
    println!(" &n1_ptr: {:p}", &n1_ptr);
    println!("*&n1_ptr: {:p}", *&n1_ptr);
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
    impl Task {
        pub const Default: Task = Task::Ready;
    }
    pub fn task(t: &Task) -> String {
        let x = match t {
            Task::Ready => "ready".to_string(),
            Task::InProgress(person) => format!("in progress by {:?}", person),
            Task::Done { finished_by } => format!("done by {:?}", finished_by),
        };
        return x;
    }

    pub trait Task2 {}

    #[derive(Debug)]
    pub struct Ready {}
    impl Task2 for Ready {}

    #[derive(Debug)]
    pub struct InProgress {
        pub person: Person,
    }
    impl Task2 for InProgress {}

    #[derive(Debug)]
    pub struct Done {
        finished_by: Person,
    }
    impl Task2 for Done {}

    pub fn task2(t: &dyn Task2) -> String {
        // let x = match t {
        //   Ready => "ready".to_string(),
        //   InProgress { person } => format!("in progress by {:?}", person),
        //   Done { finished_by } => format!("done by {:?}", finished_by),
        // };
        // return x;
        "cannot do downcasting".to_string()
    }

    pub trait Distance {
        type Other: Distance;
        fn doubled(&self) -> Self;
        fn convert(&self) -> Self::Other;
    }
    #[derive(Debug)]
    pub struct Kilometer {
        value: i32,
    }
    impl Kilometer {
        pub fn new(value: i32) -> Kilometer {
            Kilometer { value: value }
        }
    }
    impl Distance for Kilometer {
        type Other = Mile;

        fn doubled(&self) -> Kilometer {
            Kilometer {
                value: self.value * 2,
            }
        }
        fn convert(&self) -> Mile {
            Mile {
                value: (self.value as f32 / 1.6) as i32,
            }
        }
    }
    #[derive(Debug)]
    pub struct Mile {
        value: i32,
    }
    impl Mile {
        pub fn new(value: i32) -> Mile {
            Mile { value: value }
        }
    }
    impl Distance for Mile {
        type Other = Kilometer;
        fn doubled(&self) -> Mile {
            Mile {
                value: self.value * 2,
            }
        }
        fn convert(&self) -> Kilometer {
            Kilometer {
                value: (self.value as f32 * 1.6) as i32,
            }
        }
    }
    pub fn do_mile_convert() -> () {
        println!("{:?}", Mile { value: 100 }.convert());
    }
}

mod closure {
    pub fn f() -> () {
        let adder = |a, b| a + b;
        println!("adder(1, 2): {}", adder(1, 2));
        let mut n = 1;
        let add_n = |a| n + a;
        println!("add_n(2): {}", add_n(2));
        // n = 2; // `n` is borrowed by `add_n`
        let mut n = 2; // shadow-ing
        let add_n2 = move |a| n + a; // `move` keyword
        println!("add_n2(2): {}", add_n2(2));
        n = 20; // `n` belongs to `add_n`
        println!("add_n2(2): {}", add_n2(2));
    }
}

mod ownership {
    pub mod copy {
        #[derive(Debug, Copy, Clone)]
        struct Parent(isize, Child);
        #[derive(Debug, Copy, Clone)]
        struct Child(isize);
        // impl Drop for Parent { fn drop(&mut self) { println!("Drop: {:?}", self)} }
        // impl Drop for Child { fn drop(&mut self) { println!("Drop: {:?}", self)} }
        // impl Copy for Parent {}
        // impl Copy for Child {}

        pub fn f() -> () {
            println!("ownership::copy");
            {
                let p1 = Parent(1, Child(2));
                println!("p1: {:?}", p1);
            }
            let c2 = Child(3);
            {
                let c2_ = c2; // copy, not moved
                let p2 = Parent(4, c2_);
                println!("p2: {:?}", p2);
            }

            println!("{:?}", c2); // if Parent and Child don't have Copy trait, this will be an error by `c2` is borrowd by the above block
            let p3 = Parent(5, Child(6));
            println!("p3: {:?}", p3);
            {
                let p4 = Parent(7, p3.1);
                println!("p4: {:?}", p4);
            }
            println!("p3: {:?}", p3); // if Parent and Child don't have Copy trait, this will be an error by `p3` is partially borrowed by `p4`
        }
    }
    pub mod mv {
        #[derive(Debug)]
        struct Parent(isize, Child);
        #[derive(Debug)]
        struct Child(isize);
        impl Drop for Parent {
            fn drop(&mut self) {
                println!("Drop: {:?}", self)
            }
        }
        impl Drop for Child {
            fn drop(&mut self) {
                println!("Drop: {:?}", self)
            }
        }

        pub fn f() -> () {
            println!("ownership::mv");
            {
                let p1 = Parent(1, Child(2));
                println!("p1: {:?}", p1);
            }
            let c2 = Child(3);
            {
                let c2_ = c2; // copy, not moved
                let p2 = Parent(4, c2_);
                println!("p2: {:?}", p2);
            }

            // println!("{:?}", c2); // `c2` is borrowd by the above block
            let p3 = Parent(5, Child(6));
            println!("p3: {:?}", p3);
            {
                // let p4 = Parent(7, p3.1); // cannot move out of here. need Copy trait
                // println!("p4: {:?}", p4);
            }

            copy_g(p3);
            // println!("p3: {:?}", p3); // value borrwoed here after move to `copy_g`
            let p4 = Parent(7, Child(8));
            println!("p4: {:?}", p4);
            ref_g(&p4);
            println!("p4: {:?}", p4);
        }

        fn copy_g(p: Parent) -> () {
            println!("copy_g(p): {:?}", p);
        }

        fn ref_g(p: &Parent) -> () {
            println!("ref_g(p): {:?}", p);
        }
    }
}

mod rc {
    use std::rc::Rc;
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }
    pub fn f() -> () {
        let mut rc1 = Rc::new(Person {
            name: "alice".to_string(),
            age: 30,
        });
        println!(
            "strong_count: {count}, rc1: {rc1:?}",
            rc1 = rc1,
            count = Rc::strong_count(&rc1)
        );
        {
            let rc2 = Rc::clone(&rc1);
            println!(
                "strong_count: {count}, rc1: {rc1:?}, rc2: {rc2:?}",
                rc1 = rc1,
                rc2 = rc2,
                count = Rc::strong_count(&rc1)
            );

            {
                if let Some(person) = Rc::get_mut(&mut rc1) {
                    person.age += 1;
                    println!(
                        "updated strong_count: {count}, rc1: {rc1:?}",
                        rc1 = rc1,
                        count = Rc::strong_count(&rc1)
                    );
                } else {
                    println!("cannot get mut reference.");
                    println!(
                        "strong_count: {count}, rc1: {rc1:?}",
                        rc1 = rc1,
                        count = Rc::strong_count(&rc1)
                    );
                }
            }
        }
        println!(
            "strong_count: {count}, rc1: {rc1:?}",
            rc1 = rc1,
            count = Rc::strong_count(&rc1)
        );

        {
            if let Some(person) = Rc::get_mut(&mut rc1) {
                person.age += 1;
                println!(
                    "updated strong_count: {count}, rc1: {rc1:?}",
                    rc1 = rc1,
                    count = Rc::strong_count(&rc1)
                );
            } else {
                println!("cannot get mut reference.");
                println!(
                    "strong_count: {count}, rc1: {rc1:?}",
                    rc1 = rc1,
                    count = Rc::strong_count(&rc1)
                );
            }
        }
        {
            let rc3 = Rc::downgrade(&rc1);
            println!(
                "strong_count: {count}, weak_count: {weak_count}, rc1: {rc1:?}, rc3: {rc3:?}",
                rc1 = rc1,
                rc3 = rc3,
                count = Rc::strong_count(&rc1),
                weak_count = Rc::weak_count(&rc1)
            );

            if let Some(upgraded) = rc3.upgrade() {
                println!(
          "strong_count: {count}, weak_count: {weak_count}, rc1: {rc1:?}, rc3: {rc3:?}, upgraded: {upgraded:?}",
          rc1 = rc1,
          rc3 = rc3,
          upgraded = upgraded,
          count = Rc::strong_count(&rc1),
          weak_count = Rc::weak_count(&rc1)
        );
            } else {
                println!("cannot upgrade");
            }

            std::mem::drop(rc1);

            if let Some(weak) = rc3.upgrade() {
                println!("rc3: {rc3:?}, weak: {weak:?}", rc3 = rc3, weak = weak,);
            } else {
                println!(
                    "cannot upgrade. rc3: {:?}, upgrade(): {:?}",
                    rc3,
                    rc3.upgrade()
                );
            }
        }
    }
}

mod refcell {
    use std::cell::RefCell;
    use std::collections::HashSet;

    thread_local!(
      static NAMES: RefCell<HashSet<String>> = {
        let names = ["alice".to_string(), "bob".to_string()].iter().cloned().collect();
        RefCell::new(names)
      }
    );

    pub fn f() -> () {
        NAMES.with(|names| {
            println!("names: {:?}", names);
            names.borrow_mut().insert("charlie".to_string());
            println!("names: {:?}", names);
        });

        std::thread::spawn(|| {
            NAMES.with(|names| {
                println!("[spawned thread]names: {:?}", names);
                names.borrow_mut().insert("dave".to_string());
                println!("[spawned thread]names: {:?}", names);
            });
        })
        .join()
        .expect("!!!");

        NAMES.with(|names| {
            println!("names: {:?}", names);
            names.borrow_mut().insert("ellen".to_string());
            println!("names: {:?}", names);
        });
    }
}

mod arc {
    use std::collections::HashSet;
    use std::error::Error;
    use std::sync::{Arc, RwLock};

    pub fn f() -> Result<(), Box<dyn Error>> {
        type T = Arc<RwLock<HashSet<String>>>;
        let names: T = Arc::new(RwLock::new(
            ["alice".to_string(), "bob".to_string()]
                .iter()
                .cloned()
                .collect::<HashSet<String>>(),
        ));
        let start = std::time::Instant::now();
        let elapsed = move || {
            return std::time::Instant::now().duration_since(start);
        };

        let read = || -> () {
            let cloned1 = Arc::clone(&names);
            std::thread::spawn(move || {
                let read_names = cloned1.read().map_err(|err| err.to_string());
                println!("[{:?}]read_names: {:?}", elapsed(), read_names);
                std::thread::sleep_ms(30);
                println!("[{:?}]finish sleep", elapsed());
            }); //.join().expect("fail!");
        };

        let write = || -> () {
            let cloned2 = Arc::clone(&names);
            std::thread::spawn(move || {
                println!("[{:?}]write try to get lock.", elapsed());
                let write_names = cloned2.write().map_err(|err| err.to_string());
                println!("[{:?}]write got lock.", elapsed());
                write_names
                    .map(|mut wn| {
                        println!("[{:?}]write_names: {:?}", elapsed(), wn);
                        wn.insert(format!("now-{:?}", elapsed()));
                    })
                    .expect("!!!");
            }); //.join().expect("fail!");
        };

        read();
        write();
        read();
        write();
        read();

        std::thread::sleep_ms(100);
        Ok(())
    }
}
