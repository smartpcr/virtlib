// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_PortalInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_PortalInfo {

/// An integer used to uniquely identify a paticular port
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// The portal's network address
    #[serde(rename = "IPAddr")]
    pub ipaddr: Option<ISCSI_IP_Address>,

/// The portal's socket number
    #[serde(rename = "Port")]
    pub port: Option<u32>,

/// The portal's aggregation tag
    #[serde(rename = "PortalTag")]
    pub portal_tag: Option<u16>,

/// **typedef** The type of portal (Initiator or Target) 
/// 
    #[serde(rename = "PortalType")]
    pub portal_type: Option<PortalInfo_PortalType>,

/// The portal's transport protocol
    #[serde(rename = "Protocol")]
    pub protocol: Option<PortalInfo_Protocol>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u8>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u8>,
}

impl ISCSI_PortalInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            index: None,
            ipaddr: None,
            port: None,
            portal_tag: None,
            portal_type: None,
            protocol: None,
            reserved1: None,
            reserved2: None,
        }
    }


    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of IPAddr
    pub fn set_ipaddr(&mut self, value: ISCSI_IP_Address) {
        self.ipaddr = Some(value);
    }

    /// Gets the value of IPAddr
    pub fn get_ipaddr(&self) -> Option<&ISCSI_IP_Address> {
        self.ipaddr.as_ref()
    }

    /// Sets the value of Port
    pub fn set_port(&mut self, value: u32) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&u32> {
        self.port.as_ref()
    }

    /// Sets the value of PortalTag
    pub fn set_portal_tag(&mut self, value: u16) {
        self.portal_tag = Some(value);
    }

    /// Gets the value of PortalTag
    pub fn get_portal_tag(&self) -> Option<&u16> {
        self.portal_tag.as_ref()
    }

    /// Sets the value of PortalType
    pub fn set_portal_type(&mut self, value: PortalInfo_PortalType) {
        self.portal_type = Some(value);
    }

    /// Gets the value of PortalType
    pub fn get_portal_type(&self) -> Option<&PortalInfo_PortalType> {
        self.portal_type.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: PortalInfo_Protocol) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&PortalInfo_Protocol> {
        self.protocol.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u8) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u8> {
        self.reserved1.as_ref()
    }

    /// Sets the value of Reserved2
    pub fn set_reserved2(&mut self, value: u8) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of Reserved2
    pub fn get_reserved2(&self) -> Option<&u8> {
        self.reserved2.as_ref()
    }
}

