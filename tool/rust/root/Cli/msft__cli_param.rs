// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliParam struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliParam {

/// 
    #[serde(rename = "Default")]
    pub default: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Optional")]
    pub optional: Option<bool>,

/// 
    #[serde(rename = "ParaId")]
    pub para_id: Option<String>,

/// 
    #[serde(rename = "Qualifiers")]
    pub qualifiers: Vec<MSFT_CliQualifier>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl MSFT_CliParam {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            default: None,
            description: None,
            optional: None,
            para_id: None,
            qualifiers: Vec::new(),
            type: None,
        }
    }


    /// Sets the value of Default
    pub fn set_default(&mut self, value: String) {
        self.default = Some(value);
    }

    /// Gets the value of Default
    pub fn get_default(&self) -> Option<&String> {
        self.default.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Optional
    pub fn set_optional(&mut self, value: bool) {
        self.optional = Some(value);
    }

    /// Gets the value of Optional
    pub fn get_optional(&self) -> Option<&bool> {
        self.optional.as_ref()
    }

    /// Sets the value of ParaId
    pub fn set_para_id(&mut self, value: String) {
        self.para_id = Some(value);
    }

    /// Gets the value of ParaId
    pub fn get_para_id(&self) -> Option<&String> {
        self.para_id.as_ref()
    }

    /// Sets the value of Qualifiers
    pub fn set_qualifiers(&mut self, value: Vec<MSFT_CliQualifier>) {
        self.qualifiers = value;
    }

    /// Gets the value of Qualifiers
    pub fn get_qualifiers(&self) -> &Vec<MSFT_CliQualifier> {
        &self.qualifiers
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

