// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_OSVersionCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_OSVersionCheck {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "MaximumVersion")]
    pub maximum_version: Option<String>,

/// 
    #[serde(rename = "MinimumVersion")]
    pub minimum_version: Option<String>,
}

impl CIM_OSVersionCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            maximum_version: None,
            minimum_version: None,
        }
    }


    /// Sets the value of MaximumVersion
    pub fn set_maximum_version(&mut self, value: String) {
        self.maximum_version = Some(value);
    }

    /// Gets the value of MaximumVersion
    pub fn get_maximum_version(&self) -> Option<&String> {
        self.maximum_version.as_ref()
    }

    /// Sets the value of MinimumVersion
    pub fn set_minimum_version(&mut self, value: String) {
        self.minimum_version = Some(value);
    }

    /// Gets the value of MinimumVersion
    pub fn get_minimum_version(&self) -> Option<&String> {
        self.minimum_version.as_ref()
    }
}

