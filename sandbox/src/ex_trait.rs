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
}

trait Animal {
    fn berk(&self) -> ();
}
struct Dog {
    name: &'static str,
}
impl Animal for Dog {
    fn berk(&self) -> () {
        println!("Dog({})!!!", self.name);
    }
}
struct Cat {
    name: &'static str,
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
pub fn g() -> () {
    berk_dyn(&Dog { name: "pochi" });
    berk_dyn(&Cat { name: "tama" });
    berk_impl(Dog { name: "pochi" });
    berk_impl(Cat { name: "tama" });
}
