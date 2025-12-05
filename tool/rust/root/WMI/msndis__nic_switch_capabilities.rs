// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_NicSwitchCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_NicSwitchCapabilities {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "NdisReserved1")]
    pub ndis_reserved1: Option<u32>,

/// 
    #[serde(rename = "NdisReserved2")]
    pub ndis_reserved2: Option<u32>,

/// 
    #[serde(rename = "NdisReserved3")]
    pub ndis_reserved3: Option<u32>,

/// 
    #[serde(rename = "NumMacAddressesPerPort")]
    pub num_mac_addresses_per_port: Option<u32>,

/// 
    #[serde(rename = "NumTotalMacAddresses")]
    pub num_total_mac_addresses: Option<u32>,

/// 
    #[serde(rename = "NumVlansPerPort")]
    pub num_vlans_per_port: Option<u32>,
}

impl MSNdis_NicSwitchCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            flags: None,
            header: None,
            ndis_reserved1: None,
            ndis_reserved2: None,
            ndis_reserved3: None,
            num_mac_addresses_per_port: None,
            num_total_mac_addresses: None,
            num_vlans_per_port: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of NdisReserved1
    pub fn set_ndis_reserved1(&mut self, value: u32) {
        self.ndis_reserved1 = Some(value);
    }

    /// Gets the value of NdisReserved1
    pub fn get_ndis_reserved1(&self) -> Option<&u32> {
        self.ndis_reserved1.as_ref()
    }

    /// Sets the value of NdisReserved2
    pub fn set_ndis_reserved2(&mut self, value: u32) {
        self.ndis_reserved2 = Some(value);
    }

    /// Gets the value of NdisReserved2
    pub fn get_ndis_reserved2(&self) -> Option<&u32> {
        self.ndis_reserved2.as_ref()
    }

    /// Sets the value of NdisReserved3
    pub fn set_ndis_reserved3(&mut self, value: u32) {
        self.ndis_reserved3 = Some(value);
    }

    /// Gets the value of NdisReserved3
    pub fn get_ndis_reserved3(&self) -> Option<&u32> {
        self.ndis_reserved3.as_ref()
    }

    /// Sets the value of NumMacAddressesPerPort
    pub fn set_num_mac_addresses_per_port(&mut self, value: u32) {
        self.num_mac_addresses_per_port = Some(value);
    }

    /// Gets the value of NumMacAddressesPerPort
    pub fn get_num_mac_addresses_per_port(&self) -> Option<&u32> {
        self.num_mac_addresses_per_port.as_ref()
    }

    /// Sets the value of NumTotalMacAddresses
    pub fn set_num_total_mac_addresses(&mut self, value: u32) {
        self.num_total_mac_addresses = Some(value);
    }

    /// Gets the value of NumTotalMacAddresses
    pub fn get_num_total_mac_addresses(&self) -> Option<&u32> {
        self.num_total_mac_addresses.as_ref()
    }

    /// Sets the value of NumVlansPerPort
    pub fn set_num_vlans_per_port(&mut self, value: u32) {
        self.num_vlans_per_port = Some(value);
    }

    /// Gets the value of NumVlansPerPort
    pub fn get_num_vlans_per_port(&self) -> Option<&u32> {
        self.num_vlans_per_port.as_ref()
    }
}

