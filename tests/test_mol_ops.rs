use rdkit::{add_hs, clean_up, remove_hs, set_hybridization, ROMol, RemoveHsParameters};

#[test]
fn test_remove_hs() {
    let ro_mol = ROMol::from_smiles("[2H]C").unwrap();

    let mut remove_hs_parameters = RemoveHsParameters::new();
    remove_hs_parameters.set_remove_and_track_isotopes(true);
    remove_hs_parameters.set_remove_defining_bond_stereo(true);

    let no_hs_mol = remove_hs(&ro_mol, &remove_hs_parameters, true);
    let no_hs_smiles = no_hs_mol.as_smiles().unwrap();
    assert_eq!(no_hs_smiles, "C");

    let add_hs_mol = add_hs(&no_hs_mol, false, false, false);
    let add_hs_smiles = add_hs_mol.as_smiles().unwrap();
    assert_eq!(add_hs_smiles, "[H]C([H])([H])[2H]");

    let remove_hs_parameters = RemoveHsParameters::new();
    let mut no_hs_mol2 = remove_hs(&add_hs_mol, &remove_hs_parameters, true);
    let no_hs_smiles2 = no_hs_mol2.as_smiles().unwrap();
    set_hybridization(&mut no_hs_mol2);
    assert_eq!(no_hs_smiles2, "[2H]C");
}

#[test]
fn test_mol_ops_clean_up() {
    let ro_mol = ROMol::from_smiles("[H]C([H])([H])([H])").unwrap();
    let mut rw_mol = ro_mol.as_rw_mol(true, 0);
    clean_up(&mut rw_mol);
    let new_ro_mol = rw_mol.to_ro_mol();
    let smiles = new_ro_mol.as_smiles().unwrap();
    assert_eq!(smiles, "C");
}
