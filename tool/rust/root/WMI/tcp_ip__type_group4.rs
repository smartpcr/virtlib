// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TcpIp_TypeGroup4 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TcpIp_TypeGroup4 {
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
    #[serde(rename = "mss")]
    pub mss: Option<u16>,

/// 
    #[serde(rename = "PID")]
    pub pid: Option<u32>,

/// 
    #[serde(rename = "rcvwin")]
    pub rcvwin: Option<u32>,

/// 
    #[serde(rename = "rcvwinscale")]
    pub rcvwinscale: Option<i16>,

/// 
    #[serde(rename = "sackopt")]
    pub sackopt: Option<u16>,

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
    #[serde(rename = "sndwinscale")]
    pub sndwinscale: Option<i16>,

/// 
    #[serde(rename = "sport")]
    pub sport: Option<serde_json::Value>,

/// 
    #[serde(rename = "tsopt")]
    pub tsopt: Option<u16>,

/// 
    #[serde(rename = "wsopt")]
    pub wsopt: Option<u16>,
}

impl TcpIp_TypeGroup4 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TcpIp::new(),
            connid: None,
            daddr: None,
            dport: None,
            mss: None,
            pid: None,
            rcvwin: None,
            rcvwinscale: None,
            sackopt: None,
            saddr: None,
            seqnum: None,
            size: None,
            sndwinscale: None,
            sport: None,
            tsopt: None,
            wsopt: None,
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

    /// Sets the value of mss
    pub fn set_mss(&mut self, value: u16) {
        self.mss = Some(value);
    }

    /// Gets the value of mss
    pub fn get_mss(&self) -> Option<&u16> {
        self.mss.as_ref()
    }

    /// Sets the value of PID
    pub fn set_pid(&mut self, value: u32) {
        self.pid = Some(value);
    }

    /// Gets the value of PID
    pub fn get_pid(&self) -> Option<&u32> {
        self.pid.as_ref()
    }

    /// Sets the value of rcvwin
    pub fn set_rcvwin(&mut self, value: u32) {
        self.rcvwin = Some(value);
    }

    /// Gets the value of rcvwin
    pub fn get_rcvwin(&self) -> Option<&u32> {
        self.rcvwin.as_ref()
    }

    /// Sets the value of rcvwinscale
    pub fn set_rcvwinscale(&mut self, value: i16) {
        self.rcvwinscale = Some(value);
    }

    /// Gets the value of rcvwinscale
    pub fn get_rcvwinscale(&self) -> Option<&i16> {
        self.rcvwinscale.as_ref()
    }

    /// Sets the value of sackopt
    pub fn set_sackopt(&mut self, value: u16) {
        self.sackopt = Some(value);
    }

    /// Gets the value of sackopt
    pub fn get_sackopt(&self) -> Option<&u16> {
        self.sackopt.as_ref()
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

    /// Sets the value of sndwinscale
    pub fn set_sndwinscale(&mut self, value: i16) {
        self.sndwinscale = Some(value);
    }

    /// Gets the value of sndwinscale
    pub fn get_sndwinscale(&self) -> Option<&i16> {
        self.sndwinscale.as_ref()
    }

    /// Sets the value of sport
    pub fn set_sport(&mut self, value: serde_json::Value) {
        self.sport = Some(value);
    }

    /// Gets the value of sport
    pub fn get_sport(&self) -> Option<&serde_json::Value> {
        self.sport.as_ref()
    }

    /// Sets the value of tsopt
    pub fn set_tsopt(&mut self, value: u16) {
        self.tsopt = Some(value);
    }

    /// Gets the value of tsopt
    pub fn get_tsopt(&self) -> Option<&u16> {
        self.tsopt.as_ref()
    }

    /// Sets the value of wsopt
    pub fn set_wsopt(&mut self, value: u16) {
        self.wsopt = Some(value);
    }

    /// Gets the value of wsopt
    pub fn get_wsopt(&self) -> Option<&u16> {
        self.wsopt.as_ref()
    }
}

