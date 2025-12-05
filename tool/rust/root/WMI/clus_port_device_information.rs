// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusPortDeviceInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusPortDeviceInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// Connected Node .
    #[serde(rename = "ConnectedNode")]
    pub connected_node: Option<String>,

/// Connected Node Device Number .
    #[serde(rename = "ConnectedNodeDeviceNumber")]
    pub connected_node_device_number: Option<u32>,

/// Connected Node Id.
    #[serde(rename = "ConnectedNodeId")]
    pub connected_node_id: Option<u32>,

/// Device Path attribute.
    #[serde(rename = "DeviceAttribute")]
    pub device_attribute: Option<u32>,

/// Device Guid .
    #[serde(rename = "DeviceGuid")]
    pub device_guid: Option<String>,

/// Device Number.
    #[serde(rename = "DeviceNumber")]
    pub device_number: Option<u32>,

/// ClusPort Device State.
    #[serde(rename = "DeviceState")]
    pub device_state: Option<u32>,

/// ClusPort Device Type.
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// Flags
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// IsReadOnly.
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// Number of Multi-Paths.
    #[serde(rename = "NumberOfPaths")]
    pub number_of_paths: Option<u32>,

/// Path Info.
    #[serde(rename = "Paths")]
    pub paths: Vec<ClusPortPathInformation>,

/// ProductId
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,

/// SerialNumber
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// VendorId
    #[serde(rename = "VendorId")]
    pub vendor_id: Option<String>,
}

impl ClusPortDeviceInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            connected_node: None,
            connected_node_device_number: None,
            connected_node_id: None,
            device_attribute: None,
            device_guid: None,
            device_number: None,
            device_state: None,
            device_type: None,
            flags: None,
            instance_name: None,
            is_read_only: None,
            number_of_paths: None,
            paths: Vec::new(),
            product_id: None,
            serial_number: None,
            vendor_id: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of ConnectedNode
    pub fn set_connected_node(&mut self, value: String) {
        self.connected_node = Some(value);
    }

    /// Gets the value of ConnectedNode
    pub fn get_connected_node(&self) -> Option<&String> {
        self.connected_node.as_ref()
    }

    /// Sets the value of ConnectedNodeDeviceNumber
    pub fn set_connected_node_device_number(&mut self, value: u32) {
        self.connected_node_device_number = Some(value);
    }

    /// Gets the value of ConnectedNodeDeviceNumber
    pub fn get_connected_node_device_number(&self) -> Option<&u32> {
        self.connected_node_device_number.as_ref()
    }

    /// Sets the value of ConnectedNodeId
    pub fn set_connected_node_id(&mut self, value: u32) {
        self.connected_node_id = Some(value);
    }

    /// Gets the value of ConnectedNodeId
    pub fn get_connected_node_id(&self) -> Option<&u32> {
        self.connected_node_id.as_ref()
    }

    /// Sets the value of DeviceAttribute
    pub fn set_device_attribute(&mut self, value: u32) {
        self.device_attribute = Some(value);
    }

    /// Gets the value of DeviceAttribute
    pub fn get_device_attribute(&self) -> Option<&u32> {
        self.device_attribute.as_ref()
    }

    /// Sets the value of DeviceGuid
    pub fn set_device_guid(&mut self, value: String) {
        self.device_guid = Some(value);
    }

    /// Gets the value of DeviceGuid
    pub fn get_device_guid(&self) -> Option<&String> {
        self.device_guid.as_ref()
    }

    /// Sets the value of DeviceNumber
    pub fn set_device_number(&mut self, value: u32) {
        self.device_number = Some(value);
    }

    /// Gets the value of DeviceNumber
    pub fn get_device_number(&self) -> Option<&u32> {
        self.device_number.as_ref()
    }

    /// Sets the value of DeviceState
    pub fn set_device_state(&mut self, value: u32) {
        self.device_state = Some(value);
    }

    /// Gets the value of DeviceState
    pub fn get_device_state(&self) -> Option<&u32> {
        self.device_state.as_ref()
    }

    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of IsReadOnly
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = Some(value);
    }

    /// Gets the value of IsReadOnly
    pub fn get_is_read_only(&self) -> Option<&bool> {
        self.is_read_only.as_ref()
    }

    /// Sets the value of NumberOfPaths
    pub fn set_number_of_paths(&mut self, value: u32) {
        self.number_of_paths = Some(value);
    }

    /// Gets the value of NumberOfPaths
    pub fn get_number_of_paths(&self) -> Option<&u32> {
        self.number_of_paths.as_ref()
    }

    /// Sets the value of Paths
    pub fn set_paths(&mut self, value: Vec<ClusPortPathInformation>) {
        self.paths = value;
    }

    /// Gets the value of Paths
    pub fn get_paths(&self) -> &Vec<ClusPortPathInformation> {
        &self.paths
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

    /// Sets the value of VendorId
    pub fn set_vendor_id(&mut self, value: String) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorId
    pub fn get_vendor_id(&self) -> Option<&String> {
        self.vendor_id.as_ref()
    }
}

