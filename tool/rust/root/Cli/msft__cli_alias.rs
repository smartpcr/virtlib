// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Cli
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CliAlias struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CliAlias {

/// 
    #[serde(rename = "Connection")]
    pub connection: Option<MSFT_CliConnection>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Formats")]
    pub formats: Vec<MSFT_CliFormat>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "PWhere")]
    pub pwhere: Option<String>,

/// 
    #[serde(rename = "Qualifiers")]
    pub qualifiers: Vec<MSFT_CliQualifier>,

/// 
    #[serde(rename = "Target")]
    pub target: Option<String>,

/// 
    #[serde(rename = "Verbs")]
    pub verbs: Vec<MSFT_CliVerb>,
}

impl MSFT_CliAlias {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection: None,
            description: None,
            formats: Vec::new(),
            friendly_name: None,
            pwhere: None,
            qualifiers: Vec::new(),
            target: None,
            verbs: Vec::new(),
        }
    }


    /// Sets the value of Connection
    pub fn set_connection(&mut self, value: MSFT_CliConnection) {
        self.connection = Some(value);
    }

    /// Gets the value of Connection
    pub fn get_connection(&self) -> Option<&MSFT_CliConnection> {
        self.connection.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Formats
    pub fn set_formats(&mut self, value: Vec<MSFT_CliFormat>) {
        self.formats = value;
    }

    /// Gets the value of Formats
    pub fn get_formats(&self) -> &Vec<MSFT_CliFormat> {
        &self.formats
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of PWhere
    pub fn set_pwhere(&mut self, value: String) {
        self.pwhere = Some(value);
    }

    /// Gets the value of PWhere
    pub fn get_pwhere(&self) -> Option<&String> {
        self.pwhere.as_ref()
    }

    /// Sets the value of Qualifiers
    pub fn set_qualifiers(&mut self, value: Vec<MSFT_CliQualifier>) {
        self.qualifiers = value;
    }

    /// Gets the value of Qualifiers
    pub fn get_qualifiers(&self) -> &Vec<MSFT_CliQualifier> {
        &self.qualifiers
    }

    /// Sets the value of Target
    pub fn set_target(&mut self, value: String) {
        self.target = Some(value);
    }

    /// Gets the value of Target
    pub fn get_target(&self) -> Option<&String> {
        self.target.as_ref()
    }

    /// Sets the value of Verbs
    pub fn set_verbs(&mut self, value: Vec<MSFT_CliVerb>) {
        self.verbs = value;
    }

    /// Gets the value of Verbs
    pub fn get_verbs(&self) -> &Vec<MSFT_CliVerb> {
        &self.verbs
    }
}

