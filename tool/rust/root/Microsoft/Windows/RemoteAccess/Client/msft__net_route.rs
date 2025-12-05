// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetRoute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetRoute {
    #[serde(flatten)]
    pub base: CIM_NextHopRoute,

/// 
    #[serde(rename = "AddressFamily")]
    pub address_family: Option<u16>,

/// 
    #[serde(rename = "DestinationPrefix")]
    pub destination_prefix: Option<String>,

/// 
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "NextHop")]
    pub next_hop: Option<String>,

/// 
    #[serde(rename = "PreferredLifetime")]
    pub preferred_lifetime: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<u16>,

/// 
    #[serde(rename = "Publish")]
    pub publish: Option<u8>,

/// 
    #[serde(rename = "Store")]
    pub store: Option<u8>,

/// 
    #[serde(rename = "ValidLifetime")]
    pub valid_lifetime: Option<String>,
}

impl MSFT_NetRoute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NextHopRoute::new(),
            address_family: None,
            destination_prefix: None,
            interface_alias: None,
            interface_index: None,
            next_hop: None,
            preferred_lifetime: None,
            protocol: None,
            publish: None,
            store: None,
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

    /// Sets the value of DestinationPrefix
    pub fn set_destination_prefix(&mut self, value: String) {
        self.destination_prefix = Some(value);
    }

    /// Gets the value of DestinationPrefix
    pub fn get_destination_prefix(&self) -> Option<&String> {
        self.destination_prefix.as_ref()
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

    /// Sets the value of NextHop
    pub fn set_next_hop(&mut self, value: String) {
        self.next_hop = Some(value);
    }

    /// Gets the value of NextHop
    pub fn get_next_hop(&self) -> Option<&String> {
        self.next_hop.as_ref()
    }

    /// Sets the value of PreferredLifetime
    pub fn set_preferred_lifetime(&mut self, value: String) {
        self.preferred_lifetime = Some(value);
    }

    /// Gets the value of PreferredLifetime
    pub fn get_preferred_lifetime(&self) -> Option<&String> {
        self.preferred_lifetime.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: u16) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&u16> {
        self.protocol.as_ref()
    }

    /// Sets the value of Publish
    pub fn set_publish(&mut self, value: u8) {
        self.publish = Some(value);
    }

    /// Gets the value of Publish
    pub fn get_publish(&self) -> Option<&u8> {
        self.publish.as_ref()
    }

    /// Sets the value of Store
    pub fn set_store(&mut self, value: u8) {
        self.store = Some(value);
    }

    /// Gets the value of Store
    pub fn get_store(&self) -> Option<&u8> {
        self.store.as_ref()
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
    /// * `destination_prefix` -  (String)
    /// * `interface_alias` -  (String)
    /// * `interface_index` -  (u32)
    /// * `next_hop` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `policy_store` -  (String)
    /// * `preferred_lifetime` -  (String)
    /// * `protocol` -  (u16)
    /// * `publish` -  (u8)
    /// * `route_metric` -  (u16)
    /// * `valid_lifetime` -  (String)

    /// * `cmdlet_output` -  (MSFT_NetRoute[])
    /// * `return_value` -  (u32)
    pub fn create(&self, interface_index: u32, interface_alias: &String, destination_prefix: &String, next_hop: &String, publish: u8, route_metric: u16, protocol: u16, valid_lifetime: &String, preferred_lifetime: &String, policy_store: &String, address_family: u16, pass_thru: bool, cmdlet_output: &mut Vec<MSFT_NetRoute>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InterfaceIndex".to_string(), value: interface_index.into() });
        args.push(MethodParameter { name: "InterfaceAlias".to_string(), value: interface_alias.into() });
        args.push(MethodParameter { name: "DestinationPrefix".to_string(), value: destination_prefix.into() });
        args.push(MethodParameter { name: "NextHop".to_string(), value: next_hop.into() });
        args.push(MethodParameter { name: "Publish".to_string(), value: publish.into() });
        args.push(MethodParameter { name: "RouteMetric".to_string(), value: route_metric.into() });
        args.push(MethodParameter { name: "Protocol".to_string(), value: protocol.into() });
        args.push(MethodParameter { name: "ValidLifetime".to_string(), value: valid_lifetime.into() });
        args.push(MethodParameter { name: "PreferredLifetime".to_string(), value: preferred_lifetime.into() });
        args.push(MethodParameter { name: "PolicyStore".to_string(), value: policy_store.into() });
        args.push(MethodParameter { name: "AddressFamily".to_string(), value: address_family.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Create", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `interface_index` -  (u32)
    /// * `local_ipaddress` -  (String)
    /// * `remote_ipaddress` -  (String)

    /// * `cmdlet_output` -  (CIM_ManagedElement[])
    /// * `return_value` -  (u32)
    pub fn select(&self, interface_index: u32, local_ipaddress: &String, remote_ipaddress: &String, cmdlet_output: &mut Vec<CIM_ManagedElement>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InterfaceIndex".to_string(), value: interface_index.into() });
        args.push(MethodParameter { name: "LocalIPAddress".to_string(), value: local_ipaddress.into() });
        args.push(MethodParameter { name: "RemoteIPAddress".to_string(), value: remote_ipaddress.into() });

        let result = self.invoke_method("Select", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

