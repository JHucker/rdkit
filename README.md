RDKit
---

A high-level library for performing common RDKit tasks such as SMILES parsing, molecule normalization, etc. Uses
the C++ API via bindings from [rdkit-sys](https://crates.io/crate/rdkit-sys).

Notice: Requires rdkit 2023.09.1 or higher (like Ubuntu Noble 24.04)

This fork and branch contain the following deviations and / or new features compared to the original repo:

Comparison to Original Repo
---
- RDKit changes
    - Exposed SMILES writing params, to allow generation of randomised SMILES ([PR](https://github.com/rdkit-rs/rdkit/pull/13), [issue](https://github.com/rdkit-rs/rdkit-sys/issues/11)).
    - Ensure that Mol -> SMILES is fallible ([PR](https://github.com/rdkit-rs/rdkit/pull/24), [issue](https://github.com/rdkit-rs/rdkit/issues/25))
- Other
    - Added `.pre-commit-config.yaml`
    - Added `rust-toolchain.toml`, specifying nightly channel to appease clippy issues in `benches/molecule_benchmark.rs`

Conda rdkit compatability, for rdkit-rs 0.4.11, rdkit 2024.03.5 is incompatible (build will fail). 2024.03-1 -> 2024.03.4 seem to work.

Prerequisites
---

On a Mac:

    brew install rdkit

On Linux you will need a custom build of rdkit with static libraries. At the moment, we have some pre-compiled tar balls
available for AMD64 and ARM64 for the latest RKDit which get the job done for our CI (Note: these have been compiled on
Ubuntu 22.04, not 14.04...):

https://rdkit-rs-debian.s3.eu-central-1.amazonaws.com/rdkit_2024_03_3_ubuntu_14_04_amd64.tar.gz
https://rdkit-rs-debian.s3.eu-central-1.amazonaws.com/rdkit_2024_03_3_ubuntu_14_04_arm64.tar.gz

Release Both rdkit and rdkit-sys
---

The `rdkit-sys` crate is a member of the `rdkit` workspace. All crates in the workspace should move in lockstep versions
for simplicity's sake, making it easy to tell which crates are compatible.

Managing multiple crates in a single repository:

    rdkit % cargo workspaces version patch
    rdkit % cargo publish
    rdkit % cd rdkit-sys
    rdkit-sys % cargo publish


## Formatting
### C++ formatting
Clang formatting should be automatically applied when saving files in VSCode if you have the `clangd` extension installed. If you don't have it installed, you can install it from the marketplace. Else by right-clicking on the file and selecting `Format Document` should do the work.

### Rust formatting
Rust formatting should be automatically applied when saving files in VSCode if you have the `rust-analyzer` extension installed. Else by running `cargo fmt` in the root directory the entire project will be formatted.
