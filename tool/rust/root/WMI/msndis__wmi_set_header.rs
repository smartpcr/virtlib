// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiSetHeader struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiSetHeader {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "NetLuid")]
    pub net_luid: Option<u64>,

/// 
    #[serde(rename = "Padding")]
    pub padding: Option<u32>,

/// 
    #[serde(rename = "PortNumber")]
    pub port_number: Option<u32>,

/// 
    #[serde(rename = "RequestId")]
    pub request_id: Option<u64>,

/// 
    #[serde(rename = "Timeout")]
    pub timeout: Option<u32>,
}

impl MSNdis_WmiSetHeader {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            header: None,
            net_luid: None,
            padding: None,
            port_number: None,
            request_id: None,
            timeout: None,
        }
    }


    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of NetLuid
    pub fn set_net_luid(&mut self, value: u64) {
        self.net_luid = Some(value);
    }

    /// Gets the value of NetLuid
    pub fn get_net_luid(&self) -> Option<&u64> {
        self.net_luid.as_ref()
    }

    /// Sets the value of Padding
    pub fn set_padding(&mut self, value: u32) {
        self.padding = Some(value);
    }

    /// Gets the value of Padding
    pub fn get_padding(&self) -> Option<&u32> {
        self.padding.as_ref()
    }

    /// Sets the value of PortNumber
    pub fn set_port_number(&mut self, value: u32) {
        self.port_number = Some(value);
    }

    /// Gets the value of PortNumber
    pub fn get_port_number(&self) -> Option<&u32> {
        self.port_number.as_ref()
    }

    /// Sets the value of RequestId
    pub fn set_request_id(&mut self, value: u64) {
        self.request_id = Some(value);
    }

    /// Gets the value of RequestId
    pub fn get_request_id(&self) -> Option<&u64> {
        self.request_id.as_ref()
    }

    /// Sets the value of Timeout
    pub fn set_timeout(&mut self, value: u32) {
        self.timeout = Some(value);
    }

    /// Gets the value of Timeout
    pub fn get_timeout(&self) -> Option<&u32> {
        self.timeout.as_ref()
    }
}

