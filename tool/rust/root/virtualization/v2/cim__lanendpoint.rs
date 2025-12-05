// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LANEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LANEndpoint {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// Other unicast addresses that may be used to communicate with the LANEndpoint.
    #[serde(rename = "AliasAddresses")]
    pub alias_addresses: Vec<String>,

/// Multicast addresses to which the LANEndpoint listens.
    #[serde(rename = "GroupAddresses")]
    pub group_addresses: Vec<String>,

/// A label or identifier for the LAN Segment to which the Endpoint is connected. If the Endpoint is not currently active/connected or this information is not known, then LANID is NULL.
    #[serde(rename = "LANID")]
    pub lanid: Option<String>,

/// An indication of the kind of technology used on the LAN. This property is deprecated in lieu of ProtocolType, which is an enumeration inherited from ProtocolEndpoint and which includes the Values specified here.
    #[serde(rename = "LANType")]
    pub lantype: Option<LANEndpoint_LANType>,

/// The principal unicast address used in communication with the LANEndpoint. The MAC address is formatted as twelve hexadecimal digits (e.g., "010203040506"), with each pair representing one of the six octets of the MAC address in "canonical" bit order according to RFC 2469.
    #[serde(rename = "MACAddress")]
    pub macaddress: Option<String>,

/// The largest information field that may be sent or received by the LANEndpoint.
    #[serde(rename = "MaxDataSize")]
    pub max_data_size: Option<u32>,

/// A free-form string that describes the type of technology used on the LAN when the value of the LANType property is equal to 1 (i.e., "Other"). This property is deprecated since its purpose overlaps with OtherTypeDescription, which which is inherited from ProtocolEndpoint.
    #[serde(rename = "OtherLANType")]
    pub other_lantype: Option<String>,
}

impl CIM_LANEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            alias_addresses: Vec::new(),
            group_addresses: Vec::new(),
            lanid: None,
            lantype: None,
            macaddress: None,
            max_data_size: None,
            other_lantype: None,
        }
    }


    /// Sets the value of AliasAddresses
    pub fn set_alias_addresses(&mut self, value: Vec<String>) {
        self.alias_addresses = value;
    }

    /// Gets the value of AliasAddresses
    pub fn get_alias_addresses(&self) -> &Vec<String> {
        &self.alias_addresses
    }

    /// Sets the value of GroupAddresses
    pub fn set_group_addresses(&mut self, value: Vec<String>) {
        self.group_addresses = value;
    }

    /// Gets the value of GroupAddresses
    pub fn get_group_addresses(&self) -> &Vec<String> {
        &self.group_addresses
    }

    /// Sets the value of LANID
    pub fn set_lanid(&mut self, value: String) {
        self.lanid = Some(value);
    }

    /// Gets the value of LANID
    pub fn get_lanid(&self) -> Option<&String> {
        self.lanid.as_ref()
    }

    /// Sets the value of LANType
    pub fn set_lantype(&mut self, value: LANEndpoint_LANType) {
        self.lantype = Some(value);
    }

    /// Gets the value of LANType
    pub fn get_lantype(&self) -> Option<&LANEndpoint_LANType> {
        self.lantype.as_ref()
    }

    /// Sets the value of MACAddress
    pub fn set_macaddress(&mut self, value: String) {
        self.macaddress = Some(value);
    }

    /// Gets the value of MACAddress
    pub fn get_macaddress(&self) -> Option<&String> {
        self.macaddress.as_ref()
    }

    /// Sets the value of MaxDataSize
    pub fn set_max_data_size(&mut self, value: u32) {
        self.max_data_size = Some(value);
    }

    /// Gets the value of MaxDataSize
    pub fn get_max_data_size(&self) -> Option<&u32> {
        self.max_data_size.as_ref()
    }

    /// Sets the value of OtherLANType
    pub fn set_other_lantype(&mut self, value: String) {
        self.other_lantype = Some(value);
    }

    /// Gets the value of OtherLANType
    pub fn get_other_lantype(&self) -> Option<&String> {
        self.other_lantype.as_ref()
    }
}

