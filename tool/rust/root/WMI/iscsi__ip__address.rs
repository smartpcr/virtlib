// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_IP_Address struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_IP_Address {

/// 
    #[serde(rename = "IpV4Address")]
    pub ip_v4_address: Option<u32>,

/// 
    #[serde(rename = "IpV6Address")]
    pub ip_v6_address: Vec<u8>,

/// 
    #[serde(rename = "IpV6FlowInfo")]
    pub ip_v6_flow_info: Option<u32>,

/// 
    #[serde(rename = "IpV6ScopeId")]
    pub ip_v6_scope_id: Option<u32>,

/// 
    #[serde(rename = "TextAddress")]
    pub text_address: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<Address_Type>,
}

impl ISCSI_IP_Address {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            ip_v4_address: None,
            ip_v6_address: Vec::new(),
            ip_v6_flow_info: None,
            ip_v6_scope_id: None,
            text_address: None,
            type: None,
        }
    }


    /// Sets the value of IpV4Address
    pub fn set_ip_v4_address(&mut self, value: u32) {
        self.ip_v4_address = Some(value);
    }

    /// Gets the value of IpV4Address
    pub fn get_ip_v4_address(&self) -> Option<&u32> {
        self.ip_v4_address.as_ref()
    }

    /// Sets the value of IpV6Address
    pub fn set_ip_v6_address(&mut self, value: Vec<u8>) {
        self.ip_v6_address = value;
    }

    /// Gets the value of IpV6Address
    pub fn get_ip_v6_address(&self) -> &Vec<u8> {
        &self.ip_v6_address
    }

    /// Sets the value of IpV6FlowInfo
    pub fn set_ip_v6_flow_info(&mut self, value: u32) {
        self.ip_v6_flow_info = Some(value);
    }

    /// Gets the value of IpV6FlowInfo
    pub fn get_ip_v6_flow_info(&self) -> Option<&u32> {
        self.ip_v6_flow_info.as_ref()
    }

    /// Sets the value of IpV6ScopeId
    pub fn set_ip_v6_scope_id(&mut self, value: u32) {
        self.ip_v6_scope_id = Some(value);
    }

    /// Gets the value of IpV6ScopeId
    pub fn get_ip_v6_scope_id(&self) -> Option<&u32> {
        self.ip_v6_scope_id.as_ref()
    }

    /// Sets the value of TextAddress
    pub fn set_text_address(&mut self, value: String) {
        self.text_address = Some(value);
    }

    /// Gets the value of TextAddress
    pub fn get_text_address(&self) -> Option<&String> {
        self.text_address.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: Address_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&Address_Type> {
        self.type.as_ref()
    }
}

