// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CopyFileAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CopyFileAction {
    #[serde(flatten)]
    pub base: CIM_FileAction,

/// 
    #[serde(rename = "DeleteAfterCopy")]
    pub delete_after_copy: Option<bool>,

/// 
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

/// 
    #[serde(rename = "Source")]
    pub source: Option<String>,
}

impl CIM_CopyFileAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FileAction::new(),
            delete_after_copy: None,
            destination: None,
            source: None,
        }
    }


    /// Sets the value of DeleteAfterCopy
    pub fn set_delete_after_copy(&mut self, value: bool) {
        self.delete_after_copy = Some(value);
    }

    /// Gets the value of DeleteAfterCopy
    pub fn get_delete_after_copy(&self) -> Option<&bool> {
        self.delete_after_copy.as_ref()
    }

    /// Sets the value of Destination
    pub fn set_destination(&mut self, value: String) {
        self.destination = Some(value);
    }

    /// Gets the value of Destination
    pub fn get_destination(&self) -> Option<&String> {
        self.destination.as_ref()
    }

    /// Sets the value of Source
    pub fn set_source(&mut self, value: String) {
        self.source = Some(value);
    }

    /// Gets the value of Source
    pub fn get_source(&self) -> Option<&String> {
        self.source.as_ref()
    }
}

