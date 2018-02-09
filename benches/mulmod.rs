#![feature(test)]

extern crate num_traits;
extern crate num_bigint;
extern crate gmp;
extern crate ramp;
extern crate rug;
extern crate test;

use gmp::mpz::Mpz;
use num_traits::Num;
use num_bigint::BigInt;
use ramp::Int;
use rug::Integer as RugInt;
use test::Bencher;

const P: &'static str = "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084171";
const G: &'static str = "11717829880366207009516117596335367088558084999998952205599979459063929499736583746670572176471460312928594829675428279466566527115212748467589894601965568";
const H: &'static str = "3239475104050450443565264378728065788649097520952449527834792452971981976143292558073856937958553180532878928001494706097394108577585732452307673444020333";

#[bench]
fn mpz_mulmod(b: &mut Bencher) {
    let p = Mpz::from_str_radix(P, 10).unwrap();
    let g = Mpz::from_str_radix(G, 10).unwrap();
    let h = Mpz::from_str_radix(H, 10).unwrap();
    b.iter(|| (&g * &h) % &p);
}

#[bench]
fn bigint_mulmod(b: &mut Bencher) {
    let p = BigInt::from_str_radix(P, 10).unwrap();
    let g = BigInt::from_str_radix(G, 10).unwrap();
    let h = BigInt::from_str_radix(H, 10).unwrap();
    b.iter(|| (&g * &h) % &p);
}

#[bench]
fn int_mulmod(b: &mut Bencher) {
    let p = Int::from_str_radix(P, 10).unwrap();
    let g = Int::from_str_radix(G, 10).unwrap();
    let h = Int::from_str_radix(H, 10).unwrap();
    b.iter(|| (&g * &h) % &p);
}

#[bench]
fn rugint_mulmod(b: &mut Bencher) {
    let p = RugInt::from_str_radix(P, 10).unwrap();
    let g = RugInt::from_str_radix(G, 10).unwrap();
    let h = RugInt::from_str_radix(H, 10).unwrap();
    b.iter(|| RugInt::from(&g * &h) % &p);
}
