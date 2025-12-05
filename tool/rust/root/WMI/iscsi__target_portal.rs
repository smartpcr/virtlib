// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_TargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_TargetPortal {

/// 
    #[serde(rename = "Address")]
    pub address: Option<ISCSI_IP_Address>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// 
    #[serde(rename = "Socket")]
    pub socket: Option<u16>,
}

impl ISCSI_TargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address: None,
            reserved: None,
            socket: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: ISCSI_IP_Address) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&ISCSI_IP_Address> {
        self.address.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of Socket
    pub fn set_socket(&mut self, value: u16) {
        self.socket = Some(value);
    }

    /// Gets the value of Socket
    pub fn get_socket(&self) -> Option<&u16> {
        self.socket.as_ref()
    }
}

