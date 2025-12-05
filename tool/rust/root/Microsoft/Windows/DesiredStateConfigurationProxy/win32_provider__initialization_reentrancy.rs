// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Win32Provider_InitializationReentrancy
//////////////////////////////////////////////

/// Win32Provider_InitializationReentrancy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Win32Provider_InitializationReentrancy {
    /// CLSID
    #[serde(rename = "CLSID")]
    CLSID = 0,
    /// Namespace
    #[serde(rename = "Namespace")]
    Namespace = 1,
    /// COM_Object
    #[serde(rename = "COM_Object")]
    COMObject = 2,
}

impl Default for Win32Provider_InitializationReentrancy {
    fn default() -> Self {
        Self::CLSID
    }
}

