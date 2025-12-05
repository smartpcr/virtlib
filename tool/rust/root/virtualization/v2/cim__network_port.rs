// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NetworkPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NetworkPort {
    #[serde(flatten)]
    pub base: CIM_LogicalPort,

/// The active or negotiated maximum transmission unit (MTU) that can be supported.
    #[serde(rename = "ActiveMaximumTransmissionUnit")]
    pub active_maximum_transmission_unit: Option<u64>,

/// A Boolean that indicates whether the NetworkPort is capable of automatically determining the speed or other communications characteristics of the attached network media.
    #[serde(rename = "AutoSense")]
    pub auto_sense: Option<bool>,

/// Boolean that indicates that the port is operating in full duplex mode.
    #[serde(rename = "FullDuplex")]
    pub full_duplex: Option<bool>,

/// An enumeration of the types of links. When set to 1 ("Other"), the related property OtherLinkTechnology contains a string description of the type of link.
    #[serde(rename = "LinkTechnology")]
    pub link_technology: Option<NetworkPort_LinkTechnology>,

/// An array of strings that indicates the network addresses for the port.
    #[serde(rename = "NetworkAddresses")]
    pub network_addresses: Vec<String>,

/// A string value that describes LinkTechnology when it is set to 1, "Other".
    #[serde(rename = "OtherLinkTechnology")]
    pub other_link_technology: Option<String>,

/// Note: The use of this property is deprecated in lieu of CIM_LogicalPort.PortType. 
/// Deprecated description: The type of module, when PortType is set to 1 ("Other".)
    #[serde(rename = "OtherNetworkPortType")]
    pub other_network_port_type: Option<String>,

/// PermanentAddress defines the network address that is hardcoded into a port. This 'hardcoded' address can be changed using a firmware upgrade or a software configuration. When this change is made, the field should be updated at the same time. PermanentAddress should be left blank if no 'hardcoded' address exists for the NetworkAdapter.
    #[serde(rename = "PermanentAddress")]
    pub permanent_address: Option<String>,

/// NetworkPorts are often numbered relative to either a logical module or a network element.
    #[serde(rename = "PortNumber")]
    pub port_number: Option<u16>,

/// The maximum transmission unit (MTU) that can be supported.
    #[serde(rename = "SupportedMaximumTransmissionUnit")]
    pub supported_maximum_transmission_unit: Option<u64>,
}

impl CIM_NetworkPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalPort::new(),
            active_maximum_transmission_unit: None,
            auto_sense: None,
            full_duplex: None,
            link_technology: None,
            network_addresses: Vec::new(),
            other_link_technology: None,
            other_network_port_type: None,
            permanent_address: None,
            port_number: None,
            supported_maximum_transmission_unit: None,
        }
    }


    /// Sets the value of ActiveMaximumTransmissionUnit
    pub fn set_active_maximum_transmission_unit(&mut self, value: u64) {
        self.active_maximum_transmission_unit = Some(value);
    }

    /// Gets the value of ActiveMaximumTransmissionUnit
    pub fn get_active_maximum_transmission_unit(&self) -> Option<&u64> {
        self.active_maximum_transmission_unit.as_ref()
    }

    /// Sets the value of AutoSense
    pub fn set_auto_sense(&mut self, value: bool) {
        self.auto_sense = Some(value);
    }

    /// Gets the value of AutoSense
    pub fn get_auto_sense(&self) -> Option<&bool> {
        self.auto_sense.as_ref()
    }

    /// Sets the value of FullDuplex
    pub fn set_full_duplex(&mut self, value: bool) {
        self.full_duplex = Some(value);
    }

    /// Gets the value of FullDuplex
    pub fn get_full_duplex(&self) -> Option<&bool> {
        self.full_duplex.as_ref()
    }

    /// Sets the value of LinkTechnology
    pub fn set_link_technology(&mut self, value: NetworkPort_LinkTechnology) {
        self.link_technology = Some(value);
    }

    /// Gets the value of LinkTechnology
    pub fn get_link_technology(&self) -> Option<&NetworkPort_LinkTechnology> {
        self.link_technology.as_ref()
    }

    /// Sets the value of NetworkAddresses
    pub fn set_network_addresses(&mut self, value: Vec<String>) {
        self.network_addresses = value;
    }

    /// Gets the value of NetworkAddresses
    pub fn get_network_addresses(&self) -> &Vec<String> {
        &self.network_addresses
    }

    /// Sets the value of OtherLinkTechnology
    pub fn set_other_link_technology(&mut self, value: String) {
        self.other_link_technology = Some(value);
    }

    /// Gets the value of OtherLinkTechnology
    pub fn get_other_link_technology(&self) -> Option<&String> {
        self.other_link_technology.as_ref()
    }

    /// Sets the value of OtherNetworkPortType
    pub fn set_other_network_port_type(&mut self, value: String) {
        self.other_network_port_type = Some(value);
    }

    /// Gets the value of OtherNetworkPortType
    pub fn get_other_network_port_type(&self) -> Option<&String> {
        self.other_network_port_type.as_ref()
    }

    /// Sets the value of PermanentAddress
    pub fn set_permanent_address(&mut self, value: String) {
        self.permanent_address = Some(value);
    }

    /// Gets the value of PermanentAddress
    pub fn get_permanent_address(&self) -> Option<&String> {
        self.permanent_address.as_ref()
    }

    /// Sets the value of PortNumber
    pub fn set_port_number(&mut self, value: u16) {
        self.port_number = Some(value);
    }

    /// Gets the value of PortNumber
    pub fn get_port_number(&self) -> Option<&u16> {
        self.port_number.as_ref()
    }

    /// Sets the value of SupportedMaximumTransmissionUnit
    pub fn set_supported_maximum_transmission_unit(&mut self, value: u64) {
        self.supported_maximum_transmission_unit = Some(value);
    }

    /// Gets the value of SupportedMaximumTransmissionUnit
    pub fn get_supported_maximum_transmission_unit(&self) -> Option<&u64> {
        self.supported_maximum_transmission_unit.as_ref()
    }
}

