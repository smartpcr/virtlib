// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPAddress {
    #[serde(flatten)]
    pub base: CIM_IPProtocolEndpoint,

/// 
    #[serde(rename = "AddressFamily")]
    pub address_family: Option<u16>,

/// 
    #[serde(rename = "AddressState")]
    pub address_state: Option<u16>,

/// 
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "IPAddress")]
    pub ipaddress: Option<String>,

/// 
    #[serde(rename = "PreferredLifetime")]
    pub preferred_lifetime: Option<String>,

/// 
    #[serde(rename = "PrefixOrigin")]
    pub prefix_origin: Option<u16>,

/// 
    #[serde(rename = "SkipAsSource")]
    pub skip_as_source: Option<bool>,

/// 
    #[serde(rename = "Store")]
    pub store: Option<u8>,

/// 
    #[serde(rename = "SuffixOrigin")]
    pub suffix_origin: Option<u16>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u8>,

/// 
    #[serde(rename = "ValidLifetime")]
    pub valid_lifetime: Option<String>,
}

impl MSFT_NetIPAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_IPProtocolEndpoint::new(),
            address_family: None,
            address_state: None,
            interface_alias: None,
            interface_index: None,
            ipaddress: None,
            preferred_lifetime: None,
            prefix_origin: None,
            skip_as_source: None,
            store: None,
            suffix_origin: None,
            type: None,
            valid_lifetime: None,
        }
    }


    /// Sets the value of AddressFamily
    pub fn set_address_family(&mut self, value: u16) {
        self.address_family = Some(value);
    }

    /// Gets the value of AddressFamily
    pub fn get_address_family(&self) -> Option<&u16> {
        self.address_family.as_ref()
    }

    /// Sets the value of AddressState
    pub fn set_address_state(&mut self, value: u16) {
        self.address_state = Some(value);
    }

    /// Gets the value of AddressState
    pub fn get_address_state(&self) -> Option<&u16> {
        self.address_state.as_ref()
    }

    /// Sets the value of InterfaceAlias
    pub fn set_interface_alias(&mut self, value: String) {
        self.interface_alias = Some(value);
    }

    /// Gets the value of InterfaceAlias
    pub fn get_interface_alias(&self) -> Option<&String> {
        self.interface_alias.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: String) {
        self.ipaddress = Some(value);
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> Option<&String> {
        self.ipaddress.as_ref()
    }

    /// Sets the value of PreferredLifetime
    pub fn set_preferred_lifetime(&mut self, value: String) {
        self.preferred_lifetime = Some(value);
    }

    /// Gets the value of PreferredLifetime
    pub fn get_preferred_lifetime(&self) -> Option<&String> {
        self.preferred_lifetime.as_ref()
    }

    /// Sets the value of PrefixOrigin
    pub fn set_prefix_origin(&mut self, value: u16) {
        self.prefix_origin = Some(value);
    }

    /// Gets the value of PrefixOrigin
    pub fn get_prefix_origin(&self) -> Option<&u16> {
        self.prefix_origin.as_ref()
    }

    /// Sets the value of SkipAsSource
    pub fn set_skip_as_source(&mut self, value: bool) {
        self.skip_as_source = Some(value);
    }

    /// Gets the value of SkipAsSource
    pub fn get_skip_as_source(&self) -> Option<&bool> {
        self.skip_as_source.as_ref()
    }

    /// Sets the value of Store
    pub fn set_store(&mut self, value: u8) {
        self.store = Some(value);
    }

    /// Gets the value of Store
    pub fn get_store(&self) -> Option<&u8> {
        self.store.as_ref()
    }

    /// Sets the value of SuffixOrigin
    pub fn set_suffix_origin(&mut self, value: u16) {
        self.suffix_origin = Some(value);
    }

    /// Gets the value of SuffixOrigin
    pub fn get_suffix_origin(&self) -> Option<&u16> {
        self.suffix_origin.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u8) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u8> {
        self.type.as_ref()
    }

    /// Sets the value of ValidLifetime
    pub fn set_valid_lifetime(&mut self, value: String) {
        self.valid_lifetime = Some(value);
    }

    /// Gets the value of ValidLifetime
    pub fn get_valid_lifetime(&self) -> Option<&String> {
        self.valid_lifetime.as_ref()
    }

/// 

    /// * `address_family` -  (u16)
    /// * `address_state` -  (u16)
    /// * `default_gateway` -  (String)
    /// * `interface_alias` -  (String)
    /// * `interface_index` -  (u32)
    /// * `ipaddress` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `policy_store` -  (String)
    /// * `preferred_lifetime` -  (String)
    /// * `prefix_length` -  (u8)
    /// * `prefix_origin` -  (u16)
    /// * `skip_as_source` -  (bool)
    /// * `suffix_origin` -  (u16)
    /// * `type` -  (u8)
    /// * `valid_lifetime` -  (String)

    /// * `cmdlet_output` -  (MSFT_NetIPAddress[])
    /// * `return_value` -  (u32)
    pub fn create(&self, interface_index: u32, interface_alias: &String, ipaddress: &String, address_family: u16, prefix_length: u8, type: u8, prefix_origin: u16, suffix_origin: u16, address_state: u16, valid_lifetime: &String, preferred_lifetime: &String, skip_as_source: bool, default_gateway: &String, policy_store: &String, pass_thru: bool, cmdlet_output: &mut Vec<MSFT_NetIPAddress>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InterfaceIndex".to_string(), value: interface_index.into() });
        args.push(MethodParameter { name: "InterfaceAlias".to_string(), value: interface_alias.into() });
        args.push(MethodParameter { name: "IPAddress".to_string(), value: ipaddress.into() });
        args.push(MethodParameter { name: "AddressFamily".to_string(), value: address_family.into() });
        args.push(MethodParameter { name: "PrefixLength".to_string(), value: prefix_length.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "PrefixOrigin".to_string(), value: prefix_origin.into() });
        args.push(MethodParameter { name: "SuffixOrigin".to_string(), value: suffix_origin.into() });
        args.push(MethodParameter { name: "AddressState".to_string(), value: address_state.into() });
        args.push(MethodParameter { name: "ValidLifetime".to_string(), value: valid_lifetime.into() });
        args.push(MethodParameter { name: "PreferredLifetime".to_string(), value: preferred_lifetime.into() });
        args.push(MethodParameter { name: "SkipAsSource".to_string(), value: skip_as_source.into() });
        args.push(MethodParameter { name: "DefaultGateway".to_string(), value: default_gateway.into() });
        args.push(MethodParameter { name: "PolicyStore".to_string(), value: policy_store.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Create", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

