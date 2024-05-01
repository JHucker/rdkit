#include "rust/cxx.h"
#include <DataStructs/ExplicitBitVect.h>
#include <DataStructs/BitOps.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/Fingerprints/MorganFingerprints.h>


namespace RDKit {
    std::shared_ptr<ExplicitBitVect> get_rdkit_fingerprint(const std::shared_ptr<ROMol> &mol) {
        return std::shared_ptr<ExplicitBitVect>(RDKFingerprintMol(*mol));
    }

    std::shared_ptr<ExplicitBitVect> get_morgan_fingerprint(const std::shared_ptr<ROMol> &mol, long unsigned int radius, long unsigned int n_bits) {
        return std::shared_ptr<ExplicitBitVect>(RDKit::MorganFingerprints::getFingerprintAsBitVect(*mol, radius, n_bits));
    }

    std::unique_ptr<std::vector<unsigned char>> explicit_bit_vect_to_binary_vect(const std::shared_ptr<ExplicitBitVect> &bitvect) {
        std::string binary_text = BitVectToBinaryText(*bitvect);

        std::vector<unsigned char> *vect = new std::vector<unsigned char>;
        vect->insert(vect->begin(), binary_text.begin(), binary_text.end());

        return std::unique_ptr<std::vector<unsigned char>>(vect);
    }

    std::shared_ptr<ExplicitBitVect> copy_explicit_bit_vect(const std::shared_ptr<ExplicitBitVect> &orig) {
        std::shared_ptr<ExplicitBitVect> fingerprint(new ExplicitBitVect(*orig));
        return fingerprint;
    }

    std::unique_ptr<std::vector<uint64_t>> explicit_bit_vect_to_u64_vec(const std::shared_ptr<ExplicitBitVect> &bitvect) {
        std::vector<uint64_t> bytes;
        bytes.reserve(bitvect->dp_bits->num_blocks());
        boost::to_block_range(*bitvect->dp_bits, (std::back_inserter(bytes)));
        std::vector<uint64_t> *bytes_heap = new std::vector<uint64_t>(bytes);
        return std::unique_ptr<std::vector<uint64_t>>(bytes_heap);
    }
}