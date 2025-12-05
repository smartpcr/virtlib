// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilitiesEx struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilitiesEx {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities,

/// 
    #[serde(rename = "IsVxlanUDPPortConfigurable")]
    pub is_vxlan_udpport_configurable: Option<bool>,

/// 
    #[serde(rename = "VxlanUDPPortNumber")]
    pub vxlan_udpport_number: Option<u16>,
}

impl MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilitiesEx {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterEncapsulatedPacketTaskOffloadCapabilities::new(),
            is_vxlan_udpport_configurable: None,
            vxlan_udpport_number: None,
        }
    }


    /// Sets the value of IsVxlanUDPPortConfigurable
    pub fn set_is_vxlan_udpport_configurable(&mut self, value: bool) {
        self.is_vxlan_udpport_configurable = Some(value);
    }

    /// Gets the value of IsVxlanUDPPortConfigurable
    pub fn get_is_vxlan_udpport_configurable(&self) -> Option<&bool> {
        self.is_vxlan_udpport_configurable.as_ref()
    }

    /// Sets the value of VxlanUDPPortNumber
    pub fn set_vxlan_udpport_number(&mut self, value: u16) {
        self.vxlan_udpport_number = Some(value);
    }

    /// Gets the value of VxlanUDPPortNumber
    pub fn get_vxlan_udpport_number(&self) -> Option<&u16> {
        self.vxlan_udpport_number.as_ref()
    }
}

