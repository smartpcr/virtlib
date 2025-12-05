// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_StorageRedundancyGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_StorageRedundancyGroup {
    #[serde(flatten)]
    pub base: CIM_RedundancyGroup,

/// 
    #[serde(rename = "TypeOfAlgorithm")]
    pub type_of_algorithm: Option<u16>,
}

impl CIM_StorageRedundancyGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RedundancyGroup::new(),
            type_of_algorithm: None,
        }
    }


    /// Sets the value of TypeOfAlgorithm
    pub fn set_type_of_algorithm(&mut self, value: u16) {
        self.type_of_algorithm = Some(value);
    }

    /// Gets the value of TypeOfAlgorithm
    pub fn get_type_of_algorithm(&self) -> Option<&u16> {
        self.type_of_algorithm.as_ref()
    }
}

