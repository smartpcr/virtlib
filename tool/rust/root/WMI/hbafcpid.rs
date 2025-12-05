// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HBAFCPID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HBAFCPID {

/// 
    #[serde(rename = "Fcid")]
    pub fcid: Option<u32>,

/// 
    #[serde(rename = "FcpLun")]
    pub fcp_lun: Option<u64>,

/// 
    #[serde(rename = "NodeWWN")]
    pub node_wwn: Vec<u8>,

/// 
    #[serde(rename = "PortWWN")]
    pub port_wwn: Vec<u8>,
}

impl HBAFCPID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fcid: None,
            fcp_lun: None,
            node_wwn: Vec::new(),
            port_wwn: Vec::new(),
        }
    }


    /// Sets the value of Fcid
    pub fn set_fcid(&mut self, value: u32) {
        self.fcid = Some(value);
    }

    /// Gets the value of Fcid
    pub fn get_fcid(&self) -> Option<&u32> {
        self.fcid.as_ref()
    }

    /// Sets the value of FcpLun
    pub fn set_fcp_lun(&mut self, value: u64) {
        self.fcp_lun = Some(value);
    }

    /// Gets the value of FcpLun
    pub fn get_fcp_lun(&self) -> Option<&u64> {
        self.fcp_lun.as_ref()
    }

    /// Sets the value of NodeWWN
    pub fn set_node_wwn(&mut self, value: Vec<u8>) {
        self.node_wwn = value;
    }

    /// Gets the value of NodeWWN
    pub fn get_node_wwn(&self) -> &Vec<u8> {
        &self.node_wwn
    }

    /// Sets the value of PortWWN
    pub fn set_port_wwn(&mut self, value: Vec<u8>) {
        self.port_wwn = value;
    }

    /// Gets the value of PortWWN
    pub fn get_port_wwn(&self) -> &Vec<u8> {
        &self.port_wwn
    }
}

