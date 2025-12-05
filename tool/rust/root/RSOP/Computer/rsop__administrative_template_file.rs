// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_AdministrativeTemplateFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_AdministrativeTemplateFile {

/// 
    #[serde(rename = "GPOID")]
    pub gpoid: Option<String>,

/// 
    #[serde(rename = "lastWriteTime")]
    pub last_write_time: Option<String>,

/// 
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl RSOP_AdministrativeTemplateFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            gpoid: None,
            last_write_time: None,
            name: None,
        }
    }


    /// Sets the value of GPOID
    pub fn set_gpoid(&mut self, value: String) {
        self.gpoid = Some(value);
    }

    /// Gets the value of GPOID
    pub fn get_gpoid(&self) -> Option<&String> {
        self.gpoid.as_ref()
    }

    /// Sets the value of lastWriteTime
    pub fn set_last_write_time(&mut self, value: String) {
        self.last_write_time = Some(value);
    }

    /// Gets the value of lastWriteTime
    pub fn get_last_write_time(&self) -> Option<&String> {
        self.last_write_time.as_ref()
    }

    /// Sets the value of name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

