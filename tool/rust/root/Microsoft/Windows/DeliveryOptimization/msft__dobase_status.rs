// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DOBaseStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DOBaseStatus {

/// 
    #[serde(rename = "Id")]
    pub id: Option<u8>,
}

impl MSFT_DOBaseStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            id: None,
        }
    }


    /// Sets the value of Id
    pub fn set_id(&mut self, value: u8) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u8> {
        self.id.as_ref()
    }
}

