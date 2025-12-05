// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliTranslateTableEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliTranslateTableEntry {

/// 
    #[serde(rename = "FromValue")]
    pub from_value: Option<String>,

/// 
    #[serde(rename = "ToValue")]
    pub to_value: Option<String>,
}

impl MSFT_CliTranslateTableEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            from_value: None,
            to_value: None,
        }
    }


    /// Sets the value of FromValue
    pub fn set_from_value(&mut self, value: String) {
        self.from_value = Some(value);
    }

    /// Gets the value of FromValue
    pub fn get_from_value(&self) -> Option<&String> {
        self.from_value.as_ref()
    }

    /// Sets the value of ToValue
    pub fn set_to_value(&mut self, value: String) {
        self.to_value = Some(value);
    }

    /// Gets the value of ToValue
    pub fn get_to_value(&self) -> Option<&String> {
        self.to_value.as_ref()
    }
}

