#![cfg_attr(feature = "unstable", feature(test))]

use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::pow::Pow;

fn b(v: u32) -> BigUint {
    BigUint::from(v)
}

fn ackermann(m: u32, n: u32) -> BigUint {
    fn work(m: BigUint, n: BigUint) -> BigUint {
        match m.to_u32().unwrap() {
            0 => n + b(1),
            1 => n + b(2),
            2 => b(2) * n + b(3),
            3 => {
                let exp = (n + b(3)).to_u128().unwrap();
                b(2).pow(exp) - b(3)
            },
            _ => {
                if m == b(0) {
                    n + b(1)
                } else if n == b(0) {
                    work(m - b(1), b(1))
                } else {
                    work(m.clone() - b(1), work(m, n - b(1)))
                }
            }
        }
    }
    work(b(m), b(n))
}

fn main() {
    println!("{}", ackermann(1,0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use num_bigint::BigUint;

    #[test]
    fn a0(){
        for i in 0..10{
            assert_eq!(ackermann(0, i), b(i + 1));
        }
    }
    #[test]
    fn a1(){
        for i in 0..10{
            assert_eq!(ackermann(1, i), b(i + 2));
        }
    }
    #[test]
    fn a2(){
        for i in 0..10{
            assert_eq!(ackermann(2, i), b(2 * i + 3));
        }
    }
    #[test]
    fn a3(){
        for i in 0..10{
            let ui: u32 = i.try_into().unwrap();
            assert_eq!(ackermann(3, i), b((2_i32.pow(ui + 3) - 3).try_into().unwrap()));
        }
    }
    #[test]
    fn a4(){
        assert_eq!(ackermann(4, 0), b(13));
        assert_eq!(ackermann(4, 1), b(65533));

    }
    #[test]
    fn conv_from_u32_biguint(){
        assert_eq!(b(5), BigUint::new(vec![5]));
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench{
    extern crate test;

    use super::*;

    use self::test::Bencher;

    #[bench]
    fn naive_bench(b: &mut Bencher){
        b.iter(|| naive_achermann(3, 7))
    }
    fn bituint_bench(b: &mut Bencher){
        b.iter(|| bituint_achermann(3, 7))
    }
    fn memo_bench(b: &mut Bencher){
        b.iter(|| achermann(3, 7))
    }
}
