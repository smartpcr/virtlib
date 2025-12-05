// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Image_V1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Image_V1 {
    #[serde(flatten)]
    pub base: MSNT_SystemTrace,
}

impl Image_V1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNT_SystemTrace::new(),
        }
    }

}

