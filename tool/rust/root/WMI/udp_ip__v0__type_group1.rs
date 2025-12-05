// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UdpIp_V0_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UdpIp_V0_TypeGroup1 {
    #[serde(flatten)]
    pub base: UdpIp_V0,

/// 
    #[serde(rename = "context")]
    pub context: Option<u32>,

/// 
    #[serde(rename = "daddr")]
    pub daddr: Option<serde_json::Value>,

/// 
    #[serde(rename = "dport")]
    pub dport: Option<serde_json::Value>,

/// 
    #[serde(rename = "dsize")]
    pub dsize: Option<u16>,

/// 
    #[serde(rename = "saddr")]
    pub saddr: Option<serde_json::Value>,

/// 
    #[serde(rename = "size")]
    pub size: Option<u16>,

/// 
    #[serde(rename = "sport")]
    pub sport: Option<serde_json::Value>,
}

impl UdpIp_V0_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: UdpIp_V0::new(),
            context: None,
            daddr: None,
            dport: None,
            dsize: None,
            saddr: None,
            size: None,
            sport: None,
        }
    }


    /// Sets the value of context
    pub fn set_context(&mut self, value: u32) {
        self.context = Some(value);
    }

    /// Gets the value of context
    pub fn get_context(&self) -> Option<&u32> {
        self.context.as_ref()
    }

    /// Sets the value of daddr
    pub fn set_daddr(&mut self, value: serde_json::Value) {
        self.daddr = Some(value);
    }

    /// Gets the value of daddr
    pub fn get_daddr(&self) -> Option<&serde_json::Value> {
        self.daddr.as_ref()
    }

    /// Sets the value of dport
    pub fn set_dport(&mut self, value: serde_json::Value) {
        self.dport = Some(value);
    }

    /// Gets the value of dport
    pub fn get_dport(&self) -> Option<&serde_json::Value> {
        self.dport.as_ref()
    }

    /// Sets the value of dsize
    pub fn set_dsize(&mut self, value: u16) {
        self.dsize = Some(value);
    }

    /// Gets the value of dsize
    pub fn get_dsize(&self) -> Option<&u16> {
        self.dsize.as_ref()
    }

    /// Sets the value of saddr
    pub fn set_saddr(&mut self, value: serde_json::Value) {
        self.saddr = Some(value);
    }

    /// Gets the value of saddr
    pub fn get_saddr(&self) -> Option<&serde_json::Value> {
        self.saddr.as_ref()
    }

    /// Sets the value of size
    pub fn set_size(&mut self, value: u16) {
        self.size = Some(value);
    }

    /// Gets the value of size
    pub fn get_size(&self) -> Option<&u16> {
        self.size.as_ref()
    }

    /// Sets the value of sport
    pub fn set_sport(&mut self, value: serde_json::Value) {
        self.sport = Some(value);
    }

    /// Gets the value of sport
    pub fn get_sport(&self) -> Option<&serde_json::Value> {
        self.sport.as_ref()
    }
}

