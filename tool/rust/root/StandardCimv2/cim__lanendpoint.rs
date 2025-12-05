// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LANEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LANEndpoint {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// 
    #[serde(rename = "AliasAddresses")]
    pub alias_addresses: Vec<String>,

/// 
    #[serde(rename = "GroupAddresses")]
    pub group_addresses: Vec<String>,

/// 
    #[serde(rename = "LANID")]
    pub lanid: Option<String>,

/// 
    #[serde(rename = "LANType")]
    pub lantype: Option<u16>,

/// 
    #[serde(rename = "MACAddress")]
    pub macaddress: Option<String>,

/// 
    #[serde(rename = "MaxDataSize")]
    pub max_data_size: Option<u32>,

/// 
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
    pub fn set_lantype(&mut self, value: u16) {
        self.lantype = Some(value);
    }

    /// Gets the value of LANType
    pub fn get_lantype(&self) -> Option<&u16> {
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

