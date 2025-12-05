// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNodeToStorageEnclosure struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNodeToStorageEnclosure {

/// 
    #[serde(rename = "CurrentSensorOperationalStatus")]
    pub current_sensor_operational_status: Vec<u16>,

/// 
    #[serde(rename = "EnclosureNumber")]
    pub enclosure_number: Option<u32>,

/// 
    #[serde(rename = "FanOperationalStatus")]
    pub fan_operational_status: Vec<u16>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "IOControllerOperationalStatus")]
    pub iocontroller_operational_status: Vec<u16>,

/// 
    #[serde(rename = "IsPhysicallyConnected")]
    pub is_physically_connected: Option<bool>,

/// 
    #[serde(rename = "PowerSupplyOperationalStatus")]
    pub power_supply_operational_status: Vec<u16>,

/// 
    #[serde(rename = "SlotOperationalStatus")]
    pub slot_operational_status: Vec<u16>,

/// 
    #[serde(rename = "StorageEnclosure")]
    pub storage_enclosure: Option<MSFT_StorageEnclosure>,

/// 
    #[serde(rename = "StorageNode")]
    pub storage_node: Option<MSFT_StorageNode>,

/// 
    #[serde(rename = "TemperatureSensorOperationalStatus")]
    pub temperature_sensor_operational_status: Vec<u16>,

/// 
    #[serde(rename = "VoltageSensorOperationalStatus")]
    pub voltage_sensor_operational_status: Vec<u16>,
}

impl MSFT_StorageNodeToStorageEnclosure {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            current_sensor_operational_status: Vec::new(),
            enclosure_number: None,
            fan_operational_status: Vec::new(),
            health_status: None,
            iocontroller_operational_status: Vec::new(),
            is_physically_connected: None,
            power_supply_operational_status: Vec::new(),
            slot_operational_status: Vec::new(),
            storage_enclosure: None,
            storage_node: None,
            temperature_sensor_operational_status: Vec::new(),
            voltage_sensor_operational_status: Vec::new(),
        }
    }


    /// Sets the value of CurrentSensorOperationalStatus
    pub fn set_current_sensor_operational_status(&mut self, value: Vec<u16>) {
        self.current_sensor_operational_status = value;
    }

    /// Gets the value of CurrentSensorOperationalStatus
    pub fn get_current_sensor_operational_status(&self) -> &Vec<u16> {
        &self.current_sensor_operational_status
    }

    /// Sets the value of EnclosureNumber
    pub fn set_enclosure_number(&mut self, value: u32) {
        self.enclosure_number = Some(value);
    }

    /// Gets the value of EnclosureNumber
    pub fn get_enclosure_number(&self) -> Option<&u32> {
        self.enclosure_number.as_ref()
    }

    /// Sets the value of FanOperationalStatus
    pub fn set_fan_operational_status(&mut self, value: Vec<u16>) {
        self.fan_operational_status = value;
    }

    /// Gets the value of FanOperationalStatus
    pub fn get_fan_operational_status(&self) -> &Vec<u16> {
        &self.fan_operational_status
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of IOControllerOperationalStatus
    pub fn set_iocontroller_operational_status(&mut self, value: Vec<u16>) {
        self.iocontroller_operational_status = value;
    }

    /// Gets the value of IOControllerOperationalStatus
    pub fn get_iocontroller_operational_status(&self) -> &Vec<u16> {
        &self.iocontroller_operational_status
    }

    /// Sets the value of IsPhysicallyConnected
    pub fn set_is_physically_connected(&mut self, value: bool) {
        self.is_physically_connected = Some(value);
    }

    /// Gets the value of IsPhysicallyConnected
    pub fn get_is_physically_connected(&self) -> Option<&bool> {
        self.is_physically_connected.as_ref()
    }

    /// Sets the value of PowerSupplyOperationalStatus
    pub fn set_power_supply_operational_status(&mut self, value: Vec<u16>) {
        self.power_supply_operational_status = value;
    }

    /// Gets the value of PowerSupplyOperationalStatus
    pub fn get_power_supply_operational_status(&self) -> &Vec<u16> {
        &self.power_supply_operational_status
    }

    /// Sets the value of SlotOperationalStatus
    pub fn set_slot_operational_status(&mut self, value: Vec<u16>) {
        self.slot_operational_status = value;
    }

    /// Gets the value of SlotOperationalStatus
    pub fn get_slot_operational_status(&self) -> &Vec<u16> {
        &self.slot_operational_status
    }

    /// Sets the value of StorageEnclosure
    pub fn set_storage_enclosure(&mut self, value: MSFT_StorageEnclosure) {
        self.storage_enclosure = Some(value);
    }

    /// Gets the value of StorageEnclosure
    pub fn get_storage_enclosure(&self) -> Option<&MSFT_StorageEnclosure> {
        self.storage_enclosure.as_ref()
    }

    /// Sets the value of StorageNode
    pub fn set_storage_node(&mut self, value: MSFT_StorageNode) {
        self.storage_node = Some(value);
    }

    /// Gets the value of StorageNode
    pub fn get_storage_node(&self) -> Option<&MSFT_StorageNode> {
        self.storage_node.as_ref()
    }

    /// Sets the value of TemperatureSensorOperationalStatus
    pub fn set_temperature_sensor_operational_status(&mut self, value: Vec<u16>) {
        self.temperature_sensor_operational_status = value;
    }

    /// Gets the value of TemperatureSensorOperationalStatus
    pub fn get_temperature_sensor_operational_status(&self) -> &Vec<u16> {
        &self.temperature_sensor_operational_status
    }

    /// Sets the value of VoltageSensorOperationalStatus
    pub fn set_voltage_sensor_operational_status(&mut self, value: Vec<u16>) {
        self.voltage_sensor_operational_status = value;
    }

    /// Gets the value of VoltageSensorOperationalStatus
    pub fn get_voltage_sensor_operational_status(&self) -> &Vec<u16> {
        &self.voltage_sensor_operational_status
    }
}

