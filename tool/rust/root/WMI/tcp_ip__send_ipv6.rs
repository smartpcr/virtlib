// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TcpIp_SendIPV6 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TcpIp_SendIPV6 {
    #[serde(flatten)]
    pub base: TcpIp,

/// 
    #[serde(rename = "connid")]
    pub connid: Option<u32>,

/// 
    #[serde(rename = "daddr")]
    pub daddr: Option<serde_json::Value>,

/// 
    #[serde(rename = "dport")]
    pub dport: Option<serde_json::Value>,

/// 
    #[serde(rename = "endtime")]
    pub endtime: Option<u32>,

/// 
    #[serde(rename = "PID")]
    pub pid: Option<u32>,

/// 
    #[serde(rename = "saddr")]
    pub saddr: Option<serde_json::Value>,

/// 
    #[serde(rename = "seqnum")]
    pub seqnum: Option<u32>,

/// 
    #[serde(rename = "size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "sport")]
    pub sport: Option<serde_json::Value>,

/// 
    #[serde(rename = "startime")]
    pub startime: Option<u32>,
}

impl TcpIp_SendIPV6 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TcpIp::new(),
            connid: None,
            daddr: None,
            dport: None,
            endtime: None,
            pid: None,
            saddr: None,
            seqnum: None,
            size: None,
            sport: None,
            startime: None,
        }
    }


    /// Sets the value of connid
    pub fn set_connid(&mut self, value: u32) {
        self.connid = Some(value);
    }

    /// Gets the value of connid
    pub fn get_connid(&self) -> Option<&u32> {
        self.connid.as_ref()
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

    /// Sets the value of endtime
    pub fn set_endtime(&mut self, value: u32) {
        self.endtime = Some(value);
    }

    /// Gets the value of endtime
    pub fn get_endtime(&self) -> Option<&u32> {
        self.endtime.as_ref()
    }

    /// Sets the value of PID
    pub fn set_pid(&mut self, value: u32) {
        self.pid = Some(value);
    }

    /// Gets the value of PID
    pub fn get_pid(&self) -> Option<&u32> {
        self.pid.as_ref()
    }

    /// Sets the value of saddr
    pub fn set_saddr(&mut self, value: serde_json::Value) {
        self.saddr = Some(value);
    }

    /// Gets the value of saddr
    pub fn get_saddr(&self) -> Option<&serde_json::Value> {
        self.saddr.as_ref()
    }

    /// Sets the value of seqnum
    pub fn set_seqnum(&mut self, value: u32) {
        self.seqnum = Some(value);
    }

    /// Gets the value of seqnum
    pub fn get_seqnum(&self) -> Option<&u32> {
        self.seqnum.as_ref()
    }

    /// Sets the value of size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of size
    pub fn get_size(&self) -> Option<&u32> {
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

    /// Sets the value of startime
    pub fn set_startime(&mut self, value: u32) {
        self.startime = Some(value);
    }

    /// Gets the value of startime
    pub fn get_startime(&self) -> Option<&u32> {
        self.startime.as_ref()
    }
}

