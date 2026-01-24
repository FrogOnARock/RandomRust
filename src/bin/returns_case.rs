use std::marker::PhantomData;


struct Price; // Unit Type
struct Return; // Unit Type
struct Directional; // Geometry type
struct Metric; //Geometry type

struct RawVector<Unit> {
    values: Vec<f64>,
    _unit: PhantomData<Unit>
}

struct ComparableVector<Geometry> {
    values: Vec<f64>,
    _geom: PhantomData<Geometry>
}


impl<U> ComparableVector<U> {
    fn new(values: Vec<f64>) -> Self {
        ComparableVector {
            values, _geom: PhantomData
        }
    }
}

impl<U> RawVector<U> {
    fn new(values: Vec<f64>) -> Self {
        RawVector { values, _unit: PhantomData}
    }
}

fn dot(v1: &ComparableVector<Directional>, v2: &ComparableVector<Directional>) -> Result<f64, &'static str> {
    if v1.values.len() != v2.values.len() {
        return Err("Vectors must be of the same length");
    }

    Ok (
        v1.values.iter()
            .zip(v2.values.iter())
            .map(|(x, y)| x * y)
            .sum()
    )

}

impl<Directional> ComparableVector<Directional> {
    fn norm(&self) -> f64 {

        self.values.iter()
        .map(|x| x.powi(2))
        .sum::<f64>()
        .sqrt()

    }
}

impl<Price> RawVector<Price> {
    fn log_returns(&self) -> ComparableVector<Directional> {

        ComparableVector::<Directional>::new (
            self.values
                .windows(2)
                .map(|w| (w[1] / w[0]).ln())
                .collect()
        )
    }
}



fn cosine_similarity(v1: &ComparableVector<Directional>, v2: &ComparableVector<Directional>) -> Result<f64, &'static str> {

    let eps: f64 = std::f64::EPSILON;
    let min: f64 = 1e-14;
    let denom: f64 = v1.norm() * v2.norm();
    let num: f64 = dot(&v1, &v2)?;

    if denom < f64::max(eps * denom, min) {
        return Err("Floating point arithmetic fails -> exploding noise.")
    }

    if v1.values.len() == 0 || v2.values.len() == 0 {
        return Err("One of the vectors lengths is zero")
    }

    if v1.values.len() != v2.values.len() {
        return Err("Vectors need to be of the same length")
    }

    Ok (
        num / denom
    )
}

fn euclidean_distance(v1: &ComparableVector<Metric>, v2: &ComparableVector<Metric>) -> f64 {

    v1.values.iter()
        .zip(v2.values.iter())
        .map(|(x, y)| (x-y).powi(2))
        .sum::<f64>()
        .sqrt()

}



fn main() {
    let v = RawVector::<Price>::new(vec![10.0, 20.0, 30.0]);
    let v2 = RawVector::<Price>::new(vec![100.0, 200.0, 300.0]);

    let log_v1 = v.log_returns();
    let log_v2 = v2.log_returns();

    let m_v = ComparableVector::<Metric>::new(v.values);
    let m_v2 = ComparableVector::<Metric>::new(v2.values);

    println!("Cosine Similarity of log returns: {}", cosine_similarity(&log_v1, &log_v2).unwrap());
    println!("Euclidean distance of prices: {}", euclidean_distance(&m_v, &m_v2));

}
