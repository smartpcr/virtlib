// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ExtraCapacityGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ExtraCapacityGroup {
    #[serde(flatten)]
    pub base: CIM_RedundancyGroup,

/// 
    #[serde(rename = "MinNumberNeeded")]
    pub min_number_needed: Option<u32>,
}

impl CIM_ExtraCapacityGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RedundancyGroup::new(),
            min_number_needed: None,
        }
    }


    /// Sets the value of MinNumberNeeded
    pub fn set_min_number_needed(&mut self, value: u32) {
        self.min_number_needed = Some(value);
    }

    /// Gets the value of MinNumberNeeded
    pub fn get_min_number_needed(&self) -> Option<&u32> {
        self.min_number_needed.as_ref()
    }
}

