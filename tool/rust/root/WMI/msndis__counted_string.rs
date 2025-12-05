// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_CountedString struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_CountedString {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Length")]
    pub length: Option<u16>,

/// 
    #[serde(rename = "String")]
    pub string: Vec<char>,
}

impl MSNdis_CountedString {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            length: None,
            string: Vec::new(),
        }
    }


    /// Sets the value of Length
    pub fn set_length(&mut self, value: u16) {
        self.length = Some(value);
    }

    /// Gets the value of Length
    pub fn get_length(&self) -> Option<&u16> {
        self.length.as_ref()
    }

    /// Sets the value of String
    pub fn set_string(&mut self, value: Vec<char>) {
        self.string = value;
    }

    /// Gets the value of String
    pub fn get_string(&self) -> &Vec<char> {
        &self.string
    }
}

