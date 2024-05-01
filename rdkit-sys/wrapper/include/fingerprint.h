#pragma once

#include "rust/cxx.h"

namespace RDKit {
    using ExplicitBitVect = ::ExplicitBitVect;
    std::shared_ptr<ExplicitBitVect> get_rdkit_fingerprint(const std::shared_ptr<ROMol> &mol);
    std::shared_ptr<ExplicitBitVect> get_morgan_fingerprint(const std::shared_ptr<ROMol> &mol, long unsigned int radius, long unsigned int n_bits);
    std::unique_ptr<std::vector<unsigned char>>  explicit_bit_vect_to_binary_vect(const std::shared_ptr<ExplicitBitVect> &bitvect);

    std::shared_ptr<ExplicitBitVect> copy_explicit_bit_vect(const std::shared_ptr<ExplicitBitVect> &orig);
    unsigned int get_num_on_bits(const std::shared_ptr<ExplicitBitVect> &bitvect);

    std::unique_ptr<std::vector<uint64_t>> explicit_bit_vect_to_u64_vec(const std::shared_ptr<ExplicitBitVect> &bitvect);
}