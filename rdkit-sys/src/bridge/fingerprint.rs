#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {

    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/fingerprint.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type ExplicitBitVect;
        pub fn get_rdkit_fingerprint(mol: &SharedPtr<ROMol>) -> SharedPtr<ExplicitBitVect>;
        pub fn get_morgan_fingerprint(
            mol: &SharedPtr<ROMol>,
            radius: usize,
            n_bits: usize,
        ) -> SharedPtr<ExplicitBitVect>;

        pub fn explicit_bit_vect_to_binary_vect(
            fingerprint: &SharedPtr<ExplicitBitVect>,
        ) -> UniquePtr<CxxVector<u8>>;

        pub fn copy_explicit_bit_vect(
            fingerprint: &SharedPtr<ExplicitBitVect>,
        ) -> SharedPtr<ExplicitBitVect>;

        pub fn explicit_bit_vect_to_u64_vec(
            bitvect: &SharedPtr<ExplicitBitVect>,
        ) -> UniquePtr<CxxVector<u64>>;
    }
}
