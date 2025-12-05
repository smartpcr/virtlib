// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_InitiatorPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_InitiatorPort {

/// 
    #[serde(rename = "AlternateNodeAddress")]
    pub alternate_node_address: Vec<String>,

/// 
    #[serde(rename = "AlternatePortAddress")]
    pub alternate_port_address: Vec<String>,

/// 
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<u16>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NodeAddress")]
    pub node_address: Option<String>,

/// 
    #[serde(rename = "ObjectId")]
    pub object_id: Option<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OtherConnectionTypeDescription")]
    pub other_connection_type_description: Option<String>,

/// 
    #[serde(rename = "PortAddress")]
    pub port_address: Option<String>,

/// 
    #[serde(rename = "PortType")]
    pub port_type: Option<u16>,
}

impl MSFT_InitiatorPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alternate_node_address: Vec::new(),
            alternate_port_address: Vec::new(),
            connection_type: None,
            instance_name: None,
            node_address: None,
            object_id: None,
            operational_status: Vec::new(),
            other_connection_type_description: None,
            port_address: None,
            port_type: None,
        }
    }


    /// Sets the value of AlternateNodeAddress
    pub fn set_alternate_node_address(&mut self, value: Vec<String>) {
        self.alternate_node_address = value;
    }

    /// Gets the value of AlternateNodeAddress
    pub fn get_alternate_node_address(&self) -> &Vec<String> {
        &self.alternate_node_address
    }

    /// Sets the value of AlternatePortAddress
    pub fn set_alternate_port_address(&mut self, value: Vec<String>) {
        self.alternate_port_address = value;
    }

    /// Gets the value of AlternatePortAddress
    pub fn get_alternate_port_address(&self) -> &Vec<String> {
        &self.alternate_port_address
    }

    /// Sets the value of ConnectionType
    pub fn set_connection_type(&mut self, value: u16) {
        self.connection_type = Some(value);
    }

    /// Gets the value of ConnectionType
    pub fn get_connection_type(&self) -> Option<&u16> {
        self.connection_type.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NodeAddress
    pub fn set_node_address(&mut self, value: String) {
        self.node_address = Some(value);
    }

    /// Gets the value of NodeAddress
    pub fn get_node_address(&self) -> Option<&String> {
        self.node_address.as_ref()
    }

    /// Sets the value of ObjectId
    pub fn set_object_id(&mut self, value: String) {
        self.object_id = Some(value);
    }

    /// Gets the value of ObjectId
    pub fn get_object_id(&self) -> Option<&String> {
        self.object_id.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of OtherConnectionTypeDescription
    pub fn set_other_connection_type_description(&mut self, value: String) {
        self.other_connection_type_description = Some(value);
    }

    /// Gets the value of OtherConnectionTypeDescription
    pub fn get_other_connection_type_description(&self) -> Option<&String> {
        self.other_connection_type_description.as_ref()
    }

    /// Sets the value of PortAddress
    pub fn set_port_address(&mut self, value: String) {
        self.port_address = Some(value);
    }

    /// Gets the value of PortAddress
    pub fn get_port_address(&self) -> Option<&String> {
        self.port_address.as_ref()
    }

    /// Sets the value of PortType
    pub fn set_port_type(&mut self, value: u16) {
        self.port_type = Some(value);
    }

    /// Gets the value of PortType
    pub fn get_port_type(&self) -> Option<&u16> {
        self.port_type.as_ref()
    }

/// 

    /// * `node_address` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_node_address(&self, node_address: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeAddress".to_string(), value: node_address.into() });

        let result = self.invoke_method("SetNodeAddress", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

