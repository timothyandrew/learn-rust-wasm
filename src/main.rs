use fixedbitset::FixedBitSet;

fn main() {
  let mut bitset = FixedBitSet::with_capacity(64);
  bitset.set(0, true);
  bitset.set(3, true);
  bitset.set(33, true);
  println!("{:?}", bitset);
  println!("{:?}", bitset.as_slice());
}