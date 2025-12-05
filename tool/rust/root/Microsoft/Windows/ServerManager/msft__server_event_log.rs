// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerEventLog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerEventLog {

/// 
    #[serde(rename = "LocalizedName")]
    pub localized_name: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_ServerEventLog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            localized_name: None,
            name: None,
        }
    }


    /// Sets the value of LocalizedName
    pub fn set_localized_name(&mut self, value: String) {
        self.localized_name = Some(value);
    }

    /// Gets the value of LocalizedName
    pub fn get_localized_name(&self) -> Option<&String> {
        self.localized_name.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

