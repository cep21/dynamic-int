extern crate num;
use std::ops::{Add,Mul,Shr};
use std::num::Int;

use self::num::bigint::BigUint;
use self::num::Zero;
use self::num::Integer;
use self::num::bigint::ToBigUint;

pub enum DynamicChangingU64 {
    Regular(u64),
    Struct(BigUint),
}

impl DynamicChangingU64 {
    pub fn new(val: u64) -> DynamicChangingU64 {
        DynamicChangingU64::Regular(val)
    }
    pub fn is_zero(&self) -> bool {
        match *self {
            DynamicChangingU64::Regular(ref v) => {
                *v == 0
            }
            DynamicChangingU64::Struct(ref v) => {
                v.is_zero()
            }
        }
    }
    pub fn is_odd(&self) -> bool {
        match *self {
            DynamicChangingU64::Regular(ref v) => {
                *v % 2 == 1
            }
            DynamicChangingU64::Struct(ref v) => {
                v.is_odd()
            }
        }
    }
    pub fn bits(&self) -> usize {
        match *self {
            DynamicChangingU64::Regular(ref v) => {
                v.to_biguint().unwrap().bits()
            }
            DynamicChangingU64::Struct(ref v) => {
                v.bits()
            }
        }
    }
}

impl Add<u64> for DynamicChangingU64 {
    type Output = DynamicChangingU64;
    fn add(self, other: u64) -> DynamicChangingU64 {
        match self {
            DynamicChangingU64::Regular(ref v) => {
                match v.checked_add(other as u64) {
                    Some(v2) => DynamicChangingU64::Regular(v2),
                    None => DynamicChangingU64::Struct(v.to_biguint().unwrap() +
                                                       other.to_biguint().unwrap())
                }
            }
            DynamicChangingU64::Struct(ref v) => {
                DynamicChangingU64::Struct(v + other.to_biguint().unwrap())
            }
        }
    }
}

impl Mul<u64> for DynamicChangingU64 {
    type Output = DynamicChangingU64;
    fn mul(self, other: u64) -> DynamicChangingU64 {
        match self {
            DynamicChangingU64::Regular(ref v) => {
                match v.checked_mul(other as u64) {
                    Some(v2) => DynamicChangingU64::Regular(v2),
                    None => {
                        println!("switch over!");
                        DynamicChangingU64::Struct(v.to_biguint().unwrap() *
                                                       other.to_biguint().unwrap())
                    }
                }
            }
            DynamicChangingU64::Struct(ref v) => {
                DynamicChangingU64::Struct(v * other.to_biguint().unwrap())
            }
        }
    }
}

impl Shr<usize> for DynamicChangingU64 {
    type Output = DynamicChangingU64;
    fn shr(self, other: usize) -> DynamicChangingU64 {
        match self {
            DynamicChangingU64::Regular(ref v) => {
                DynamicChangingU64::Regular(*v >> other)
            }
            DynamicChangingU64::Struct(ref v) => {
                DynamicChangingU64::Struct(v >> other)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    extern crate num;
    use DynamicChangingU64;

    use self::num::Integer;
    use self::test::Bencher;
    use self::num::bigint::ToBigUint;

    #[bench]
    fn bench_dynamic_mul(b: &mut Bencher) {
        b.iter(|| {
            let a = DynamicChangingU64::new(123);
            let b = 123;
            let c = a * b;
            assert_eq!(true, c.is_odd());
        });
    }

    #[bench]
    fn bench_normal_mul(b: &mut Bencher) {
        b.iter(|| {
            let a = 123u64;
            let b = 123u64;
            let c = a * b;
            assert_eq!(1, c % 2);
        });
    }

    #[bench]
    fn bench_bigint_mul(b: &mut Bencher) {
        b.iter(|| {
            let a = 123u64.to_biguint().unwrap();
            let b = 123u64.to_biguint().unwrap();
            let c = a * b;
            assert_eq!(true, c.is_odd());
        });
    }
}
