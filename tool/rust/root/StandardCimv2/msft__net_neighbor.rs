// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNeighbor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNeighbor {
    #[serde(flatten)]
    pub base: CIM_RemoteServiceAccessPoint,

/// 
    #[serde(rename = "AddressFamily")]
    pub address_family: Option<u16>,

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
    #[serde(rename = "LinkLayerAddress")]
    pub link_layer_address: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u8>,

/// 
    #[serde(rename = "Store")]
    pub store: Option<u8>,
}

impl MSFT_NetNeighbor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RemoteServiceAccessPoint::new(),
            address_family: None,
            interface_alias: None,
            interface_index: None,
            ipaddress: None,
            link_layer_address: None,
            state: None,
            store: None,
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

    /// Sets the value of LinkLayerAddress
    pub fn set_link_layer_address(&mut self, value: String) {
        self.link_layer_address = Some(value);
    }

    /// Gets the value of LinkLayerAddress
    pub fn get_link_layer_address(&self) -> Option<&String> {
        self.link_layer_address.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u8) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u8> {
        self.state.as_ref()
    }

    /// Sets the value of Store
    pub fn set_store(&mut self, value: u8) {
        self.store = Some(value);
    }

    /// Gets the value of Store
    pub fn get_store(&self) -> Option<&u8> {
        self.store.as_ref()
    }

/// 

    /// * `address_family` -  (u16)
    /// * `interface_alias` -  (String)
    /// * `interface_index` -  (u32)
    /// * `ipaddress` -  (String)
    /// * `link_layer_address` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `policy_store` -  (String)
    /// * `state` -  (u8)

    /// * `cmdlet_output` -  (MSFT_NetNeighbor[])
    /// * `return_value` -  (u32)
    pub fn create(&self, interface_index: u32, interface_alias: &String, ipaddress: &String, policy_store: &String, link_layer_address: &String, state: u8, address_family: u16, pass_thru: bool, cmdlet_output: &mut Vec<MSFT_NetNeighbor>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InterfaceIndex".to_string(), value: interface_index.into() });
        args.push(MethodParameter { name: "InterfaceAlias".to_string(), value: interface_alias.into() });
        args.push(MethodParameter { name: "IPAddress".to_string(), value: ipaddress.into() });
        args.push(MethodParameter { name: "PolicyStore".to_string(), value: policy_store.into() });
        args.push(MethodParameter { name: "LinkLayerAddress".to_string(), value: link_layer_address.into() });
        args.push(MethodParameter { name: "State".to_string(), value: state.into() });
        args.push(MethodParameter { name: "AddressFamily".to_string(), value: address_family.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Create", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

