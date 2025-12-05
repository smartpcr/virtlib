// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ScriptingStandardConsumerSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptingStandardConsumerSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "MaximumScripts")]
    pub maximum_scripts: Option<u32>,

/// 
    #[serde(rename = "Timeout")]
    pub timeout: Option<u32>,
}

impl ScriptingStandardConsumerSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            maximum_scripts: None,
            timeout: None,
        }
    }


    /// Sets the value of MaximumScripts
    pub fn set_maximum_scripts(&mut self, value: u32) {
        self.maximum_scripts = Some(value);
    }

    /// Gets the value of MaximumScripts
    pub fn get_maximum_scripts(&self) -> Option<&u32> {
        self.maximum_scripts.as_ref()
    }

    /// Sets the value of Timeout
    pub fn set_timeout(&mut self, value: u32) {
        self.timeout = Some(value);
    }

    /// Gets the value of Timeout
    pub fn get_timeout(&self) -> Option<&u32> {
        self.timeout.as_ref()
    }
}

