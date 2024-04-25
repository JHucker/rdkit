RDKit
---

A high-level library for performing common RDKit tasks such as SMILES parsing, molecule normalization, etc. Uses
the C++ API via bindings from [rdkit-sys](https://crates.io/crate/rdkit-sys).

This fork and branch contain the following deviations and / or new features compared to the original repo:

Comparison to Original Repo
---
 - RDKit changes
   - Exposed SMILES writing params, to allow generation of randomised SMILES ([PR](https://github.com/rdkit-rs/rdkit/pull/13), [issue](https://github.com/rdkit-rs/rdkit-sys/issues/11)).
   - Ensure that Mol -> SMILES is fallible ([PR](https://github.com/rdkit-rs/rdkit/pull/24), [issue](https://github.com/rdkit-rs/rdkit/issues/25))
 - Other
   - Added .pre-commit-config.yaml

Prerequisites
---

On a Mac:

    brew install rdkit

On Linux you will need a custom build of rdkit with static libraries. We are working to provide suitable debian packages.
This is holding up our CI.

Release Both rdkit and rdkit-sys
---

The `rdkit-sys` crate is a member of the `rdkit` workspace. All crates in the workspace should move in lockstep versions for simplicity's sake, making it easy to tell which crates are compatible.

Managing multiple crates in a single repository:

    cargo workspace version patch
