#pragma once

#include "rust/cxx.h"
#include <GraphMol/Descriptors/MolDescriptors.h>
#include <GraphMol/Descriptors/Property.h>

namespace RDKit {
    using Descriptors::Properties;

    double get_exact_molecular_weight(const std::shared_ptr<ROMol> &mol);
    std::shared_ptr<Properties> new_properties();
    std::unique_ptr<std::vector<std::string>> get_property_names(const std::shared_ptr<Properties> &props);
    std::unique_ptr<std::vector<double>> compute_properties(const std::shared_ptr<Properties> &props, const std::shared_ptr<ROMol> &mol);
}
