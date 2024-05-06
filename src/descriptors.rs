use std::collections::HashMap;

use cxx::SharedPtr;

use crate::ROMol;

pub struct Properties {
    ptr: SharedPtr<rdkit_sys::descriptors_ffi::Properties>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties::new()
    }
}

pub fn get_exact_molecular_weight(ro_mol: &ROMol) -> f64 {
    return rdkit_sys::descriptors_ffi::get_exact_molecular_weight(&ro_mol.ptr);
}

impl Properties {
    pub fn new() -> Self {
        Properties {
            ptr: rdkit_sys::descriptors_ffi::new_properties(),
        }
    }

    pub fn compute_properties(&self, ro_mol: &ROMol) -> HashMap<String, f64> {
        let names = rdkit_sys::descriptors_ffi::get_property_names(&self.ptr);
        let computed = rdkit_sys::descriptors_ffi::compute_properties(&self.ptr, &ro_mol.ptr);

        assert!(names.len() != 0);
        assert!(computed.len() == names.len());

        names
            .into_iter()
            .zip(computed.as_slice())
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    }
}
