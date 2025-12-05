// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_OdbcDriver struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_OdbcDriver {

/// 
    #[serde(rename = "KeyValuePair")]
    pub key_value_pair: Vec<MSFT_OdbcKeyValuePair>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Platform")]
    pub platform: Option<String>,
}

impl MSFT_OdbcDriver {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            key_value_pair: Vec::new(),
            name: None,
            platform: None,
        }
    }


    /// Sets the value of KeyValuePair
    pub fn set_key_value_pair(&mut self, value: Vec<MSFT_OdbcKeyValuePair>) {
        self.key_value_pair = value;
    }

    /// Gets the value of KeyValuePair
    pub fn get_key_value_pair(&self) -> &Vec<MSFT_OdbcKeyValuePair> {
        &self.key_value_pair
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Platform
    pub fn set_platform(&mut self, value: String) {
        self.platform = Some(value);
    }

    /// Gets the value of Platform
    pub fn get_platform(&self) -> Option<&String> {
        self.platform.as_ref()
    }
}

