use crate::BitSet;

#[test]
#[should_panic]
fn empty_bitset() {
    BitSet::with_capacity(0);
}

#[test]
fn simple() {
    let bs = BitSet::with_capacity(31);
    assert_eq!(bs.num_bits, 31);
    assert_eq!(bs.chunks.len(), 1);
}

#[test]
fn boundary() {
    let bs = BitSet::with_capacity(32);
    assert_eq!(bs.num_bits, 32);
    assert_eq!(bs.chunks.len(), 1);
}

#[test]
fn boundary_next() {
    let bs = BitSet::with_capacity(33);
    assert_eq!(bs.num_bits, 33);
    assert_eq!(bs.chunks.len(), 2);
}

#[test]
fn some_mask() {
    assert_eq!(BitSet::last_mask(3), 0b111);
}

#[test]
fn boundary_mask() {
    assert_eq!(BitSet::last_mask(32), u32::MAX);
}

#[test]
fn overlap_mask() {
    assert_eq!(BitSet::last_mask(34), 0b11);
}

#[test]
fn double_boundary_mask() {
    assert_eq!(BitSet::last_mask(64), u32::MAX);
}

#[test]
fn from_bits() {
    let bs = BitSet::from_bits(vec![0b111], 3);
    assert_eq!(bs.num_bits, 3);
    assert_eq!(bs.chunks.len(), 1);
    assert_eq!(bs.chunks[0], 0b111);
}

#[test]
#[should_panic]
fn from_bits_empty() {
    BitSet::from_bits(Vec::new(), 0);
}

#[test]
#[should_panic]
fn from_bits_overflow() {
    BitSet::from_bits(vec![0b101], 2);
}

#[test]
fn test_one() {
    let bs = BitSet::from_bits(vec![0b10001_u32.to_le()], 5);
    assert_eq!(bs.test(4), true);
}

#[test]
fn test_zero() {
    let bs = BitSet::from_bits(vec![0b10001_u32.to_le()], 5);
    assert_eq!(bs.test(3), false);
}

#[test]
fn test_one_boundary() {
    let bs = BitSet::from_bits(vec![0b10000000000000000000000000000000_u32.to_le()], 32);
    assert_eq!(bs.test(31), true);
}

#[test]
fn test_zero_boundary() {
    let bs = BitSet::from_bits(vec![0], 32);
    assert_eq!(bs.test(31), false);
}

#[test]
fn test_one_boundary_next() {
    let bs = BitSet::from_bits(vec![0, 1_u32.to_le()], 33);
    assert_eq!(bs.test(32), true);
}

#[test]
fn test_zero_boundary_next() {
    let bs = BitSet::from_bits(vec![0, 0], 33);
    assert_eq!(bs.test(32), false);
}

#[test]
#[should_panic]
fn test_out_of_bounds() {
    let bs = BitSet::with_capacity(10);
    bs.test(10);
}

#[test]
fn set() {
    let mut bs = BitSet::with_capacity(8);
    bs.set(2);
    assert_eq!(bs.chunks[0], 0b100);
}

#[test]
fn set_boundary() {
    let mut bs = BitSet::with_capacity(32);
    bs.set(31);
}

#[test]
fn set_boundary_next() {
    let mut bs = BitSet::with_capacity(33);
    bs.set(32);
    assert_eq!(bs.test(32), true);
}

#[test]
#[should_panic]
fn set_out_of_bounds() {
    let mut bs = BitSet::with_capacity(33);
    bs.set(33);
}

#[test]
fn reset() {
    let mut bs = BitSet::from_bits(vec![u32::MAX], 32);
    bs.reset(15);
    assert_eq!(bs.test(15), false);
}

#[test]
fn reset_boundary() {
    let mut bs = BitSet::from_bits(vec![u32::MAX], 32);
    bs.reset(31);
    assert_eq!(bs.test(31), false);
}

#[test]
fn reset_boundary_next() {
    let mut bs = BitSet::from_bits(vec![u32::MAX, 1], 33);
    bs.reset(32);
    assert_eq!(bs.test(32), false);
}

#[test]
#[should_panic]
fn reset_out_of_bounds() {
    let mut bs = BitSet::with_capacity(32);
    bs.reset(32);
}

#[test]
fn big_bitset() {
    let mut bs = BitSet::with_capacity(3176);
    bs.set(2233);
    assert_eq!(bs.test(2232), false);
    assert_eq!(bs.test(2233), true);
    assert_eq!(bs.test(2234), false);
    bs.reset(2233);
    assert_eq!(bs.test(2233), false);
}

#[test]
fn set_all_in_range() {
    let mut bs = BitSet::with_capacity(32);
    bs.set_all();
    assert_eq!(bs.chunks[0], u32::MAX);
}

#[test]
fn set_all_verify_invariant() {
    let mut bs = BitSet::with_capacity(16);
    bs.set_all();
    assert_eq!(bs.chunks[0], u16::MAX as u32);
}

#[test]
fn reset_all() {
    let mut bs = BitSet::from_bits(vec![0b1111111100001111], 16);
    bs.reset_all();
    assert_eq!(bs.chunks[0], 0);
}

#[test]
fn repr() {
    let mut bs = BitSet::with_capacity(36);
    bs.set(4);
    bs.set(34);
    bs.set(17);
    assert_eq!(
        bs.chunks,
        vec![
            0b000000000000000100000000000010000_u32.to_le(),
            0b0100_u32.to_le()
        ]
    );
}
