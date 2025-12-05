// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_SecuritySettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_SecuritySettings {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl RSOP_SecuritySettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            error_code: None,
            status: None,
        }
    }


    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: u32) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&u32> {
        self.error_code.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

