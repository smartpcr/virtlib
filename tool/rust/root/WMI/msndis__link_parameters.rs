// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_LinkParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_LinkParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "AutoNegotiationFlags")]
    pub auto_negotiation_flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "MediaDuplexState")]
    pub media_duplex_state: Option<u32>,

/// 
    #[serde(rename = "PauseFunctions")]
    pub pause_functions: Option<u32>,

/// 
    #[serde(rename = "RcvLinkSpeed")]
    pub rcv_link_speed: Option<u64>,

/// 
    #[serde(rename = "XmitLinkSpeed")]
    pub xmit_link_speed: Option<u64>,
}

impl MSNdis_LinkParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            auto_negotiation_flags: None,
            header: None,
            media_duplex_state: None,
            pause_functions: None,
            rcv_link_speed: None,
            xmit_link_speed: None,
        }
    }


    /// Sets the value of AutoNegotiationFlags
    pub fn set_auto_negotiation_flags(&mut self, value: u32) {
        self.auto_negotiation_flags = Some(value);
    }

    /// Gets the value of AutoNegotiationFlags
    pub fn get_auto_negotiation_flags(&self) -> Option<&u32> {
        self.auto_negotiation_flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of MediaDuplexState
    pub fn set_media_duplex_state(&mut self, value: u32) {
        self.media_duplex_state = Some(value);
    }

    /// Gets the value of MediaDuplexState
    pub fn get_media_duplex_state(&self) -> Option<&u32> {
        self.media_duplex_state.as_ref()
    }

    /// Sets the value of PauseFunctions
    pub fn set_pause_functions(&mut self, value: u32) {
        self.pause_functions = Some(value);
    }

    /// Gets the value of PauseFunctions
    pub fn get_pause_functions(&self) -> Option<&u32> {
        self.pause_functions.as_ref()
    }

    /// Sets the value of RcvLinkSpeed
    pub fn set_rcv_link_speed(&mut self, value: u64) {
        self.rcv_link_speed = Some(value);
    }

    /// Gets the value of RcvLinkSpeed
    pub fn get_rcv_link_speed(&self) -> Option<&u64> {
        self.rcv_link_speed.as_ref()
    }

    /// Sets the value of XmitLinkSpeed
    pub fn set_xmit_link_speed(&mut self, value: u64) {
        self.xmit_link_speed = Some(value);
    }

    /// Gets the value of XmitLinkSpeed
    pub fn get_xmit_link_speed(&self) -> Option<&u64> {
        self.xmit_link_speed.as_ref()
    }
}

