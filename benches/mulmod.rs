extern crate num_traits;
extern crate num_bigint;
extern crate gmp;
extern crate ramp;
extern crate rug;
#[macro_use]
extern crate criterion;

use gmp::mpz::Mpz;
use num_traits::Num;
use num_bigint::BigInt;
use ramp::Int;
use rug::Integer as RugInt;
use criterion::Criterion;

const P: &'static str = "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084171";
const G: &'static str = "11717829880366207009516117596335367088558084999998952205599979459063929499736583746670572176471460312928594829675428279466566527115212748467589894601965568";
const H: &'static str = "3239475104050450443565264378728065788649097520952449527834792452971981976143292558073856937958553180532878928001494706097394108577585732452307673444020333";

fn mulmod_gmp(c: &mut Criterion) {
    let p = Mpz::from_str_radix(P, 10).unwrap();
    let g = Mpz::from_str_radix(G, 10).unwrap();
    let h = Mpz::from_str_radix(H, 10).unwrap();
    c.bench_function("mulmod_gmp", move |b| b.iter(|| (&g * &h) % &p));
}

fn mulmod_num_bigint(c: &mut Criterion) {
    let p = BigInt::from_str_radix(P, 10).unwrap();
    let g = BigInt::from_str_radix(G, 10).unwrap();
    let h = BigInt::from_str_radix(H, 10).unwrap();
    c.bench_function("mulmod_num", move |b| b.iter(|| (&g * &h) % &p));
}

fn mulmod_ramp(c: &mut Criterion) {
    let p = Int::from_str_radix(P, 10).unwrap();
    let g = Int::from_str_radix(G, 10).unwrap();
    let h = Int::from_str_radix(H, 10).unwrap();
    c.bench_function("mulmod_ramp", move |b| b.iter(|| (&g * &h) % &p));
}

fn mulmod_rug(c: &mut Criterion) {
    let p = RugInt::from_str_radix(P, 10).unwrap();
    let g = RugInt::from_str_radix(G, 10).unwrap();
    let h = RugInt::from_str_radix(H, 10).unwrap();
    c.bench_function("mulmod_rug", move |b| b.iter(|| RugInt::from(&g * &h) % &p));
}

criterion_group!(mulmod, mulmod_gmp, mulmod_num_bigint, mulmod_ramp, mulmod_rug);
criterion_main!(mulmod);
