// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NCProvCancelQuery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NCProvCancelQuery {
    #[serde(flatten)]
    pub base: MSFT_NCProvEvent,

/// 
    #[serde(rename = "ID")]
    pub id: Option<u32>,
}

impl MSFT_NCProvCancelQuery {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NCProvEvent::new(),
            id: None,
        }
    }


    /// Sets the value of ID
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
    }
}

