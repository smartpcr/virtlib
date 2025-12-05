// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Datapoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Datapoint {

/// 
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<f64>,
}

impl SDDC_Datapoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            timestamp: None,
            value: None,
        }
    }


    /// Sets the value of Timestamp
    pub fn set_timestamp(&mut self, value: String) {
        self.timestamp = Some(value);
    }

    /// Gets the value of Timestamp
    pub fn get_timestamp(&self) -> Option<&String> {
        self.timestamp.as_ref()
    }

    /// Sets the value of Value
    pub fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&f64> {
        self.value.as_ref()
    }
}

