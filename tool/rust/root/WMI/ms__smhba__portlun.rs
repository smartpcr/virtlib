// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_PORTLUN struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_PORTLUN {

/// 
    #[serde(rename = "domainPortWWN")]
    pub domain_port_wwn: Vec<u8>,

/// 
    #[serde(rename = "PortWWN")]
    pub port_wwn: Vec<u8>,

/// 
    #[serde(rename = "TargetLun")]
    pub target_lun: Option<u64>,
}

impl MS_SMHBA_PORTLUN {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            domain_port_wwn: Vec::new(),
            port_wwn: Vec::new(),
            target_lun: None,
        }
    }


    /// Sets the value of domainPortWWN
    pub fn set_domain_port_wwn(&mut self, value: Vec<u8>) {
        self.domain_port_wwn = value;
    }

    /// Gets the value of domainPortWWN
    pub fn get_domain_port_wwn(&self) -> &Vec<u8> {
        &self.domain_port_wwn
    }

    /// Sets the value of PortWWN
    pub fn set_port_wwn(&mut self, value: Vec<u8>) {
        self.port_wwn = value;
    }

    /// Gets the value of PortWWN
    pub fn get_port_wwn(&self) -> &Vec<u8> {
        &self.port_wwn
    }

    /// Sets the value of TargetLun
    pub fn set_target_lun(&mut self, value: u64) {
        self.target_lun = Some(value);
    }

    /// Gets the value of TargetLun
    pub fn get_target_lun(&self) -> Option<&u64> {
        self.target_lun.as_ref()
    }
}

