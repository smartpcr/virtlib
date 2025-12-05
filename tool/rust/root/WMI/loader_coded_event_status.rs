// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LoaderCodedEventStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoaderCodedEventStatus {
    #[serde(flatten)]
    pub base: Image,

/// 
    #[serde(rename = "BaseAddress")]
    pub base_address: Option<u64>,

/// 
    #[serde(rename = "Code")]
    pub code: Option<u8>,

/// 
    #[serde(rename = "ErrorOpcode")]
    pub error_opcode: Option<u8>,

/// 
    #[serde(rename = "String")]
    pub string: Option<String>,
}

impl LoaderCodedEventStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Image::new(),
            base_address: None,
            code: None,
            error_opcode: None,
            string: None,
        }
    }


    /// Sets the value of BaseAddress
    pub fn set_base_address(&mut self, value: u64) {
        self.base_address = Some(value);
    }

    /// Gets the value of BaseAddress
    pub fn get_base_address(&self) -> Option<&u64> {
        self.base_address.as_ref()
    }

    /// Sets the value of Code
    pub fn set_code(&mut self, value: u8) {
        self.code = Some(value);
    }

    /// Gets the value of Code
    pub fn get_code(&self) -> Option<&u8> {
        self.code.as_ref()
    }

    /// Sets the value of ErrorOpcode
    pub fn set_error_opcode(&mut self, value: u8) {
        self.error_opcode = Some(value);
    }

    /// Gets the value of ErrorOpcode
    pub fn get_error_opcode(&self) -> Option<&u8> {
        self.error_opcode.as_ref()
    }

    /// Sets the value of String
    pub fn set_string(&mut self, value: String) {
        self.string = Some(value);
    }

    /// Gets the value of String
    pub fn get_string(&self) -> Option<&String> {
        self.string.as_ref()
    }
}

