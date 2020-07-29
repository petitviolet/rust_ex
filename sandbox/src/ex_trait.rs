use std::ops::Add;

#[derive(Debug)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct PolarCoord {
    r: f64,
    theta: f64,
}

trait Coordinates {
    fn to_cartesian(&self) -> CartesianCoord;
    fn from_catesian<'a>(cart: &CartesianCoord) -> Self;
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(&self) -> CartesianCoord {
        CartesianCoord {
            x: self.x,
            y: self.y,
        }
    }
    fn from_catesian(cart: &CartesianCoord) -> Self {
        CartesianCoord {
            x: cart.x,
            y: cart.y,
        }
    }
}

impl Coordinates for PolarCoord {
    fn to_cartesian(&self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_catesian(cart: &CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}
impl Coordinates for (f64, f64) {
    fn to_cartesian(&self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }
    fn from_catesian(cart: &CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

impl Add<CartesianCoord> for CartesianCoord {
    type Output = CartesianCoord;
    fn add(self, rhs: CartesianCoord) -> Self::Output {
        CartesianCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn print_point<C: Coordinates + std::fmt::Debug>(c: &C) -> () {
    println!(
        "point: {:?}, cartesian: {:?}, polar: {:?}",
        c,
        c.to_cartesian(),
        PolarCoord::from_catesian(&c.to_cartesian())
    );
}

pub fn f() -> () {
    let point = (2.0, 2.0);
    print_point(&point);
    print_point(&((1.0, 2.0).to_cartesian() + (10.0, 20.0).to_cartesian()));
}

trait Animal {
    fn berk(&self) -> ();
}
#[derive(Debug)]
struct Dog {
    name: String,
    color: String,
}
impl Dog {
    pub fn new(name: impl Into<String>, color: impl Into<String>) -> Dog {
        Dog {
            name: name.into(),
            color: color.into(),
        }
    }
}
impl Animal for Dog {
    fn berk(&self) -> () {
        println!("Dog({})!!!", self.name);
    }
}
#[derive(Debug)]
struct Cat {
    name: String,
}
impl Cat {
    pub fn new(name: impl Into<String>) -> Cat {
        Cat { name: name.into() }
    }
}
impl Animal for Cat {
    fn berk(&self) -> () {
        println!("Cat({})!!!", self.name);
    }
}
fn berk_dyn(animal: &dyn Animal) -> () {
    animal.berk();
}
fn berk_impl(animal: impl Animal) -> () {
    animal.berk();
}
fn born(name: impl Into<String>) -> impl Animal + std::fmt::Debug {
    Dog::new(name, "red")
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn g() -> () {
    berk_dyn(&Dog::new("pochi", "red"));
    berk_dyn(&Cat::new("tama"));
    berk_impl(Dog::new("pochi", "red"));
    berk_impl(Cat::new("tama"));
    let animal = born("john");
    print_type_of(&animal);
    println!("animal: {:?}", animal);
    berk_dyn(&animal);
    berk_impl(animal);
}
