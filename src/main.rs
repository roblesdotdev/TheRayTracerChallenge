mod tuple;

use tuple::{Tuple, equal};

fn main() {

    let point = Tuple::point(4.3, -4.2, 3.1);
    let vector = Tuple::vector(4.3, -4.2, 3.1);

    println!("{:?}", point);
    println!("{:?}", vector);

    println!("{}", equal(3.0001, 3.0001));

    println!("Is point: {}", point.is_point());
    println!("Is vector: {}", vector.is_vector());
}
