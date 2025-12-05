// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HealthRecord struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HealthRecord {

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Units")]
    pub units: Option<u16>,
}

impl MSFT_HealthRecord {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            units: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Units
    pub fn set_units(&mut self, value: u16) {
        self.units = Some(value);
    }

    /// Gets the value of Units
    pub fn get_units(&self) -> Option<&u16> {
        self.units.as_ref()
    }
}

