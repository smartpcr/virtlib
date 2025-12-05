// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_NetworkInterface struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_NetworkInterface {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "Adapter")]
    pub adapter: Option<String>,

/// 
    #[serde(rename = "AdapterId")]
    pub adapter_id: Option<String>,

/// 
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// 
    #[serde(rename = "Characteristics")]
    pub characteristics: Option<u32>,

/// 
    #[serde(rename = "DhcpEnabled")]
    pub dhcp_enabled: Option<bool>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IPv4Addresses")]
    pub ipv4_addresses: Vec<String>,

/// 
    #[serde(rename = "IPv6Addresses")]
    pub ipv6_addresses: Vec<String>,

/// 
    #[serde(rename = "Network")]
    pub network: Option<String>,

/// 
    #[serde(rename = "Node")]
    pub node: Option<String>,

/// 
    #[serde(rename = "PrivateProperties")]
    pub private_properties: Option<MSCluster_Property>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSCluster_NetworkInterface {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            adapter: None,
            adapter_id: None,
            address: None,
            characteristics: None,
            dhcp_enabled: None,
            flags: None,
            id: None,
            ipv4_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            network: None,
            node: None,
            private_properties: None,
            state: None,
        }
    }


    /// Sets the value of Adapter
    pub fn set_adapter(&mut self, value: String) {
        self.adapter = Some(value);
    }

    /// Gets the value of Adapter
    pub fn get_adapter(&self) -> Option<&String> {
        self.adapter.as_ref()
    }

    /// Sets the value of AdapterId
    pub fn set_adapter_id(&mut self, value: String) {
        self.adapter_id = Some(value);
    }

    /// Gets the value of AdapterId
    pub fn get_adapter_id(&self) -> Option<&String> {
        self.adapter_id.as_ref()
    }

    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of Characteristics
    pub fn set_characteristics(&mut self, value: u32) {
        self.characteristics = Some(value);
    }

    /// Gets the value of Characteristics
    pub fn get_characteristics(&self) -> Option<&u32> {
        self.characteristics.as_ref()
    }

    /// Sets the value of DhcpEnabled
    pub fn set_dhcp_enabled(&mut self, value: bool) {
        self.dhcp_enabled = Some(value);
    }

    /// Gets the value of DhcpEnabled
    pub fn get_dhcp_enabled(&self) -> Option<&bool> {
        self.dhcp_enabled.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IPv4Addresses
    pub fn set_ipv4_addresses(&mut self, value: Vec<String>) {
        self.ipv4_addresses = value;
    }

    /// Gets the value of IPv4Addresses
    pub fn get_ipv4_addresses(&self) -> &Vec<String> {
        &self.ipv4_addresses
    }

    /// Sets the value of IPv6Addresses
    pub fn set_ipv6_addresses(&mut self, value: Vec<String>) {
        self.ipv6_addresses = value;
    }

    /// Gets the value of IPv6Addresses
    pub fn get_ipv6_addresses(&self) -> &Vec<String> {
        &self.ipv6_addresses
    }

    /// Sets the value of Network
    pub fn set_network(&mut self, value: String) {
        self.network = Some(value);
    }

    /// Gets the value of Network
    pub fn get_network(&self) -> Option<&String> {
        self.network.as_ref()
    }

    /// Sets the value of Node
    pub fn set_node(&mut self, value: String) {
        self.node = Some(value);
    }

    /// Gets the value of Node
    pub fn get_node(&self) -> Option<&String> {
        self.node.as_ref()
    }

    /// Sets the value of PrivateProperties
    pub fn set_private_properties(&mut self, value: MSCluster_Property) {
        self.private_properties = Some(value);
    }

    /// Gets the value of PrivateProperties
    pub fn get_private_properties(&self) -> Option<&MSCluster_Property> {
        self.private_properties.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

/// 

    /// * `control_code` -  (i32)
    /// * `input_buffer` -  (u8[])
    /// * `reason` -  (String)

    /// * `output_buffer` -  (u8[])
    /// * `output_buffer_size` -  (i32)
    pub fn execute_network_interface_control(&self, control_code: i32, input_buffer: &Vec<u8>, output_buffer: &mut Vec<u8>, output_buffer_size: &mut i32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ExecuteNetworkInterfaceControl", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_buffer_size = result.get_value("OutputBufferSize")?;
        Ok(result.return_value)

    }

}

