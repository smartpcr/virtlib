// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_EnvironmentSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_EnvironmentSpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "Environment")]
    pub environment: Option<String>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl Win32_EnvironmentSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            environment: None,
            value: None,
        }
    }


    /// Sets the value of Environment
    pub fn set_environment(&mut self, value: String) {
        self.environment = Some(value);
    }

    /// Gets the value of Environment
    pub fn get_environment(&self) -> Option<&String> {
        self.environment.as_ref()
    }

    /// Sets the value of Value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

