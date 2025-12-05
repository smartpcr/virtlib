// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ALPC_Wait_For_Reply struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ALPC_Wait_For_Reply {
    #[serde(flatten)]
    pub base: ALPC,

/// 
    #[serde(rename = "MessageID")]
    pub message_id: Option<u32>,
}

impl ALPC_Wait_For_Reply {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ALPC::new(),
            message_id: None,
        }
    }


    /// Sets the value of MessageID
    pub fn set_message_id(&mut self, value: u32) {
        self.message_id = Some(value);
    }

    /// Gets the value of MessageID
    pub fn get_message_id(&self) -> Option<&u32> {
        self.message_id.as_ref()
    }
}

