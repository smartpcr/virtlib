// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPHttpsState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPHttpsState {
    #[serde(flatten)]
    pub base: CIM_ElementSettingData,

/// 
    #[serde(rename = "InterfaceStatus")]
    pub interface_status: Option<String>,

/// 
    #[serde(rename = "LastErrorCode")]
    pub last_error_code: Option<u32>,
}

impl MSFT_NetIPHttpsState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ElementSettingData::new(),
            interface_status: None,
            last_error_code: None,
        }
    }


    /// Sets the value of InterfaceStatus
    pub fn set_interface_status(&mut self, value: String) {
        self.interface_status = Some(value);
    }

    /// Gets the value of InterfaceStatus
    pub fn get_interface_status(&self) -> Option<&String> {
        self.interface_status.as_ref()
    }

    /// Sets the value of LastErrorCode
    pub fn set_last_error_code(&mut self, value: u32) {
        self.last_error_code = Some(value);
    }

    /// Gets the value of LastErrorCode
    pub fn get_last_error_code(&self) -> Option<&u32> {
        self.last_error_code.as_ref()
    }
}

