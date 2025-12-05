// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAURun_Report_ID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAURun_Report_ID {

/// 
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,
}

impl MSFT_CAURun_Report_ID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            timestamp: None,
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
}

