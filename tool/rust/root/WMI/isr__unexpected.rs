// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISR_Unexpected struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISR_Unexpected {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "Vector")]
    pub vector: Option<u16>,
}

impl ISR_Unexpected {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            vector: None,
        }
    }


    /// Sets the value of Vector
    pub fn set_vector(&mut self, value: u16) {
        self.vector = Some(value);
    }

    /// Gets the value of Vector
    pub fn get_vector(&self) -> Option<&u16> {
        self.vector.as_ref()
    }
}

