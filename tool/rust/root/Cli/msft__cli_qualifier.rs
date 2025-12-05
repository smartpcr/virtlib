// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliQualifier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliQualifier {

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "QualifierValue")]
    pub qualifier_value: Vec<String>,
}

impl MSFT_CliQualifier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            qualifier_value: Vec::new(),
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of QualifierValue
    pub fn set_qualifier_value(&mut self, value: Vec<String>) {
        self.qualifier_value = value;
    }

    /// Gets the value of QualifierValue
    pub fn get_qualifier_value(&self) -> &Vec<String> {
        &self.qualifier_value
    }
}

