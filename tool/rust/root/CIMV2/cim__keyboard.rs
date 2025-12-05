// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Keyboard struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Keyboard {
    #[serde(flatten)]
    pub base: CIM_UserDevice,

/// 
    #[serde(rename = "Layout")]
    pub layout: Option<String>,

/// 
    #[serde(rename = "NumberOfFunctionKeys")]
    pub number_of_function_keys: Option<u16>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<u16>,
}

impl CIM_Keyboard {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_UserDevice::new(),
            layout: None,
            number_of_function_keys: None,
            password: None,
        }
    }


    /// Sets the value of Layout
    pub fn set_layout(&mut self, value: String) {
        self.layout = Some(value);
    }

    /// Gets the value of Layout
    pub fn get_layout(&self) -> Option<&String> {
        self.layout.as_ref()
    }

    /// Sets the value of NumberOfFunctionKeys
    pub fn set_number_of_function_keys(&mut self, value: u16) {
        self.number_of_function_keys = Some(value);
    }

    /// Gets the value of NumberOfFunctionKeys
    pub fn get_number_of_function_keys(&self) -> Option<&u16> {
        self.number_of_function_keys.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: u16) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&u16> {
        self.password.as_ref()
    }
}

