// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventVmNetworkAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventVmNetworkAdapter {
    #[serde(flatten)]
    pub base: MSFT_NetEventPacketCaptureTarget,

/// 
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

/// 
    #[serde(rename = "PortName")]
    pub port_name: Option<String>,

/// 
    #[serde(rename = "SwitchName")]
    pub switch_name: Option<String>,

/// 
    #[serde(rename = "VMId")]
    pub vmid: Option<String>,

/// 
    #[serde(rename = "VMName")]
    pub vmname: Option<String>,
}

impl MSFT_NetEventVmNetworkAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetEventPacketCaptureTarget::new(),
            mac_address: None,
            port_name: None,
            switch_name: None,
            vmid: None,
            vmname: None,
        }
    }


    /// Sets the value of MacAddress
    pub fn set_mac_address(&mut self, value: String) {
        self.mac_address = Some(value);
    }

    /// Gets the value of MacAddress
    pub fn get_mac_address(&self) -> Option<&String> {
        self.mac_address.as_ref()
    }

    /// Sets the value of PortName
    pub fn set_port_name(&mut self, value: String) {
        self.port_name = Some(value);
    }

    /// Gets the value of PortName
    pub fn get_port_name(&self) -> Option<&String> {
        self.port_name.as_ref()
    }

    /// Sets the value of SwitchName
    pub fn set_switch_name(&mut self, value: String) {
        self.switch_name = Some(value);
    }

    /// Gets the value of SwitchName
    pub fn get_switch_name(&self) -> Option<&String> {
        self.switch_name.as_ref()
    }

    /// Sets the value of VMId
    pub fn set_vmid(&mut self, value: String) {
        self.vmid = Some(value);
    }

    /// Gets the value of VMId
    pub fn get_vmid(&self) -> Option<&String> {
        self.vmid.as_ref()
    }

    /// Sets the value of VMName
    pub fn set_vmname(&mut self, value: String) {
        self.vmname = Some(value);
    }

    /// Gets the value of VMName
    pub fn get_vmname(&self) -> Option<&String> {
        self.vmname.as_ref()
    }
}

