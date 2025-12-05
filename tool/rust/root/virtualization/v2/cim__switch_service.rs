// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SwitchService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SwitchService {
    #[serde(flatten)]
    pub base: CIM_ForwardingService,

/// Address used by this SwitchService when it must be uniquely identified. For an ethernet bridge, the MAC Address serves as the BridgeAddress. When concatenated with a SpanningTreeService Priority, a unique bridge identifier results. The MAC address is formatted as twelve hexadecimal digits (e.g., "010203040506"), with each pair representing one of the six octets of the MAC address in "canonical" bit order according to RFC 2469. In other scenarios, like Ipv6, the address is formatted as "ffff:ffff:ffff:ffff".
    #[serde(rename = "BridgeAddress")]
    pub bridge_address: Option<String>,

/// BridgeAddressType defines the type of addressing scheme used for this Bridge and its BridgeAddress property.
    #[serde(rename = "BridgeAddressType")]
    pub bridge_address_type: Option<SwitchService_BridgeAddressType>,

/// Indicates what type of switching service can be performed.
    #[serde(rename = "BridgeType")]
    pub bridge_type: Option<SwitchService_BridgeType>,

/// The number of switch ports controlled by this switching service.
    #[serde(rename = "NumPorts")]
    pub num_ports: Option<u16>,
}

impl CIM_SwitchService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ForwardingService::new(),
            bridge_address: None,
            bridge_address_type: None,
            bridge_type: None,
            num_ports: None,
        }
    }


    /// Sets the value of BridgeAddress
    pub fn set_bridge_address(&mut self, value: String) {
        self.bridge_address = Some(value);
    }

    /// Gets the value of BridgeAddress
    pub fn get_bridge_address(&self) -> Option<&String> {
        self.bridge_address.as_ref()
    }

    /// Sets the value of BridgeAddressType
    pub fn set_bridge_address_type(&mut self, value: SwitchService_BridgeAddressType) {
        self.bridge_address_type = Some(value);
    }

    /// Gets the value of BridgeAddressType
    pub fn get_bridge_address_type(&self) -> Option<&SwitchService_BridgeAddressType> {
        self.bridge_address_type.as_ref()
    }

    /// Sets the value of BridgeType
    pub fn set_bridge_type(&mut self, value: SwitchService_BridgeType) {
        self.bridge_type = Some(value);
    }

    /// Gets the value of BridgeType
    pub fn get_bridge_type(&self) -> Option<&SwitchService_BridgeType> {
        self.bridge_type.as_ref()
    }

    /// Sets the value of NumPorts
    pub fn set_num_ports(&mut self, value: u16) {
        self.num_ports = Some(value);
    }

    /// Gets the value of NumPorts
    pub fn get_num_ports(&self) -> Option<&u16> {
        self.num_ports.as_ref()
    }
}

