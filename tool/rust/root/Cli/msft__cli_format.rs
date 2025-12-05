// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliFormat struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliFormat {

/// 
    #[serde(rename = "Format")]
    pub format: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Properties")]
    pub properties: Vec<MSFT_CliProperty>,
}

impl MSFT_CliFormat {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            format: None,
            name: None,
            properties: Vec::new(),
        }
    }


    /// Sets the value of Format
    pub fn set_format(&mut self, value: String) {
        self.format = Some(value);
    }

    /// Gets the value of Format
    pub fn get_format(&self) -> Option<&String> {
        self.format.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Properties
    pub fn set_properties(&mut self, value: Vec<MSFT_CliProperty>) {
        self.properties = value;
    }

    /// Gets the value of Properties
    pub fn get_properties(&self) -> &Vec<MSFT_CliProperty> {
        &self.properties
    }
}

