use dmntk_feel::nanos_to_string;
use test::Bencher;

#[bench]
fn nanos_to_string_0001(b: &mut Bencher) {
  b.iter(|| nanos_to_string(12000));
}
