// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiIPSecOffloadV1_IPv4ESP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiIPSecOffloadV1_IPv4ESP {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Des")]
    pub des: Option<u32>,

/// 
    #[serde(rename = "NullEsp")]
    pub null_esp: Option<u32>,

/// 
    #[serde(rename = "Receive")]
    pub receive: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// 
    #[serde(rename = "Send")]
    pub send: Option<u32>,

/// 
    #[serde(rename = "Transport")]
    pub transport: Option<u32>,

/// 
    #[serde(rename = "TripleDes")]
    pub triple_des: Option<u32>,

/// 
    #[serde(rename = "Tunnel")]
    pub tunnel: Option<u32>,
}

impl MSNdis_WmiIPSecOffloadV1_IPv4ESP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            des: None,
            null_esp: None,
            receive: None,
            reserved: None,
            send: None,
            transport: None,
            triple_des: None,
            tunnel: None,
        }
    }


    /// Sets the value of Des
    pub fn set_des(&mut self, value: u32) {
        self.des = Some(value);
    }

    /// Gets the value of Des
    pub fn get_des(&self) -> Option<&u32> {
        self.des.as_ref()
    }

    /// Sets the value of NullEsp
    pub fn set_null_esp(&mut self, value: u32) {
        self.null_esp = Some(value);
    }

    /// Gets the value of NullEsp
    pub fn get_null_esp(&self) -> Option<&u32> {
        self.null_esp.as_ref()
    }

    /// Sets the value of Receive
    pub fn set_receive(&mut self, value: u32) {
        self.receive = Some(value);
    }

    /// Gets the value of Receive
    pub fn get_receive(&self) -> Option<&u32> {
        self.receive.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of Send
    pub fn set_send(&mut self, value: u32) {
        self.send = Some(value);
    }

    /// Gets the value of Send
    pub fn get_send(&self) -> Option<&u32> {
        self.send.as_ref()
    }

    /// Sets the value of Transport
    pub fn set_transport(&mut self, value: u32) {
        self.transport = Some(value);
    }

    /// Gets the value of Transport
    pub fn get_transport(&self) -> Option<&u32> {
        self.transport.as_ref()
    }

    /// Sets the value of TripleDes
    pub fn set_triple_des(&mut self, value: u32) {
        self.triple_des = Some(value);
    }

    /// Gets the value of TripleDes
    pub fn get_triple_des(&self) -> Option<&u32> {
        self.triple_des.as_ref()
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

