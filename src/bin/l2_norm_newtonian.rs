mod correlation;

struct Vector {
    values: Vec<f64>,
}

struct Scalar {
    value: f64
}

struct UnitVector {
    values: Vec<f64>
}

trait VectorLike {
    fn values(&self) -> &[f64];
}

impl VectorLike for Vector {
    fn values(&self) -> &[f64] {
        &self.values
    }
}

impl VectorLike for UnitVector {
    fn values(&self) -> &[f64] {
        &self.values
    }
}

fn newton_sqrt(dot_prod: &Scalar) -> Result<Scalar, &'static str> {
    let tol = 1e-6;
    let x = dot_prod.value;
    let mut s = x;
    if dot_prod.value < 0.0 {
        return Err("Can not determine the square root of a negative number.");
    }


    if x == 0.0 {
        return Ok ( Scalar { value: 0.0 } )
        } else {
            loop {
                let next_s: f64 = (s + (x / s) ) / 2.0;
                if (next_s - s).abs() <= tol {
                    return Ok ( Scalar { value: next_s } )
                } else {
                    s = next_s;
                }
            }
        }
}

fn dot<A: VectorLike, B: VectorLike>(a: &A, b: &B) -> Result<Scalar, &'static str> {
    if a.values().len() != b.values().len() {
        return Err("Vectors are not the same length");
    }
    Ok( Scalar {
        value: a.values().iter()
            .zip(b.values().iter())
            .map(|(x, y)| x * y)
            .sum()
    })
}


impl Vector{
    fn l2_norm(&self) -> Scalar {
        let var = Scalar { value: dot(self, self).unwrap().value };
        Scalar { value: newton_sqrt(&var).unwrap().value }
    }
}


impl Vector{
    fn create_unitvector(&self) -> UnitVector {
        let l2 = self.l2_norm().value;
        UnitVector {
            values:
            self.values().iter()
            .map(|x| x / l2)
            .collect()
        }
    }
}


impl UnitVector{
    fn cosine_similarity(&self, vec2: &UnitVector) -> Scalar {
        let cos_sim = dot(self, vec2).unwrap().value;
        Scalar { value: cos_sim }
    }
}

fn main() {
    let v = Vector { values: vec![1.0, 2.0, 3.0] };
    let v2 = Vector { values: vec![2.0, 3.0, 4.0] };
    let uv1 = v.create_unitvector();
    let uv2 = v2.create_unitvector();
    println!("{}", uv1.cosine_similarity(&uv2).value);
}