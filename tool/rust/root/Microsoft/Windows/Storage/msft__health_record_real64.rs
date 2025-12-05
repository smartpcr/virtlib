// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HealthRecordReal64 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HealthRecordReal64 {
    #[serde(flatten)]
    pub base: MSFT_HealthRecord,

/// 
    #[serde(rename = "Value")]
    pub value: Option<f64>,
}

impl MSFT_HealthRecordReal64 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_HealthRecord::new(),
            value: None,
        }
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

