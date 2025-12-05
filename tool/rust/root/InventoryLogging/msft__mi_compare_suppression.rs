// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_MiCompareSuppression struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_MiCompareSuppression {

/// 
    #[serde(rename = "SuppressionSignal")]
    pub suppression_signal: Vec<serde_json::Value>,

/// 
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,
}

impl Msft_MiCompareSuppression {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            suppression_signal: Vec::new(),
            timestamp: None,
        }
    }


    /// Sets the value of SuppressionSignal
    pub fn set_suppression_signal(&mut self, value: Vec<serde_json::Value>) {
        self.suppression_signal = value;
    }

    /// Gets the value of SuppressionSignal
    pub fn get_suppression_signal(&self) -> &Vec<serde_json::Value> {
        &self.suppression_signal
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

