use std::fmt::{Debug, Formatter};

use cxx::let_cxx_string;
use log::error;
use rdkit_sys::*;

use crate::{Atom, Fingerprint, RWMol};

pub struct ROMol {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::ROMol>,
}

unsafe impl Send for ROMol {}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ROMolError {
    #[error("could not convert smiles to romol (nullptr)")]
    UnknownConversionError,
    #[error("could not convert smiles to romol (exception)")]
    ConversionException(String),
    #[error("could not write smiles from romol")]
    SmilesWriteError(String),
}

impl ROMol {
    pub fn from_smiles(smiles: &str) -> Result<Self, ROMolError> {
        let_cxx_string!(smiles_cxx_string = smiles);
        let ptr = ro_mol_ffi::smiles_to_mol(&smiles_cxx_string);
        match ptr {
            Ok(ptr) => {
                if ptr.is_null() {
                    Err(ROMolError::UnknownConversionError)
                } else {
                    Ok(ROMol { ptr })
                }
            }
            Err(e) => Err(ROMolError::ConversionException(e.to_string())),
        }
    }

    pub fn from_smiles_with_params(
        smiles: &str,
        params: &SmilesParserParams,
    ) -> Result<Self, String> {
        let_cxx_string!(smiles_cxx_string = smiles);

        // There is the risk that ptr is null, this is from rdkit c++
        let ptr = ro_mol_ffi::smiles_to_mol_with_params(&smiles_cxx_string, &params.ptr)
            .map_err(|except| except.what().to_owned())?;

        if ptr.is_null() {
            let informative_string = format!(
                "smiles_to_mol_with_params failed on inputs SMILES: `{}`, SmilesParserParams<sanitize={}, other properties unsupported>",
                smiles,
                params.get_sanitize()
            );
            Err(informative_string)
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn as_smiles(&self) -> Result<String, cxx::Exception> {
        ro_mol_ffi::mol_to_smiles(&self.ptr)
    }

    /// # Errors
    /// Will return `Err` if rooted_at_atom within `params` is greater than the
    /// number of atoms in the mol.
    pub fn as_smiles_with_params(&self, params: &SmilesWriteParams) -> Result<String, ROMolError> {
        let res = ro_mol_ffi::mol_to_smiles_with_params(&self.ptr, &params.ptr);
        match res {
            Ok(smi) => Ok(smi),
            Err(e) => Err(ROMolError::SmilesWriteError(e.to_string())),
        }
    }

    pub fn as_random_smiles_vec(&self, num_smiles: usize) -> Vec<String> {
        let mut params = SmilesWriteParams::default();
        params.do_random(true);
        (0..num_smiles)
            .map(|_| self.as_smiles_with_params(&params).unwrap())
            .collect()
    }

    pub fn to_molblock(&self) -> String {
        ro_mol_ffi::mol_to_molblock(&self.ptr)
    }

    pub fn as_rw_mol(&self, quick_copy: bool, conf_id: i32) -> RWMol {
        let ptr = rw_mol_ffi::rw_mol_from_ro_mol(&self.ptr, quick_copy, conf_id);
        RWMol { ptr }
    }

    pub fn fingerprint(&self) -> Fingerprint {
        let ptr = fingerprint_ffi::fingerprint_mol(&self.ptr);
        Fingerprint::new(ptr)
    }

    pub fn num_atoms(&self, only_explicit: bool) -> u32 {
        ro_mol_ffi::get_num_atoms(&self.ptr, only_explicit)
    }

    pub fn atom_with_idx(&mut self, idx: u32) -> Atom {
        let ptr = ro_mol_ffi::get_atom_with_idx(&mut self.ptr, idx);
        Atom::from_ptr(ptr)
    }

    pub fn update_property_cache(&mut self, strict: bool) {
        ro_mol_ffi::ro_mol_update_property_cache(&mut self.ptr, strict)
    }
}

impl Debug for ROMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let smiles = self.as_smiles();
        f.debug_tuple("ROMol").field(&smiles).finish()
    }
}

impl Clone for ROMol {
    fn clone(&self) -> Self {
        ROMol {
            ptr: ro_mol_ffi::copy_mol(&self.ptr),
        }
    }
}

pub struct SmilesParserParams {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::SmilesParserParams>,
}

impl SmilesParserParams {
    pub fn set_sanitize(&mut self, value: bool) {
        ro_mol_ffi::smiles_parser_params_set_sanitize(&self.ptr, value);
    }

    pub fn get_sanitize(&self) -> bool {
        ro_mol_ffi::smiles_parser_params_get_sanitize(&self.ptr)
    }
}

impl Default for SmilesParserParams {
    fn default() -> Self {
        SmilesParserParams {
            ptr: ro_mol_ffi::new_smiles_parser_params(),
        }
    }
}

pub struct SmilesWriteParams {
    pub(crate) ptr: cxx::SharedPtr<ro_mol_ffi::SmilesWriteParams>,
}

impl SmilesWriteParams {
    pub fn do_random(&mut self, value: bool) {
        ro_mol_ffi::smiles_write_params_set_do_random(&self.ptr, value);
    }

    pub fn rooted_at_atom(&mut self, value: i32) {
        ro_mol_ffi::smiles_write_params_set_rooted_at_atom(&self.ptr, value);
    }
}

impl Default for SmilesWriteParams {
    fn default() -> Self {
        SmilesWriteParams {
            ptr: ro_mol_ffi::new_smiles_write_params(),
        }
    }
}
