// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerPerformanceCounterSamples struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerPerformanceCounterSamples {

/// 
    #[serde(rename = "CounterPaths")]
    pub counter_paths: Vec<String>,

/// 
    #[serde(rename = "Timestamps")]
    pub timestamps: Vec<String>,

/// 
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

impl MSFT_ServerPerformanceCounterSamples {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            counter_paths: Vec::new(),
            timestamps: Vec::new(),
            values: Vec::new(),
        }
    }


    /// Sets the value of CounterPaths
    pub fn set_counter_paths(&mut self, value: Vec<String>) {
        self.counter_paths = value;
    }

    /// Gets the value of CounterPaths
    pub fn get_counter_paths(&self) -> &Vec<String> {
        &self.counter_paths
    }

    /// Sets the value of Timestamps
    pub fn set_timestamps(&mut self, value: Vec<String>) {
        self.timestamps = value;
    }

    /// Gets the value of Timestamps
    pub fn get_timestamps(&self) -> &Vec<String> {
        &self.timestamps
    }

    /// Sets the value of Values
    pub fn set_values(&mut self, value: Vec<String>) {
        self.values = value;
    }

    /// Gets the value of Values
    pub fn get_values(&self) -> &Vec<String> {
        &self.values
    }
}

