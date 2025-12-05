// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNatTransitionConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNatTransitionConfiguration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "InboundInterface")]
    pub inbound_interface: Vec<String>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "IPv4AddressPortPool")]
    pub ipv4_address_port_pool: Vec<String>,

/// 
    #[serde(rename = "OutboundInterface")]
    pub outbound_interface: Vec<String>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<u32>,

/// 
    #[serde(rename = "PrefixMapping")]
    pub prefix_mapping: Vec<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "TcpMappingTimeout")]
    pub tcp_mapping_timeout: Option<u32>,
}

impl MSFT_NetNatTransitionConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            inbound_interface: Vec::new(),
            instance_name: None,
            ipv4_address_port_pool: Vec::new(),
            outbound_interface: Vec::new(),
            policy_store: None,
            prefix_mapping: Vec::new(),
            state: None,
            tcp_mapping_timeout: None,
        }
    }


    /// Sets the value of InboundInterface
    pub fn set_inbound_interface(&mut self, value: Vec<String>) {
        self.inbound_interface = value;
    }

    /// Gets the value of InboundInterface
    pub fn get_inbound_interface(&self) -> &Vec<String> {
        &self.inbound_interface
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of IPv4AddressPortPool
    pub fn set_ipv4_address_port_pool(&mut self, value: Vec<String>) {
        self.ipv4_address_port_pool = value;
    }

    /// Gets the value of IPv4AddressPortPool
    pub fn get_ipv4_address_port_pool(&self) -> &Vec<String> {
        &self.ipv4_address_port_pool
    }

    /// Sets the value of OutboundInterface
    pub fn set_outbound_interface(&mut self, value: Vec<String>) {
        self.outbound_interface = value;
    }

    /// Gets the value of OutboundInterface
    pub fn get_outbound_interface(&self) -> &Vec<String> {
        &self.outbound_interface
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: u32) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&u32> {
        self.policy_store.as_ref()
    }

    /// Sets the value of PrefixMapping
    pub fn set_prefix_mapping(&mut self, value: Vec<String>) {
        self.prefix_mapping = value;
    }

    /// Gets the value of PrefixMapping
    pub fn get_prefix_mapping(&self) -> &Vec<String> {
        &self.prefix_mapping
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of TcpMappingTimeout
    pub fn set_tcp_mapping_timeout(&mut self, value: u32) {
        self.tcp_mapping_timeout = Some(value);
    }

    /// Gets the value of TcpMappingTimeout
    pub fn get_tcp_mapping_timeout(&self) -> Option<&u32> {
        self.tcp_mapping_timeout.as_ref()
    }

/// 

    /// * `pass_thru` -  (bool)

    /// * `output_object` -  (MSFT_NetNatTransitionConfiguration)
    /// * `return_value` -  (u32)
    pub fn enable(&self, pass_thru: bool, output_object: &mut MSFT_NetNatTransitionConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Enable", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }


/// 

    /// * `pass_thru` -  (bool)

    /// * `output_object` -  (MSFT_NetNatTransitionConfiguration)
    /// * `return_value` -  (u32)
    pub fn disable(&self, pass_thru: bool, output_object: &mut MSFT_NetNatTransitionConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Disable", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

