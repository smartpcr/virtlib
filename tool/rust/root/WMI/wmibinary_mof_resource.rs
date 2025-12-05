// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WMIBinaryMofResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WMIBinaryMofResource {

/// 
    #[serde(rename = "HighDateTime")]
    pub high_date_time: Option<u32>,

/// 
    #[serde(rename = "LowDateTime")]
    pub low_date_time: Option<u32>,

/// 
    #[serde(rename = "MofProcessed")]
    pub mof_processed: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl WMIBinaryMofResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            high_date_time: None,
            low_date_time: None,
            mof_processed: None,
            name: None,
        }
    }


    /// Sets the value of HighDateTime
    pub fn set_high_date_time(&mut self, value: u32) {
        self.high_date_time = Some(value);
    }

    /// Gets the value of HighDateTime
    pub fn get_high_date_time(&self) -> Option<&u32> {
        self.high_date_time.as_ref()
    }

    /// Sets the value of LowDateTime
    pub fn set_low_date_time(&mut self, value: u32) {
        self.low_date_time = Some(value);
    }

    /// Gets the value of LowDateTime
    pub fn get_low_date_time(&self) -> Option<&u32> {
        self.low_date_time.as_ref()
    }

    /// Sets the value of MofProcessed
    pub fn set_mof_processed(&mut self, value: bool) {
        self.mof_processed = Some(value);
    }

    /// Gets the value of MofProcessed
    pub fn get_mof_processed(&self) -> Option<&bool> {
        self.mof_processed.as_ref()
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

