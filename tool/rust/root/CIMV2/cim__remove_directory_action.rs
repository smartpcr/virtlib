// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RemoveDirectoryAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RemoveDirectoryAction {
    #[serde(flatten)]
    pub base: CIM_DirectoryAction,

/// 
    #[serde(rename = "MustBeEmpty")]
    pub must_be_empty: Option<bool>,
}

impl CIM_RemoveDirectoryAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DirectoryAction::new(),
            must_be_empty: None,
        }
    }


    /// Sets the value of MustBeEmpty
    pub fn set_must_be_empty(&mut self, value: bool) {
        self.must_be_empty = Some(value);
    }

    /// Gets the value of MustBeEmpty
    pub fn get_must_be_empty(&self) -> Option<&bool> {
        self.must_be_empty.as_ref()
    }
}

