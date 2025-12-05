// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileTrace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileTrace {
    #[serde(flatten)]
    pub base: MSNT_FileBaseTrace_Set1,
}

impl FileTrace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNT_FileBaseTrace_Set1::new(),
        }
    }

}

