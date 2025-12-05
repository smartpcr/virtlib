// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_FileSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_FileSpecification {
    #[serde(flatten)]
    pub base: CIM_FileSpecification,

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u16>,

/// 
    #[serde(rename = "FileID")]
    pub file_id: Option<String>,

/// 
    #[serde(rename = "Language")]
    pub language: Option<String>,

/// 
    #[serde(rename = "Sequence")]
    pub sequence: Option<u16>,
}

impl Win32_FileSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FileSpecification::new(),
            attributes: None,
            file_id: None,
            language: None,
            sequence: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u16) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u16> {
        self.attributes.as_ref()
    }

    /// Sets the value of FileID
    pub fn set_file_id(&mut self, value: String) {
        self.file_id = Some(value);
    }

    /// Gets the value of FileID
    pub fn get_file_id(&self) -> Option<&String> {
        self.file_id.as_ref()
    }

    /// Sets the value of Language
    pub fn set_language(&mut self, value: String) {
        self.language = Some(value);
    }

    /// Gets the value of Language
    pub fn get_language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Sets the value of Sequence
    pub fn set_sequence(&mut self, value: u16) {
        self.sequence = Some(value);
    }

    /// Gets the value of Sequence
    pub fn get_sequence(&self) -> Option<&u16> {
        self.sequence.as_ref()
    }
}

