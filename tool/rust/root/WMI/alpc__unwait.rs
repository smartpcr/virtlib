// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ALPC_Unwait struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ALPC_Unwait {
    #[serde(flatten)]
    pub base: ALPC,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,
}

impl ALPC_Unwait {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ALPC::new(),
            status: None,
        }
    }


    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }
}

