// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Environment struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Environment {
    #[serde(flatten)]
    pub base: CIM_SystemResource,

/// 
    #[serde(rename = "SystemVariable")]
    pub system_variable: Option<bool>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

/// 
    #[serde(rename = "VariableValue")]
    pub variable_value: Option<String>,
}

impl Win32_Environment {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SystemResource::new(),
            system_variable: None,
            user_name: None,
            variable_value: None,
        }
    }


    /// Sets the value of SystemVariable
    pub fn set_system_variable(&mut self, value: bool) {
        self.system_variable = Some(value);
    }

    /// Gets the value of SystemVariable
    pub fn get_system_variable(&self) -> Option<&bool> {
        self.system_variable.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of VariableValue
    pub fn set_variable_value(&mut self, value: String) {
        self.variable_value = Some(value);
    }

    /// Gets the value of VariableValue
    pub fn get_variable_value(&self) -> Option<&String> {
        self.variable_value.as_ref()
    }
}

