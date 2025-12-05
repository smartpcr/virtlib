// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetTeredoState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetTeredoState {
    #[serde(flatten)]
    pub base: CIM_ElementSettingData,

/// 
    #[serde(rename = "Error")]
    pub error: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<String>,
}

impl MSFT_NetTeredoState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ElementSettingData::new(),
            error: None,
            state: None,
        }
    }


    /// Sets the value of Error
    pub fn set_error(&mut self, value: String) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: String) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&String> {
        self.state.as_ref()
    }
}

