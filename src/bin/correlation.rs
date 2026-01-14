
struct Vector {
    values: Vec<f64>
}

struct Variance {
    value: f64
}

struct Covariance {
    value: f64
}

struct Correlation {
    value: f64
}

struct Mean {
    value: f64
}


impl Vector {
    fn mean(&self) -> Mean {
        let l = self.values.len() as f64;
        let sum: f64 = self.values.iter().sum();
        Mean { value: sum / l }
    }
}

impl Vector {
    fn variance(&self) -> Variance {

        if self.values.len() < 2 {
            panic!("Variance is not defined for vectors of length < 2");
        }


        let m = self.mean().value;
        let n = self.values.len() as f64;
        let s: f64 = self.values.iter()
            .map(|x| (x - m ).powi(2))
            .sum();

        if s == 0.0 { panic!("Variance is not defined for vectors with all equal values"); }

        Variance { value: s / (n - 1.0)  }
    }
}

impl Vector {
    fn covariance(&self, other: &Vector) -> Covariance {

        if self.values.len() < 2 {
            panic!("Covariance is not defined for vectors of length < 2");
        }

        if self.values.len() != other.values.len() {
            panic!("Vectors must have the same length");
        }

        let mx = self.mean().value;
        let my = other.mean().value;
        let n = self.values.len() as f64;

        let dot: f64 = self.values.iter()
            .zip(other.values.iter())
            .map(|(x, y)| (x - mx) * (y-my) )
            .sum();

        Covariance { value: dot / (n - 1.0) }
    }
}

impl Vector {
    fn correlation(&self, other: &Vector) -> Result<Correlation, &'static str> {
        if (self.values.len() == 0) || (other.values.len() == 0) {
            return Err("Vectors must have at least one element");
        } else if self.values.len() != other.values.len() {
            return Err("Vectors must have the same length");
        } else if (self.variance().value == 0.0) || (other.variance().value == 0.0) {
            return Err("Vectors must have variance > 0");
        } else if (self.values.len() < 2) || (other.values.len() < 2) {
            return Err("Correlation is not defined for vectors of length < 2");
        } else {
            Ok(
                Correlation {
                    value:
                    self.covariance(&other).value
                        / ((self.variance().value * other.variance().value).sqrt())
                }
            )
        }
    }
}


fn main () {
    let v1 = Vector { values: vec![1.0, 2.0, 3.0] };
    let v2 = Vector { values: vec![2.0, 4.0, 6.0] };
    println!("{}", v1.correlation(&v2).unwrap().value);
}
