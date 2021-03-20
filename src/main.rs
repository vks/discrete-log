#![allow(non_snake_case)]

use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::cmp::Ordering;
use std::hash::Hash;
use std::convert::From;
use std::collections::HashMap;
#[cfg(feature = "gmp")]
use gmp::mpz::Mpz;
use num_bigint::BigInt;
use num_traits::{Zero, One};
use ramp::Int;
#[cfg(feature = "rug")]
use rug::Integer as RugInt;

/// Find the standard representation of a (mod n).
fn normalize<T: Integer>(a: T, n: &T) -> T
    where for<'a> &'a T: IntegerOps<T>
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
fn extended_gcd<T: Integer>(a: T, b: T) -> GcdResult<T>
    where for<'a> &'a T: IntegerOps<T>
{
    // Euclid's extended algorithm
    let (mut s, mut old_s) = (T::zero(), T::one());
    let (mut t, mut old_t) = (T::one(), T::zero());
    let (mut r, mut old_r) = (b, a);

    while r != T::zero() {
        let quotient = &old_r / &r;
        old_r = old_r - &quotient * &r; std::mem::swap(&mut old_r, &mut r);
        old_s = old_s - &quotient * &s; std::mem::swap(&mut old_s, &mut s);
        old_t = old_t - quotient * &t; std::mem::swap(&mut old_t, &mut t);
    }

    let _quotients = (t, s); // == (a, b) / gcd

    GcdResult { gcd: old_r, c1: old_s, c2: old_t }
}

/// Calculate the inverse of a (mod n).
fn inverse<T: Integer + Clone>(a: T, n: &T) -> Option<T>
    where for<'a> &'a T: IntegerOps<T>
{
    let GcdResult { gcd, c1: c, .. } = extended_gcd(a, n.clone());
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

trait IntegerValOps<RHS, Output>: Sized
    + Add<RHS, Output = Output> + Sub<RHS, Output = Output>
    + Mul<RHS, Output = Output> + Div<RHS, Output = Output>
    + Rem<RHS, Output = Output> + Neg<Output = Output> {
}

impl<RHS, Output, T> IntegerValOps<RHS, Output> for T
    where T: Sized
    + Add<RHS, Output = Output> + Sub<RHS, Output = Output>
    + Mul<RHS, Output = Output> + Div<RHS, Output = Output>
    + Rem<RHS, Output = Output> + Neg<Output = Output>
{
}

trait IntegerOps<Base>: IntegerValOps<Base, Base>
    + for<'a> IntegerValOps<&'a Base, Base> {
}

impl<Base, T> IntegerOps<Base> for T
    where T: IntegerValOps<Base, Base>
    + for<'a> IntegerValOps<&'a Base, Base>
{
}

trait Integer:
    Sized + Eq + Ord + Clone
    + IntegerOps<Self>
    + From<u64> + Hash
{
    fn zero() -> Self;
    fn one() -> Self;
    fn invert(&self, modulus: &Self) -> Option<Self>;
    fn powm(&self, exp: &Self, modulus: &Self) -> Self;
}

#[cfg(feature = "gmp")]
impl Integer for Mpz {
    fn zero() -> Mpz {
        Mpz::zero()
    }

    fn one() -> Mpz {
        Mpz::one()
    }

    fn invert(&self, modulus: &Mpz) -> Option<Mpz> {
        self.invert(modulus)
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

    fn invert(&self, modulus: &BigInt) -> Option<BigInt> {
        inverse(self.clone(), modulus)
    }

    fn powm(&self, exp: &BigInt, modulus: &BigInt) -> BigInt {
        powm(self, exp, modulus)
    }
}

impl Integer for Int {
    fn zero() -> Int {
        Zero::zero()
    }

    fn one() -> Int {
        One::one()
    }

    fn invert(&self, modulus: &Int) -> Option<Int> {
        inverse(self.clone(), modulus)
    }

    fn powm(&self, exp: &Int, modulus: &Int) -> Int {
        self.pow_mod(exp, modulus)
    }
}

#[cfg(feature = "rug")]
impl Integer for RugInt {
    fn zero() -> RugInt {
        RugInt::new()
    }

    fn one() -> RugInt {
        RugInt::from(1)
    }

    fn invert(&self, modulus: &RugInt) -> Option<RugInt> {
        self.invert_ref(modulus).map(RugInt::from)
    }

    fn powm(&self, exp: &RugInt, modulus: &RugInt) -> RugInt {
        self.pow_mod_ref(exp, modulus).map(RugInt::from).unwrap()
    }
}

/// Calculate `x` where `g**x = h (mod p)`.
/// `x` has to be smaller than `2**x_max_exp`.
fn discrete_log<'a, T>(g: &T, h: &'a T, p: &'a T, x_max_exp: usize) -> usize
    where &'a T: Rem,
    T: Integer + From<<&'a T as Rem>::Output>,
{
    let B: usize = 1 << (x_max_exp / 2);
    let b = T::from(B as u64);

    // Build table.
    let mut table: HashMap<T, usize> = HashMap::with_capacity(B);
    let g_inv = g.invert(p).unwrap();
    let mut lhs = T::from(h % p);
    for x1 in 0..B {
        if x1 > 0 {
            lhs = (lhs * &g_inv) % p;
        }
        table.insert(lhs.clone(), x1);
    }

    // Find collision.
    let g_b = g.powm(&b, p);
    let mut rhs = T::one();
    for x0 in 0..B {
        if x0 > 0 {
            rhs = (rhs * &g_b) % p;
        }

        if let Some(&x1) = table.get(&rhs) {
            return x0 * B + x1;
        }
    }
    panic!("`x_max_exp` is too small");
}


#[test]
fn test_powm_num_bigint() {
    assert_eq!(powm(&From::from(4), &From::from(13), &From::from(497)),
               From::from(445));
}

#[cfg(feature = "gmp")]
#[test]
fn test_powm_gmp() {
    assert_eq!(Mpz::from(4).powm(&Mpz::from(13), &Mpz::from(497)),
               Mpz::from(445));
}

#[test]
fn test_powm_ramp() {
    assert_eq!(Int::from(4).powm(&Int::from(13), &Int::from(497)),
               Int::from(445));
}

#[cfg(feature = "rug")]
#[test]
fn test_powm_rug() {
    assert_eq!(RugInt::from(4).powm(&RugInt::from(13), &RugInt::from(497)),
               445);
}

#[cfg(feature = "gmp")]
#[test]
fn test_discrete_log_gmp() {
    let g = Mpz::from(2);
    let p = Mpz::from(11);
    for h in 1..11 {
        let h = Mpz::from(h);
        let x = discrete_log(&g, &h, &p, 10);
        assert_eq!(g.pow(x as u32) % &p, h);
    }
}

#[test]
fn test_discrete_log_num_bigint() {
    let g = BigInt::from(2);
    let p = BigInt::from(11);
    for h in 1..11 {
        let h = BigInt::from(h);
        let x = discrete_log(&g, &h, &p, 10);
        assert_eq!(num_traits::pow(g.clone(), x) % &p, h);
    }
}

#[test]
fn test_discrete_log_ramp() {
    let g = Int::from(2);
    let p = Int::from(11);
    for h in 1..11 {
        let h = Int::from(h);
        let x = discrete_log(&g, &h, &p, 10);
        assert_eq!(num_traits::pow(g.clone(), x) % &p, h);
    }
}

#[cfg(feature = "rug")]
#[test]
fn test_discrete_log_rug() {
    use rug::ops::Pow;
    let g = RugInt::from(2);
    let p = RugInt::from(11);
    for h in 1..11 {
        let h = RugInt::from(h);
        let x = discrete_log(&g, &h, &p, 10);
        assert_eq!(RugInt::from((&g).pow(x as u32)) % &p, h);
    }
}

#[cfg(not(test))]
fn main() {
    use num_traits::Num;

    const P: &str = "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084171";
    const G: &str = "11717829880366207009516117596335367088558084999998952205599979459063929499736583746670572176471460312928594829675428279466566527115212748467589894601965568";
    const H: &str = "3239475104050450443565264378728065788649097520952449527834792452971981976143292558073856937958553180532878928001494706097394108577585732452307673444020333";

    let arg = std::env::args().nth(1).unwrap_or("bigint".into());

    let x = match arg.as_ref() {
        #[cfg(feature = "gmp")]
        "gmp" => {
            let p = Mpz::from_str_radix(P, 10).unwrap();
            let g = Mpz::from_str_radix(G, 10).unwrap();
            let h = Mpz::from_str_radix(H, 10).unwrap();
            discrete_log(&g, &h, &p, 40)
        },
        "num_bigint" => {
            let p = BigInt::from_str_radix(P, 10).unwrap();
            let g = BigInt::from_str_radix(G, 10).unwrap();
            let h = BigInt::from_str_radix(H, 10).unwrap();
            discrete_log(&g, &h, &p, 40)
        },
        "ramp" => {
            let p = Int::from_str_radix(P, 10).unwrap();
            let g = Int::from_str_radix(G, 10).unwrap();
            let h = Int::from_str_radix(H, 10).unwrap();
            discrete_log(&g, &h, &p, 40)
        },
        #[cfg(feature = "rug")]
        "rug" => {
            let p = RugInt::from_str_radix(P, 10).unwrap();
            let g = RugInt::from_str_radix(G, 10).unwrap();
            let h = RugInt::from_str_radix(H, 10).unwrap();
            discrete_log(&g, &h, &p, 40)
        },
        _ => panic!("unknown argument!"),
    };

    assert_eq!(x, 375374217830);
    println!("{}", x);
}
