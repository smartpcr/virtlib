// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_Network struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_Network {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// 
    #[serde(rename = "AddressMask")]
    pub address_mask: Option<String>,

/// 
    #[serde(rename = "AutoMetric")]
    pub auto_metric: Option<bool>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IPv4Addresses")]
    pub ipv4_addresses: Vec<String>,

/// 
    #[serde(rename = "IPv4PrefixLengths")]
    pub ipv4_prefix_lengths: Vec<String>,

/// 
    #[serde(rename = "IPv6Addresses")]
    pub ipv6_addresses: Vec<String>,

/// 
    #[serde(rename = "IPv6PrefixLengths")]
    pub ipv6_prefix_lengths: Vec<String>,

/// 
    #[serde(rename = "Metric")]
    pub metric: Option<u32>,

/// 
    #[serde(rename = "PrivateProperties")]
    pub private_properties: Option<MSCluster_Property>,

/// 
    #[serde(rename = "Role")]
    pub role: Option<u32>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSCluster_Network {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            address: None,
            address_mask: None,
            auto_metric: None,
            id: None,
            ipv4_addresses: Vec::new(),
            ipv4_prefix_lengths: Vec::new(),
            ipv6_addresses: Vec::new(),
            ipv6_prefix_lengths: Vec::new(),
            metric: None,
            private_properties: None,
            role: None,
            state: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of AddressMask
    pub fn set_address_mask(&mut self, value: String) {
        self.address_mask = Some(value);
    }

    /// Gets the value of AddressMask
    pub fn get_address_mask(&self) -> Option<&String> {
        self.address_mask.as_ref()
    }

    /// Sets the value of AutoMetric
    pub fn set_auto_metric(&mut self, value: bool) {
        self.auto_metric = Some(value);
    }

    /// Gets the value of AutoMetric
    pub fn get_auto_metric(&self) -> Option<&bool> {
        self.auto_metric.as_ref()
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

    /// Sets the value of IPv4PrefixLengths
    pub fn set_ipv4_prefix_lengths(&mut self, value: Vec<String>) {
        self.ipv4_prefix_lengths = value;
    }

    /// Gets the value of IPv4PrefixLengths
    pub fn get_ipv4_prefix_lengths(&self) -> &Vec<String> {
        &self.ipv4_prefix_lengths
    }

    /// Sets the value of IPv6Addresses
    pub fn set_ipv6_addresses(&mut self, value: Vec<String>) {
        self.ipv6_addresses = value;
    }

    /// Gets the value of IPv6Addresses
    pub fn get_ipv6_addresses(&self) -> &Vec<String> {
        &self.ipv6_addresses
    }

    /// Sets the value of IPv6PrefixLengths
    pub fn set_ipv6_prefix_lengths(&mut self, value: Vec<String>) {
        self.ipv6_prefix_lengths = value;
    }

    /// Gets the value of IPv6PrefixLengths
    pub fn get_ipv6_prefix_lengths(&self) -> &Vec<String> {
        &self.ipv6_prefix_lengths
    }

    /// Sets the value of Metric
    pub fn set_metric(&mut self, value: u32) {
        self.metric = Some(value);
    }

    /// Gets the value of Metric
    pub fn get_metric(&self) -> Option<&u32> {
        self.metric.as_ref()
    }

    /// Sets the value of PrivateProperties
    pub fn set_private_properties(&mut self, value: MSCluster_Property) {
        self.private_properties = Some(value);
    }

    /// Gets the value of PrivateProperties
    pub fn get_private_properties(&self) -> Option<&MSCluster_Property> {
        self.private_properties.as_ref()
    }

    /// Sets the value of Role
    pub fn set_role(&mut self, value: u32) {
        self.role = Some(value);
    }

    /// Gets the value of Role
    pub fn get_role(&self) -> Option<&u32> {
        self.role.as_ref()
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

    /// * `new_name` -  (String)
    /// * `reason` -  (String)
    pub fn rename(&self, new_name: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `control_code` -  (i32)
    /// * `input_buffer` -  (u8[])
    /// * `reason` -  (String)

    /// * `output_buffer` -  (u8[])
    /// * `output_buffer_size` -  (i32)
    pub fn execute_network_control(&self, control_code: i32, input_buffer: &Vec<u8>, output_buffer: &mut Vec<u8>, output_buffer_size: &mut i32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ExecuteNetworkControl", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_buffer_size = result.get_value("OutputBufferSize")?;
        Ok(result.return_value)

    }

}

