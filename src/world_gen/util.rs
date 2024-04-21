use crate::world_gen::prelude::*;

pub fn to_1d(x: usize, y: usize, z: usize) -> usize {
    x + y * X_SIZE + z * Y_SIZE * X_SIZE
}

pub fn index(arr: [f64; TOTAL_SIZE], x: usize, y: usize, z: usize) -> f64 {
    arr[to_1d(x, y, z)]
}

pub fn to_3d(n: usize) -> (usize, usize, usize) {
    let x: usize = n % X_SIZE;
    let y: usize = (n / (X_SIZE)) % Y_SIZE;
    let z: usize = n / (X_SIZE * Y_SIZE);
    (x, y, z)
}

pub fn index_reverse(n: usize) -> Index3D {
    let index = to_3d(n);
    Index3D {
        x: index.0,
        y: index.1,
        z: index.2,
    }
}