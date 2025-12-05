// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiIPSecOffloadV1_IPv4AH struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiIPSecOffloadV1_IPv4AH {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Md5")]
    pub md5: Option<u32>,

/// 
    #[serde(rename = "Receive")]
    pub receive: Option<u32>,

/// 
    #[serde(rename = "Send")]
    pub send: Option<u32>,

/// 
    #[serde(rename = "Sha_1")]
    pub sha_1: Option<u32>,

/// 
    #[serde(rename = "Transport")]
    pub transport: Option<u32>,

/// 
    #[serde(rename = "Tunnel")]
    pub tunnel: Option<u32>,
}

impl MSNdis_WmiIPSecOffloadV1_IPv4AH {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            md5: None,
            receive: None,
            send: None,
            sha_1: None,
            transport: None,
            tunnel: None,
        }
    }


    /// Sets the value of Md5
    pub fn set_md5(&mut self, value: u32) {
        self.md5 = Some(value);
    }

    /// Gets the value of Md5
    pub fn get_md5(&self) -> Option<&u32> {
        self.md5.as_ref()
    }

    /// Sets the value of Receive
    pub fn set_receive(&mut self, value: u32) {
        self.receive = Some(value);
    }

    /// Gets the value of Receive
    pub fn get_receive(&self) -> Option<&u32> {
        self.receive.as_ref()
    }

    /// Sets the value of Send
    pub fn set_send(&mut self, value: u32) {
        self.send = Some(value);
    }

    /// Gets the value of Send
    pub fn get_send(&self) -> Option<&u32> {
        self.send.as_ref()
    }

    /// Sets the value of Sha_1
    pub fn set_sha_1(&mut self, value: u32) {
        self.sha_1 = Some(value);
    }

    /// Gets the value of Sha_1
    pub fn get_sha_1(&self) -> Option<&u32> {
        self.sha_1.as_ref()
    }

    /// Sets the value of Transport
    pub fn set_transport(&mut self, value: u32) {
        self.transport = Some(value);
    }

    /// Gets the value of Transport
    pub fn get_transport(&self) -> Option<&u32> {
        self.transport.as_ref()
    }

    /// Sets the value of Tunnel
    pub fn set_tunnel(&mut self, value: u32) {
        self.tunnel = Some(value);
    }

    /// Gets the value of Tunnel
    pub fn get_tunnel(&self) -> Option<&u32> {
        self.tunnel.as_ref()
    }
}

