// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Wdac
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_OdbcKeyValuePair struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_OdbcKeyValuePair {

/// 
    #[serde(rename = "key")]
    pub key: Option<String>,

/// 
    #[serde(rename = "ParentKey")]
    pub parent_key: Option<String>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl MSFT_OdbcKeyValuePair {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            key: None,
            parent_key: None,
            value: None,
        }
    }


    /// Sets the value of key
    pub fn set_key(&mut self, value: String) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    /// Sets the value of ParentKey
    pub fn set_parent_key(&mut self, value: String) {
        self.parent_key = Some(value);
    }

    /// Gets the value of ParentKey
    pub fn get_parent_key(&self) -> Option<&String> {
        self.parent_key.as_ref()
    }

    /// Sets the value of Value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

