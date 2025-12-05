// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Metric struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Metric {

/// 
    #[serde(rename = "CurrentTime")]
    pub current_time: Option<String>,

/// 
    #[serde(rename = "Datapoints")]
    pub datapoints: Vec<SDDC_Datapoint>,
}

impl SDDC_Metric {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            current_time: None,
            datapoints: Vec::new(),
        }
    }


    /// Sets the value of CurrentTime
    pub fn set_current_time(&mut self, value: String) {
        self.current_time = Some(value);
    }

    /// Gets the value of CurrentTime
    pub fn get_current_time(&self) -> Option<&String> {
        self.current_time.as_ref()
    }

    /// Sets the value of Datapoints
    pub fn set_datapoints(&mut self, value: Vec<SDDC_Datapoint>) {
        self.datapoints = value;
    }

    /// Gets the value of Datapoints
    pub fn get_datapoints(&self) -> &Vec<SDDC_Datapoint> {
        &self.datapoints
    }
}

