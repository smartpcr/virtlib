// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_VideoSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_VideoSettings {
    #[serde(flatten)]
    pub base: CIM_VideoSetting,
}

impl Win32_VideoSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VideoSetting::new(),
        }
    }

}

