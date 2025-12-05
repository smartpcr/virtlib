// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ODBCTranslatorSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ODBCTranslatorSpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "File")]
    pub file: Option<String>,

/// 
    #[serde(rename = "SetupFile")]
    pub setup_file: Option<String>,

/// 
    #[serde(rename = "Translator")]
    pub translator: Option<String>,
}

impl Win32_ODBCTranslatorSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            file: None,
            setup_file: None,
            translator: None,
        }
    }


    /// Sets the value of File
    pub fn set_file(&mut self, value: String) {
        self.file = Some(value);
    }

    /// Gets the value of File
    pub fn get_file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    /// Sets the value of SetupFile
    pub fn set_setup_file(&mut self, value: String) {
        self.setup_file = Some(value);
    }

    /// Gets the value of SetupFile
    pub fn get_setup_file(&self) -> Option<&String> {
        self.setup_file.as_ref()
    }

    /// Sets the value of Translator
    pub fn set_translator(&mut self, value: String) {
        self.translator = Some(value);
    }

    /// Gets the value of Translator
    pub fn get_translator(&self) -> Option<&String> {
        self.translator.as_ref()
    }
}

