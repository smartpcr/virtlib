// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerBpaResult struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerBpaResult {

/// 
    #[serde(rename = "BpaXPath")]
    pub bpa_xpath: Option<String>,

/// 
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

impl MSFT_ServerBpaResult {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bpa_xpath: None,
            values: Vec::new(),
        }
    }


    /// Sets the value of BpaXPath
    pub fn set_bpa_xpath(&mut self, value: String) {
        self.bpa_xpath = Some(value);
    }

    /// Gets the value of BpaXPath
    pub fn get_bpa_xpath(&self) -> Option<&String> {
        self.bpa_xpath.as_ref()
    }

    /// Sets the value of Values
    pub fn set_values(&mut self, value: Vec<String>) {
        self.values = value;
    }

    /// Gets the value of Values
    pub fn get_values(&self) -> &Vec<String> {
        &self.values
    }
}

