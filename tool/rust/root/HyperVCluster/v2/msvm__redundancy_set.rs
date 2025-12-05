// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_RedundancySet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_RedundancySet {
    #[serde(flatten)]
    pub base: CIM_RedundancySet,
}

impl Msvm_RedundancySet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RedundancySet::new(),
        }
    }

}

impl Msvm_RedundancySet {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Vec<Msvm_ComputerSystem>, WmiError> {
        self.get_all_related("Msvm_ComputerSystem")
    }

}

