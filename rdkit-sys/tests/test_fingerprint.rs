#[test]
fn test_fingerprint_to_vec() {
    cxx::let_cxx_string!(smiles = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smiles).unwrap();

    let fingerprint = rdkit_sys::fingerprint_ffi::get_rdkit_fingerprint(&mol);
    let bytes = rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_u64_vec(&fingerprint);
    let bytes: Vec<u64> = bytes.into_iter().map(|x| *x).collect();
    println!("bytes: {:?}", bytes);
    assert_eq!(bytes.len(), 32);
}
