// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutputWriteArray struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutputWriteArray {
    #[serde(flatten)]
    pub base: MSFT_DSCConfigurationOutput,

/// 
    #[serde(rename = "Array")]
    pub array: Vec<serde_json::Value>,

/// 
    #[serde(rename = "ParameterName")]
    pub parameter_name: Option<String>,
}

impl MSFT_DSCConfigurationOutputWriteArray {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DSCConfigurationOutput::new(),
            array: Vec::new(),
            parameter_name: None,
        }
    }


    /// Sets the value of Array
    pub fn set_array(&mut self, value: Vec<serde_json::Value>) {
        self.array = value;
    }

    /// Gets the value of Array
    pub fn get_array(&self) -> &Vec<serde_json::Value> {
        &self.array
    }

    /// Sets the value of ParameterName
    pub fn set_parameter_name(&mut self, value: String) {
        self.parameter_name = Some(value);
    }

    /// Gets the value of ParameterName
    pub fn get_parameter_name(&self) -> Option<&String> {
        self.parameter_name.as_ref()
    }
}

