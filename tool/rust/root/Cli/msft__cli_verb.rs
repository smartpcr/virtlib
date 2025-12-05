// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliVerb struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliVerb {

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
    #[serde(rename = "Parameters")]
    pub parameters: Vec<MSFT_CliParam>,

/// 
    #[serde(rename = "Qualifiers")]
    pub qualifiers: Vec<MSFT_CliQualifier>,

/// 
    #[serde(rename = "Usage")]
    pub usage: Option<String>,

/// 
    #[serde(rename = "VerbType")]
    pub verb_type: Option<u32>,
}

impl MSFT_CliVerb {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            derivation: None,
            description: None,
            name: None,
            parameters: Vec::new(),
            qualifiers: Vec::new(),
            usage: None,
            verb_type: None,
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

    /// Sets the value of Parameters
    pub fn set_parameters(&mut self, value: Vec<MSFT_CliParam>) {
        self.parameters = value;
    }

    /// Gets the value of Parameters
    pub fn get_parameters(&self) -> &Vec<MSFT_CliParam> {
        &self.parameters
    }

    /// Sets the value of Qualifiers
    pub fn set_qualifiers(&mut self, value: Vec<MSFT_CliQualifier>) {
        self.qualifiers = value;
    }

    /// Gets the value of Qualifiers
    pub fn get_qualifiers(&self) -> &Vec<MSFT_CliQualifier> {
        &self.qualifiers
    }

    /// Sets the value of Usage
    pub fn set_usage(&mut self, value: String) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&String> {
        self.usage.as_ref()
    }

    /// Sets the value of VerbType
    pub fn set_verb_type(&mut self, value: u32) {
        self.verb_type = Some(value);
    }

    /// Gets the value of VerbType
    pub fn get_verb_type(&self) -> Option<&u32> {
        self.verb_type.as_ref()
    }
}

