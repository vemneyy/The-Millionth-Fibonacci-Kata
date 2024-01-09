use num::bigint::BigInt;
use num::traits::{Zero, One};

fn fib_matrix_mult(a: &[[BigInt; 2]; 2], b: &[[BigInt; 2]; 2]) -> [[BigInt; 2]; 2] {
    let mut result = [[BigInt::zero(), BigInt::zero()], [BigInt::zero(), BigInt::zero()]];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += &a[i][k] * &b[k][j];
            }
        }
    }
    result
}

fn fib_matrix_pow(mut base: [[BigInt; 2]; 2], mut exponent: u32) -> [[BigInt; 2]; 2] {
    let mut result = [[BigInt::one(), BigInt::zero()], [BigInt::zero(), BigInt::one()]];
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = fib_matrix_mult(&result, &base);
        }
        base = fib_matrix_mult(&base, &base);
        exponent /= 2;
    }
    result
}

fn fib(n: i32) -> BigInt {
    if n == 0 {
        return BigInt::zero();
    }

    let positive_n = n.abs() as u32;
    let matrix = [[BigInt::one(), BigInt::one()], [BigInt::one(), BigInt::zero()]];

    let result_matrix = fib_matrix_pow(matrix, positive_n - 1);
    let fib_n = &result_matrix[0][0];

    if n < 0 && n % 2 == 0 {
        -fib_n.clone()
    } else {
        fib_n.clone()
    }
}
