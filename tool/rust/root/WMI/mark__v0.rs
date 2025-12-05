// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Mark_V0 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Mark_V0 {
    #[serde(flatten)]
    pub base: PerfInfo_V0,

/// 
    #[serde(rename = "Message")]
    pub message: Option<String>,

/// 
    #[serde(rename = "Padding")]
    pub padding: Vec<char>,
}

impl Mark_V0 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V0::new(),
            message: None,
            padding: Vec::new(),
        }
    }


    /// Sets the value of Message
    pub fn set_message(&mut self, value: String) {
        self.message = Some(value);
    }

    /// Gets the value of Message
    pub fn get_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Sets the value of Padding
    pub fn set_padding(&mut self, value: Vec<char>) {
        self.padding = value;
    }

    /// Gets the value of Padding
    pub fn get_padding(&self) -> &Vec<char> {
        &self.padding
    }
}

