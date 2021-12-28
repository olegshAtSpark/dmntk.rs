#![feature(test)]

extern crate test;

use dmntk_feel_number::dec::{dec_add, dec_compare, dec_divide, dec_from_bcd, dec_from_i32, dec_from_string, dec_multiply, dec_subtract};
use test::Bencher;

#[bench]
fn bench_dec_from_i32(b: &mut Bencher) {
  b.iter(|| dec_from_i32(i32::MAX))
}

#[bench]
fn bench_dec_from_string(b: &mut Bencher) {
  b.iter(|| dec_from_string("123456.789"))
}

#[bench]
fn bench_dec_from_bcd(b: &mut Bencher) {
  b.iter(|| {
    dec_from_bcd(&[
      8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ])
  })
}

#[bench]
fn bench_dec_add(b: &mut Bencher) {
  let x = dec_from_string("0.1");
  let y = dec_from_string("0.3");
  b.iter(|| dec_add(&x, &y))
}

#[bench]
fn bench_dec_add_1(b: &mut Bencher) {
  let x = dec_from_string("123.45");
  let y = dec_from_string("0.3847847");
  b.iter(|| dec_add(&x, &y))
}

#[bench]
fn bench_dec_sub(b: &mut Bencher) {
  let x = dec_from_string("0.1");
  let y = dec_from_string("0.3");
  b.iter(|| dec_subtract(&x, &y))
}

#[bench]
fn bench_dec_div(b: &mut Bencher) {
  let x = dec_from_string("0.1");
  let y = dec_from_string("0.3");
  b.iter(|| dec_divide(&x, &y))
}

#[bench]
fn bench_dec_mul(b: &mut Bencher) {
  let x = dec_from_string("0.1");
  let y = dec_from_string("0.3");
  b.iter(|| dec_multiply(&x, &y))
}

#[bench]
fn bench_dec_compare_total(b: &mut Bencher) {
  let x = dec_from_string("0.1");
  let y = dec_from_string("0.3");
  b.iter(|| dec_compare(&x, &y));
}
