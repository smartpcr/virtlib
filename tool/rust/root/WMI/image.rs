// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Image struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub base: MSNT_SystemTrace,
}

impl Image {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNT_SystemTrace::new(),
        }
    }

}

