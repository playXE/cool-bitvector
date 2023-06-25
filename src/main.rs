use cool_bitvector::BitVector;

fn main() {
    let mut bvec = BitVector::with_capacity(32);

    bvec.set(16, true);
    bvec.set(67, true);

    println!("{}", bvec.get(16));
    println!("{}", bvec.get(67));
}
