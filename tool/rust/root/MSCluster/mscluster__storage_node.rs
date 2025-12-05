// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_StorageNode struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_StorageNode {

/// 
    #[serde(rename = "ConnectionString")]
    pub connection_string: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "ManufacturerId")]
    pub manufacturer_id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "StorageNodeHealth")]
    pub storage_node_health: Option<u32>,

/// 
    #[serde(rename = "StorageNodeOperationalStatus")]
    pub storage_node_operational_status: Option<u32>,
}

impl MSCluster_StorageNode {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_string: None,
            description: None,
            id: None,
            location: None,
            manufacturer_id: None,
            name: None,
            product_id: None,
            serial_number: None,
            state: None,
            storage_node_health: None,
            storage_node_operational_status: None,
        }
    }


    /// Sets the value of ConnectionString
    pub fn set_connection_string(&mut self, value: String) {
        self.connection_string = Some(value);
    }

    /// Gets the value of ConnectionString
    pub fn get_connection_string(&self) -> Option<&String> {
        self.connection_string.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of ManufacturerId
    pub fn set_manufacturer_id(&mut self, value: String) {
        self.manufacturer_id = Some(value);
    }

    /// Gets the value of ManufacturerId
    pub fn get_manufacturer_id(&self) -> Option<&String> {
        self.manufacturer_id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ProductId
    pub fn set_product_id(&mut self, value: String) {
        self.product_id = Some(value);
    }

    /// Gets the value of ProductId
    pub fn get_product_id(&self) -> Option<&String> {
        self.product_id.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of StorageNodeHealth
    pub fn set_storage_node_health(&mut self, value: u32) {
        self.storage_node_health = Some(value);
    }

    /// Gets the value of StorageNodeHealth
    pub fn get_storage_node_health(&self) -> Option<&u32> {
        self.storage_node_health.as_ref()
    }

    /// Sets the value of StorageNodeOperationalStatus
    pub fn set_storage_node_operational_status(&mut self, value: u32) {
        self.storage_node_operational_status = Some(value);
    }

    /// Gets the value of StorageNodeOperationalStatus
    pub fn get_storage_node_operational_status(&self) -> Option<&u32> {
        self.storage_node_operational_status.as_ref()
    }

/// 

    /// * `description` -  (String)
    /// * `flags` -  (u32)
    /// * `location` -  (String)
    /// * `name` -  (String)

    /// * `added_storage_node` -  (MSCluster_StorageNode)
    /// * `return_value` -  (u32)
    pub fn add_storage_node(&self, name: &String, description: &String, location: &String, flags: u32, added_storage_node: &mut MSCluster_StorageNode) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Location".to_string(), value: location.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("AddStorageNode", &args)?;
        let added_storage_node = result.get_value("AddedStorageNode")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)
    /// * `flags` -  (u32)
    /// * `location` -  (String)
    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_storage_node(&self, new_name: &String, description: &String, location: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Location".to_string(), value: location.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("SetStorageNode", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `parent` -  (MSCluster_FaultDomain)
    /// * `return_value` -  (u32)
    pub fn get_parent(&self, parent: &mut MSCluster_FaultDomain, flags: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = flags {
            args.push(MethodParameter { name: "Flags".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetParent", &args)?;
        let parent = result.get_value("Parent")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_storage_node(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveStorageNode", &args)

    }

}

