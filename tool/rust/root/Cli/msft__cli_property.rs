// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliProperty struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliProperty {

/// 
    #[serde(rename = "Derivation")]
    pub derivation: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Qualifiers")]
    pub qualifiers: Vec<MSFT_CliQualifier>,
}

impl MSFT_CliProperty {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            derivation: None,
            description: None,
            name: None,
            qualifiers: Vec::new(),
        }
    }


    /// Sets the value of Derivation
    pub fn set_derivation(&mut self, value: String) {
        self.derivation = Some(value);
    }

    /// Gets the value of Derivation
    pub fn get_derivation(&self) -> Option<&String> {
        self.derivation.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Qualifiers
    pub fn set_qualifiers(&mut self, value: Vec<MSFT_CliQualifier>) {
        self.qualifiers = value;
    }

    /// Gets the value of Qualifiers
    pub fn get_qualifiers(&self) -> &Vec<MSFT_CliQualifier> {
        &self.qualifiers
    }
}

