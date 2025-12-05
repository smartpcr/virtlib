// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcLogFileSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcLogFileSettings {

/// 
    #[serde(rename = "MaxSizeInMB")]
    pub max_size_in_mb: Option<u32>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "SizeInMB")]
    pub size_in_mb: Option<u32>,
}

impl DtcLogFileSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            max_size_in_mb: None,
            path: None,
            size_in_mb: None,
        }
    }


    /// Sets the value of MaxSizeInMB
    pub fn set_max_size_in_mb(&mut self, value: u32) {
        self.max_size_in_mb = Some(value);
    }

    /// Gets the value of MaxSizeInMB
    pub fn get_max_size_in_mb(&self) -> Option<&u32> {
        self.max_size_in_mb.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of SizeInMB
    pub fn set_size_in_mb(&mut self, value: u32) {
        self.size_in_mb = Some(value);
    }

    /// Gets the value of SizeInMB
    pub fn get_size_in_mb(&self) -> Option<&u32> {
        self.size_in_mb.as_ref()
    }
}

