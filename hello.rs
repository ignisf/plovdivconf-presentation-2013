extern mod extra;
use extra::complex::*;

struct Circle {
    k: Cmplx<float>,
    z: Cmplx<float>
}

impl Circle {
    fn new(k: float, x: float, y: float) -> Circle {
        Circle {k: Cmplx::new(k, 0.0), z: Cmplx::new(x, y)}
    }
}

fn get_daughter(m: &Circle, a1: &Circle, a2: &Circle,
                a3: &Circle, k: float) -> Cmplx<float> {
    let (~p, ~q) = (~Cmplx::new(2.0, 0.0), ~Cmplx::new(k, 0.0));
    ((m.z * m.k + a2.z * a2.k + a3.z * a3.k) * p - a1.z * a1.k) / q
}

fn main() {
    let a = get_daughter(~Circle::new(1.0, 2.0, 3.0),
                         ~Circle::new(1.0, 4.0, 5.0),
                         ~Circle::new(1.0, 6.0, 7.0),
                         ~Circle::new(1.0, 8.0, 9.0), 2.0);

    println(fmt!("You guessed: %?", a));
}

// Complex z =  m.z().multiply(m.k).add(a2.z().multiply(a2.k)).add(a3.z().multiply(a3.k)).multiply(2).subtract(a1.z().multiply(a1.k)).divide(new Complex(k, 0));