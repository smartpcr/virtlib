// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PortStateData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PortStateData {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "MediaConnectState")]
    pub media_connect_state: Option<u32>,

/// 
    #[serde(rename = "RcvAuthorizationState")]
    pub rcv_authorization_state: Option<u32>,

/// 
    #[serde(rename = "RcvControlState")]
    pub rcv_control_state: Option<u32>,

/// 
    #[serde(rename = "RcvLinkSpeed")]
    pub rcv_link_speed: Option<u64>,

/// 
    #[serde(rename = "SendAuthorizationState")]
    pub send_authorization_state: Option<u32>,

/// 
    #[serde(rename = "SendControlState")]
    pub send_control_state: Option<u32>,

/// 
    #[serde(rename = "XmitLinkSpeed")]
    pub xmit_link_speed: Option<u64>,
}

impl MSNdis_PortStateData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            direction: None,
            flags: None,
            header: None,
            media_connect_state: None,
            rcv_authorization_state: None,
            rcv_control_state: None,
            rcv_link_speed: None,
            send_authorization_state: None,
            send_control_state: None,
            xmit_link_speed: None,
        }
    }


    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: u32) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&u32> {
        self.direction.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of MediaConnectState
    pub fn set_media_connect_state(&mut self, value: u32) {
        self.media_connect_state = Some(value);
    }

    /// Gets the value of MediaConnectState
    pub fn get_media_connect_state(&self) -> Option<&u32> {
        self.media_connect_state.as_ref()
    }

    /// Sets the value of RcvAuthorizationState
    pub fn set_rcv_authorization_state(&mut self, value: u32) {
        self.rcv_authorization_state = Some(value);
    }

    /// Gets the value of RcvAuthorizationState
    pub fn get_rcv_authorization_state(&self) -> Option<&u32> {
        self.rcv_authorization_state.as_ref()
    }

    /// Sets the value of RcvControlState
    pub fn set_rcv_control_state(&mut self, value: u32) {
        self.rcv_control_state = Some(value);
    }

    /// Gets the value of RcvControlState
    pub fn get_rcv_control_state(&self) -> Option<&u32> {
        self.rcv_control_state.as_ref()
    }

    /// Sets the value of RcvLinkSpeed
    pub fn set_rcv_link_speed(&mut self, value: u64) {
        self.rcv_link_speed = Some(value);
    }

    /// Gets the value of RcvLinkSpeed
    pub fn get_rcv_link_speed(&self) -> Option<&u64> {
        self.rcv_link_speed.as_ref()
    }

    /// Sets the value of SendAuthorizationState
    pub fn set_send_authorization_state(&mut self, value: u32) {
        self.send_authorization_state = Some(value);
    }

    /// Gets the value of SendAuthorizationState
    pub fn get_send_authorization_state(&self) -> Option<&u32> {
        self.send_authorization_state.as_ref()
    }

    /// Sets the value of SendControlState
    pub fn set_send_control_state(&mut self, value: u32) {
        self.send_control_state = Some(value);
    }

    /// Gets the value of SendControlState
    pub fn get_send_control_state(&self) -> Option<&u32> {
        self.send_control_state.as_ref()
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

