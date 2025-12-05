// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_ScriptCmd struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_ScriptCmd {

/// 
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,

/// 
    #[serde(rename = "executionTime")]
    pub execution_time: Option<String>,

/// 
    #[serde(rename = "script")]
    pub script: Option<String>,
}

impl RSOP_ScriptCmd {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            arguments: None,
            execution_time: None,
            script: None,
        }
    }


    /// Sets the value of arguments
    pub fn set_arguments(&mut self, value: String) {
        self.arguments = Some(value);
    }

    /// Gets the value of arguments
    pub fn get_arguments(&self) -> Option<&String> {
        self.arguments.as_ref()
    }

    /// Sets the value of executionTime
    pub fn set_execution_time(&mut self, value: String) {
        self.execution_time = Some(value);
    }

    /// Gets the value of executionTime
    pub fn get_execution_time(&self) -> Option<&String> {
        self.execution_time.as_ref()
    }

    /// Sets the value of script
    pub fn set_script(&mut self, value: String) {
        self.script = Some(value);
    }

    /// Gets the value of script
    pub fn get_script(&self) -> Option<&String> {
        self.script.as_ref()
    }
}

