// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TargetPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TargetPort {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<u16>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "LinkTechnology")]
    pub link_technology: Option<u16>,

/// 
    #[serde(rename = "MaxSpeed")]
    pub max_speed: Option<u64>,

/// 
    #[serde(rename = "NetworkAddresses")]
    pub network_addresses: Vec<String>,

/// 
    #[serde(rename = "NodeAddress")]
    pub node_address: Option<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OtherConnectionTypeDescription")]
    pub other_connection_type_description: Option<String>,

/// 
    #[serde(rename = "OtherLinkTechnology")]
    pub other_link_technology: Option<String>,

/// 
    #[serde(rename = "OtherOperationalStatusDescription")]
    pub other_operational_status_description: Option<String>,

/// 
    #[serde(rename = "PortAddress")]
    pub port_address: Option<String>,

/// 
    #[serde(rename = "PortNumbers")]
    pub port_numbers: Vec<u16>,

/// 
    #[serde(rename = "PortType")]
    pub port_type: Option<u16>,

/// 
    #[serde(rename = "Role")]
    pub role: Option<u16>,

/// 
    #[serde(rename = "Speed")]
    pub speed: Option<u64>,

/// 
    #[serde(rename = "StorageControllerId")]
    pub storage_controller_id: Option<String>,

/// 
    #[serde(rename = "UsageRestriction")]
    pub usage_restriction: Option<u16>,
}

impl MSFT_TargetPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            connection_type: None,
            friendly_name: None,
            health_status: None,
            link_technology: None,
            max_speed: None,
            network_addresses: Vec::new(),
            node_address: None,
            operational_status: Vec::new(),
            other_connection_type_description: None,
            other_link_technology: None,
            other_operational_status_description: None,
            port_address: None,
            port_numbers: Vec::new(),
            port_type: None,
            role: None,
            speed: None,
            storage_controller_id: None,
            usage_restriction: None,
        }
    }


    /// Sets the value of ConnectionType
    pub fn set_connection_type(&mut self, value: u16) {
        self.connection_type = Some(value);
    }

    /// Gets the value of ConnectionType
    pub fn get_connection_type(&self) -> Option<&u16> {
        self.connection_type.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of LinkTechnology
    pub fn set_link_technology(&mut self, value: u16) {
        self.link_technology = Some(value);
    }

    /// Gets the value of LinkTechnology
    pub fn get_link_technology(&self) -> Option<&u16> {
        self.link_technology.as_ref()
    }

    /// Sets the value of MaxSpeed
    pub fn set_max_speed(&mut self, value: u64) {
        self.max_speed = Some(value);
    }

    /// Gets the value of MaxSpeed
    pub fn get_max_speed(&self) -> Option<&u64> {
        self.max_speed.as_ref()
    }

    /// Sets the value of NetworkAddresses
    pub fn set_network_addresses(&mut self, value: Vec<String>) {
        self.network_addresses = value;
    }

    /// Gets the value of NetworkAddresses
    pub fn get_network_addresses(&self) -> &Vec<String> {
        &self.network_addresses
    }

    /// Sets the value of NodeAddress
    pub fn set_node_address(&mut self, value: String) {
        self.node_address = Some(value);
    }

    /// Gets the value of NodeAddress
    pub fn get_node_address(&self) -> Option<&String> {
        self.node_address.as_ref()
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

    /// Sets the value of OtherLinkTechnology
    pub fn set_other_link_technology(&mut self, value: String) {
        self.other_link_technology = Some(value);
    }

    /// Gets the value of OtherLinkTechnology
    pub fn get_other_link_technology(&self) -> Option<&String> {
        self.other_link_technology.as_ref()
    }

    /// Sets the value of OtherOperationalStatusDescription
    pub fn set_other_operational_status_description(&mut self, value: String) {
        self.other_operational_status_description = Some(value);
    }

    /// Gets the value of OtherOperationalStatusDescription
    pub fn get_other_operational_status_description(&self) -> Option<&String> {
        self.other_operational_status_description.as_ref()
    }

    /// Sets the value of PortAddress
    pub fn set_port_address(&mut self, value: String) {
        self.port_address = Some(value);
    }

    /// Gets the value of PortAddress
    pub fn get_port_address(&self) -> Option<&String> {
        self.port_address.as_ref()
    }

    /// Sets the value of PortNumbers
    pub fn set_port_numbers(&mut self, value: Vec<u16>) {
        self.port_numbers = value;
    }

    /// Gets the value of PortNumbers
    pub fn get_port_numbers(&self) -> &Vec<u16> {
        &self.port_numbers
    }

    /// Sets the value of PortType
    pub fn set_port_type(&mut self, value: u16) {
        self.port_type = Some(value);
    }

    /// Gets the value of PortType
    pub fn get_port_type(&self) -> Option<&u16> {
        self.port_type.as_ref()
    }

    /// Sets the value of Role
    pub fn set_role(&mut self, value: u16) {
        self.role = Some(value);
    }

    /// Gets the value of Role
    pub fn get_role(&self) -> Option<&u16> {
        self.role.as_ref()
    }

    /// Sets the value of Speed
    pub fn set_speed(&mut self, value: u64) {
        self.speed = Some(value);
    }

    /// Gets the value of Speed
    pub fn get_speed(&self) -> Option<&u64> {
        self.speed.as_ref()
    }

    /// Sets the value of StorageControllerId
    pub fn set_storage_controller_id(&mut self, value: String) {
        self.storage_controller_id = Some(value);
    }

    /// Gets the value of StorageControllerId
    pub fn get_storage_controller_id(&self) -> Option<&String> {
        self.storage_controller_id.as_ref()
    }

    /// Sets the value of UsageRestriction
    pub fn set_usage_restriction(&mut self, value: u16) {
        self.usage_restriction = Some(value);
    }

    /// Gets the value of UsageRestriction
    pub fn get_usage_restriction(&self) -> Option<&u16> {
        self.usage_restriction.as_ref()
    }
}

