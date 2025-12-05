// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ComClassEmulator struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ComClassEmulator {

/// 
    #[serde(rename = "NewVersion")]
    pub new_version: Option<Win32_ClassicCOMClass>,

/// 
    #[serde(rename = "OldVersion")]
    pub old_version: Option<Win32_ClassicCOMClass>,
}

impl Win32_ComClassEmulator {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            new_version: None,
            old_version: None,
        }
    }


    /// Sets the value of NewVersion
    pub fn set_new_version(&mut self, value: Win32_ClassicCOMClass) {
        self.new_version = Some(value);
    }

    /// Gets the value of NewVersion
    pub fn get_new_version(&self) -> Option<&Win32_ClassicCOMClass> {
        self.new_version.as_ref()
    }

    /// Sets the value of OldVersion
    pub fn set_old_version(&mut self, value: Win32_ClassicCOMClass) {
        self.old_version = Some(value);
    }

    /// Gets the value of OldVersion
    pub fn get_old_version(&self) -> Option<&Win32_ClassicCOMClass> {
        self.old_version.as_ref()
    }
}

