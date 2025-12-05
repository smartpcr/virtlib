// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_StorageDefect struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_StorageDefect {

/// 
    #[serde(rename = "Error")]
    pub error: Option<CIM_StorageError>,

/// 
    #[serde(rename = "Extent")]
    pub extent: Option<CIM_StorageExtent>,
}

impl CIM_StorageDefect {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            error: None,
            extent: None,
        }
    }


    /// Sets the value of Error
    pub fn set_error(&mut self, value: CIM_StorageError) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&CIM_StorageError> {
        self.error.as_ref()
    }

    /// Sets the value of Extent
    pub fn set_extent(&mut self, value: CIM_StorageExtent) {
        self.extent = Some(value);
    }

    /// Gets the value of Extent
    pub fn get_extent(&self) -> Option<&CIM_StorageExtent> {
        self.extent.as_ref()
    }
}

