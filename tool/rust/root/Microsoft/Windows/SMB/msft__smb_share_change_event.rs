// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbShareChangeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbShareChangeEvent {

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<SmbShareChangeEvent_EventType>,

/// 
    #[serde(rename = "Share")]
    pub share: Option<MSFT_SmbShare>,
}

impl MSFT_SmbShareChangeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            event_type: None,
            share: None,
        }
    }


    /// Sets the value of EventType
    pub fn set_event_type(&mut self, value: SmbShareChangeEvent_EventType) {
        self.event_type = Some(value);
    }

    /// Gets the value of EventType
    pub fn get_event_type(&self) -> Option<&SmbShareChangeEvent_EventType> {
        self.event_type.as_ref()
    }

    /// Sets the value of Share
    pub fn set_share(&mut self, value: MSFT_SmbShare) {
        self.share = Some(value);
    }

    /// Gets the value of Share
    pub fn get_share(&self) -> Option<&MSFT_SmbShare> {
        self.share.as_ref()
    }
}

