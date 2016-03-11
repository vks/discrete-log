#![allow(non_snake_case)]

extern crate num;
extern crate gmp;

use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::cmp::Ordering;
use std::hash::Hash;
use std::convert::From;
use std::collections::HashMap;
use gmp::mpz::Mpz;
use num::{BigInt, Zero, One};
use num::traits::Num;

/// Find the standard representation of a (mod n).
fn normalize<T: Integer>(a: T, n: &T) -> T
    where
      for<'a, 'b> &'a T: Add<&'b T, Output=T>,
      for<'a> &'a T: Add<T, Output=T>,
      for<'a> T: Add<&'a T, Output=T>,
      for<'a, 'b> &'a T: Sub<&'b T, Output=T>,
      for<'a> &'a T: Sub<T, Output=T>,
      for<'a> T: Sub<&'a T, Output=T>,
      for<'a, 'b> &'a T: Mul<&'b T, Output=T>,
      for<'a> &'a T: Mul<T, Output=T>,
      for<'a> T: Mul<&'a T, Output=T>,
      for<'a, 'b> &'a T: Div<&'b T, Output=T>,
      for<'a> &'a T: Div<T, Output=T>,
      for<'a> T: Div<&'a T, Output=T>,
      for<'a, 'b> &'a T: Rem<&'b T, Output=T>,
      for<'a> &'a T: Rem<T, Output=T>,
      for<'a> T: Rem<&'a T, Output=T>,
      for<'a> &'a T: Neg<Output=T>,
{
    let a = a % n;
    match a.cmp(&T::zero()) {
        Ordering::Less => a + n,
        _ => a,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct GcdResult<T> {
    /// Greatest common divisor.
    gcd: T,
    /// Coefficients such that: gcd(a, b) = c1*a + c2*b
    c1: T, c2: T,
}

/// Calculate greatest common divisor and the corresponding coefficients.
fn extended_gcd<T: Integer + Clone>(a: T, b: T) -> GcdResult<T>
    where
      for<'a, 'b> &'a T: Add<&'b T, Output=T>,
      for<'a> &'a T: Add<T, Output=T>,
      for<'a> T: Add<&'a T, Output=T>,
      for<'a, 'b> &'a T: Sub<&'b T, Output=T>,
      for<'a> &'a T: Sub<T, Output=T>,
      for<'a> T: Sub<&'a T, Output=T>,
      for<'a, 'b> &'a T: Mul<&'b T, Output=T>,
      for<'a> &'a T: Mul<T, Output=T>,
      for<'a> T: Mul<&'a T, Output=T>,
      for<'a, 'b> &'a T: Div<&'b T, Output=T>,
      for<'a> &'a T: Div<T, Output=T>,
      for<'a> T: Div<&'a T, Output=T>,
      for<'a, 'b> &'a T: Rem<&'b T, Output=T>,
      for<'a> &'a T: Rem<T, Output=T>,
      for<'a> T: Rem<&'a T, Output=T>,
      for<'a> &'a T: Neg<Output=T>,
{
    // Euclid's extended algorithm
    let (mut s, mut old_s) = (T::zero(), T::one());
    let (mut t, mut old_t) = (T::one(), T::zero());
    let (mut r, mut old_r) = (b, a);

    while r != T::zero() {
        let quotient = &old_r / &r;
        let mut tmp;
        tmp = r.clone(); r = old_r - &quotient * r; old_r = tmp;
        tmp = s.clone(); s = old_s - &quotient * s; old_s = tmp;
        tmp = t.clone(); t = old_t - quotient * t; old_t = tmp;
    }

    let _quotients = (t, s); // == (a, b) / gcd

    GcdResult { gcd: old_r, c1: old_s, c2: old_t }
}

/// Calculate the inverse of a (mod n).
fn inverse<T: Integer + Clone>(a: T, n: &T) -> Option<T>
    where
      for<'a, 'b> &'a T: Add<&'b T, Output=T>,
      for<'a> &'a T: Add<T, Output=T>,
      for<'a> T: Add<&'a T, Output=T>,
      for<'a, 'b> &'a T: Sub<&'b T, Output=T>,
      for<'a> &'a T: Sub<T, Output=T>,
      for<'a> T: Sub<&'a T, Output=T>,
      for<'a, 'b> &'a T: Mul<&'b T, Output=T>,
      for<'a> &'a T: Mul<T, Output=T>,
      for<'a> T: Mul<&'a T, Output=T>,
      for<'a, 'b> &'a T: Div<&'b T, Output=T>,
      for<'a> &'a T: Div<T, Output=T>,
      for<'a> T: Div<&'a T, Output=T>,
      for<'a, 'b> &'a T: Rem<&'b T, Output=T>,
      for<'a> &'a T: Rem<T, Output=T>,
      for<'a> T: Rem<&'a T, Output=T>,
      for<'a> &'a T: Neg<Output=T>,
{
    let GcdResult { gcd, c1: c, c2: _ } = extended_gcd(a, n.clone());
    if gcd == T::one() {
        Some(normalize(c, n))
    } else {
        None
    }
}

/// Calculate base^exp (mod modulus).
fn powm(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    let zero = Integer::zero();
    let one: BigInt = Integer::one();
    let two = &one + &one;
    let mut exp = exp.clone();
    let mut result = one.clone();
    let mut base = base % modulus;
    if exp < zero {
        exp = -exp;
        base = inverse(base, modulus).unwrap();
    }
    while exp > zero {
        if &exp % &two == one {
            result = (result * &base) % modulus;
        }
        exp = exp >> 1;
        base = (&base * &base) % modulus;
    }
    result
}

trait Integer:
    Sized + Eq + Ord
    + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + Neg<Output=Self>
    + From<u64> + Hash
{
    fn zero() -> Self;
    fn one() -> Self;
    fn powm(&self, exp: &Self, modulus: &Self) -> Self;
}

impl Integer for Mpz {
    fn zero() -> Mpz {
        Mpz::zero()
    }

    fn one() -> Mpz {
        Mpz::one()
    }

    fn powm(&self, exp: &Mpz, modulus: &Mpz) -> Mpz {
        self.powm(exp, modulus)
    }
}

impl Integer for BigInt {
    fn zero() -> BigInt {
        Zero::zero()
    }

    fn one() -> BigInt {
        One::one()
    }

    fn powm(&self, exp: &BigInt, modulus: &BigInt) -> BigInt {
        powm(self, exp, modulus)
    }
}

/// Calculate x where g**x = h (mod p).
/// x has to be smaller than 2**x_max_exp.
fn discrete_log<T: Integer>(g: &T, h: &T, p: &T, x_max_exp: usize) -> usize
    where
      for<'a, 'b> &'a T: Add<&'b T, Output=T>,
      for<'a> &'a T: Add<T, Output=T>,
      for<'a> T: Add<&'a T, Output=T>,
      for<'a, 'b> &'a T: Sub<&'b T, Output=T>,
      for<'a> &'a T: Sub<T, Output=T>,
      for<'a> T: Sub<&'a T, Output=T>,
      for<'a, 'b> &'a T: Mul<&'b T, Output=T>,
      for<'a> &'a T: Mul<T, Output=T>,
      for<'a> T: Mul<&'a T, Output=T>,
      for<'a, 'b> &'a T: Div<&'b T, Output=T>,
      for<'a> &'a T: Div<T, Output=T>,
      for<'a> T: Div<&'a T, Output=T>,
      for<'a, 'b> &'a T: Rem<&'b T, Output=T>,
      for<'a> &'a T: Rem<T, Output=T>,
      for<'a> T: Rem<&'a T, Output=T>,
      for<'a> &'a T: Neg<Output=T>,
{
    let B: usize = num::pow(2, x_max_exp / 2);
    let b = T::from(B as u64);

    // Build table.
    let mut table: HashMap<T, usize> = HashMap::with_capacity(B);
    for x1 in 0..B {
        let x1_mpz = T::from(x1 as u64);
        let lhs = h * g.powm(&(-x1_mpz), p);
        let lhs = lhs % p;
        table.insert(lhs, x1);
    }

    // Find collision.
    for x0 in 0..B {
        let rhs = g.powm(&b, p);
        let x0_mpz = T::from(x0 as u64);
        let rhs = rhs.powm(&x0_mpz, p);

        match table.get(&rhs) {
            Some(lhs) => {
                let x1 = *lhs;
                let x = x0 * B + x1;
                return x;
            }
            None => ()
        }
    }
    panic!("`x_max_exp` is too small");
}


#[test]
fn test_powm_bigint() {
    assert_eq!(powm(&From::from(4), &From::from(13), &From::from(497)),
               From::from(445));
}

#[test]
fn test_powm_mpz() {
    assert_eq!(Mpz::from(4).powm(&Mpz::from(13), &Mpz::from(497)),
               Mpz::from(445));
}

#[test]
fn test_discrete_log_mpz() {
    let g = Mpz::from(2);
    let p = Mpz::from(11);
    for h in 1..11 {
        let h = Mpz::from(h);
        let x = discrete_log(&g, &h, &p, 10);
        assert_eq!(g.pow(x as u32) % &p, h);
    }
}

#[test]
fn test_discrete_log_bigint() {
    let g = BigInt::from(2);
    let p = BigInt::from(11);
    for h in 1..11 {
        let h = BigInt::from(h);
        let x = discrete_log(&g, &h, &p, 10);
        assert_eq!(num::pow(g.clone(), x) % &p, h);
    }
}

#[cfg(not(test))]
fn main() {
    const P: &'static str = "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084171";
    const G: &'static str = "11717829880366207009516117596335367088558084999998952205599979459063929499736583746670572176471460312928594829675428279466566527115212748467589894601965568";
    const H: &'static str = "3239475104050450443565264378728065788649097520952449527834792452971981976143292558073856937958553180532878928001494706097394108577585732452307673444020333";

    let arg = std::env::args().skip(1).next()
        .unwrap_or("bigint".into());

    let x = if arg == "mpz" {
        let p = Mpz::from_str_radix(P, 10).unwrap();
        let g = Mpz::from_str_radix(G, 10).unwrap();
        let h = Mpz::from_str_radix(H, 10).unwrap();
        discrete_log(&g, &h, &p, 40)

    } else if arg == "bigint" {
        let p = BigInt::from_str_radix(P, 10).unwrap();
        let g = BigInt::from_str_radix(G, 10).unwrap();
        let h = BigInt::from_str_radix(H, 10).unwrap();
        discrete_log(&g, &h, &p, 40)

    } else {
        panic!("unknown argument!");
    };

    assert_eq!(x, 375374217830);
    println!("{}", x);
}
