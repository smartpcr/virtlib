// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CompatibilityVector struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CompatibilityVector {

/// 
    #[serde(rename = "CompareOperation")]
    pub compare_operation: Option<u32>,

/// 
    #[serde(rename = "CompatibilityInfo")]
    pub compatibility_info: Option<u64>,

/// 
    #[serde(rename = "VectorId")]
    pub vector_id: Option<u32>,
}

impl Msvm_CompatibilityVector {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            compare_operation: None,
            compatibility_info: None,
            vector_id: None,
        }
    }


    /// Sets the value of CompareOperation
    pub fn set_compare_operation(&mut self, value: u32) {
        self.compare_operation = Some(value);
    }

    /// Gets the value of CompareOperation
    pub fn get_compare_operation(&self) -> Option<&u32> {
        self.compare_operation.as_ref()
    }

    /// Sets the value of CompatibilityInfo
    pub fn set_compatibility_info(&mut self, value: u64) {
        self.compatibility_info = Some(value);
    }

    /// Gets the value of CompatibilityInfo
    pub fn get_compatibility_info(&self) -> Option<&u64> {
        self.compatibility_info.as_ref()
    }

    /// Sets the value of VectorId
    pub fn set_vector_id(&mut self, value: u32) {
        self.vector_id = Some(value);
    }

    /// Gets the value of VectorId
    pub fn get_vector_id(&self) -> Option<&u32> {
        self.vector_id.as_ref()
    }
}

