
#[derive(Debug)]
struct Matrix<const M: usize , const N: usize> {
    data: [[f64; N]; M] //N: columns, M: rows
}

struct Vector<const N: usize> {
    data: [f64; N]
}

impl <const M: usize, const N: usize> Matrix<M, N> {
    fn mul(&self, v: &Vector<N>) -> Vector<N> {
        let mut result = [0.0; N];

        for i in 0..M {
            for j in 0..N {
                result[j] += self.data[i][j] * v.data[j]
            }
        };

        Vector { data: result }
    }
}
fn main () {
    let m1: Matrix<3, 2> = Matrix{ data: [[2.0, 3.0], [1.0, 2.0], [3.0, 6.0]] };
    let v1: Vector<2> = Vector{ data: [1.0, 2.0] };
    let ov1 = m1.mul(&v1);
    println!("{:?}", ov1.data);
}