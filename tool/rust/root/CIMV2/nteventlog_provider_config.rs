// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// NTEventlogProviderConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NTEventlogProviderConfig {

/// 
    #[serde(rename = "LastBootUpTime")]
    pub last_boot_up_time: Option<String>,
}

impl NTEventlogProviderConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            last_boot_up_time: None,
        }
    }


    /// Sets the value of LastBootUpTime
    pub fn set_last_boot_up_time(&mut self, value: String) {
        self.last_boot_up_time = Some(value);
    }

    /// Gets the value of LastBootUpTime
    pub fn get_last_boot_up_time(&self) -> Option<&String> {
        self.last_boot_up_time.as_ref()
    }
}

