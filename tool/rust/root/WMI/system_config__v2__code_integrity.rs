// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_CodeIntegrity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_CodeIntegrity {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "CodeIntegrityInfo")]
    pub code_integrity_info: Option<u32>,
}

impl SystemConfig_V2_CodeIntegrity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            code_integrity_info: None,
        }
    }


    /// Sets the value of CodeIntegrityInfo
    pub fn set_code_integrity_info(&mut self, value: u32) {
        self.code_integrity_info = Some(value);
    }

    /// Gets the value of CodeIntegrityInfo
    pub fn get_code_integrity_info(&self) -> Option<&u32> {
        self.code_integrity_info.as_ref()
    }
}

