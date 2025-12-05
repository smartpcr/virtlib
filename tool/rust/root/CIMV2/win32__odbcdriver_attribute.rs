// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ODBCDriverAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ODBCDriverAttribute {
    #[serde(flatten)]
    pub base: Win32_SettingCheck,
}

impl Win32_ODBCDriverAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_SettingCheck::new(),
        }
    }

}

