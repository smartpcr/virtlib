// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_InstalledOS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_InstalledOS {
    #[serde(flatten)]
    pub base: CIM_SystemComponent,

/// 
    #[serde(rename = "PrimaryOS")]
    pub primary_os: Option<bool>,
}

impl CIM_InstalledOS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SystemComponent::new(),
            primary_os: None,
        }
    }


    /// Sets the value of PrimaryOS
    pub fn set_primary_os(&mut self, value: bool) {
        self.primary_os = Some(value);
    }

    /// Gets the value of PrimaryOS
    pub fn get_primary_os(&self) -> Option<&bool> {
        self.primary_os.as_ref()
    }
}

