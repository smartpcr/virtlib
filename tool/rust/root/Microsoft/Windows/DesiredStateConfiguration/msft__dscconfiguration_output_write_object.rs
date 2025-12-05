// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationOutputWriteObject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationOutputWriteObject {
    #[serde(flatten)]
    pub base: MSFT_DSCConfigurationOutput,

/// 
    #[serde(rename = "Object")]
    pub object: Option<serde_json::Value>,
}

impl MSFT_DSCConfigurationOutputWriteObject {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DSCConfigurationOutput::new(),
            object: None,
        }
    }


    /// Sets the value of Object
    pub fn set_object(&mut self, value: serde_json::Value) {
        self.object = Some(value);
    }

    /// Gets the value of Object
    pub fn get_object(&self) -> Option<&serde_json::Value> {
        self.object.as_ref()
    }
}

