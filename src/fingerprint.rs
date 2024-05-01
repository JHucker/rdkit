use bitvec::prelude::*;
use cxx::SharedPtr;

use crate::ROMol;

#[derive(Clone, Debug)]
pub struct Fingerprint(pub BitVec<u8, bitvec::order::Lsb0>);

impl Fingerprint {
    pub fn new(ptr: SharedPtr<rdkit_sys::fingerprint_ffi::ExplicitBitVect>) -> Self {
        let unique_ptr_bytes = rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_u64_vec(&ptr);
        let rdkit_fingerprint_bytes: Vec<u64> = unique_ptr_bytes.into_iter().copied().collect();
        let mut bitvec_u64 = bitvec::vec::BitVec::<u64, Lsb0>::from_vec(rdkit_fingerprint_bytes);

        let mut idiomatic_bitvec_u8 = bitvec::vec::BitVec::<u8, Lsb0>::new();
        idiomatic_bitvec_u8.append(&mut bitvec_u64);

        Fingerprint(idiomatic_bitvec_u8)
    }

    pub fn tanimoto_distance(&self, other: &Fingerprint) -> f32 {
        let and = self.0.clone() & &other.0;
        let or = self.0.clone() | &other.0;

        let and_ones = and.count_ones();
        let or_ones = or.count_ones();

        and_ones as f32 / or_ones as f32
    }
}

pub fn morgan_binary_vect_from_mol(mol: &ROMol, radius: usize, nbits: usize) -> Vec<u8> {
    assert_eq!(0, nbits % 8, "nbits must be divisible by 8!");
    let morgan = rdkit_sys::fingerprint_ffi::get_morgan_fingerprint(&mol.ptr, radius, nbits);
    rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_binary_vect(&morgan)
        .into_iter()
        .copied()
        .collect()
}
