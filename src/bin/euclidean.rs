
struct Vector {
    values: Vec<f64>
}

struct Distance {
    value: f64
}

impl Vector {
    fn new(values: Vec<f64>) -> Vector {
        Vector { values }
    }
}
impl Vector {
    fn euclidean_distance(&self, other: &Vector) -> Result<Distance, &'static str> {

        if self.values.len() != other.values.len() {
            return Err("Vectors must be of the same length");
        }

        Ok (Distance {
            value: self.values.iter()
                .zip(other.values.iter())
                .map(|(x, y)| (x-y).powi(2))
                .sum()

        })

    }
}


fn main () {

    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![3.5, 4.8, 1.0]);
    println!("{}", v1.euclidean_distance(&v2).unwrap().value);

}