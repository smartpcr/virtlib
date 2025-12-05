// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNodeToStorageEnclosure struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNodeToStorageEnclosure {

/// An array containing the operational status of each current sensor of the enclosure. 
/// 0 - 'Unknown' 
/// 2 - 'OK': The element is present and working with no issues detected. 
/// 3 - 'Degraded': The element detected one or more non-critical issues. 
/// 6 - 'Error': The element detected one or more critical issues. 
/// 7 - 'Non-Recoverable Error': The element detected one or more non-recoverable issues. 
/// 0xD009 - 'Not Installed': The element is not present. 
/// 0xD00A - 'Not Available': The element is present but has problems. 
/// 0xD00B - 'No Access Allowed': No access is allowed to the element. 
/// 0xD00C - 'Not Reported' 
    #[serde(rename = "CurrentSensorOperationalStatus")]
    pub current_sensor_operational_status: Vec<StorageNodeToStorageEnclosure_CurrentSensorOperationalStatus>,

/// The device number for the enclosure on this storage node.
    #[serde(rename = "EnclosureNumber")]
    pub enclosure_number: Option<u32>,

/// An array containing the operational status of each fan of the enclosure. 
/// 0 - 'Unknown' 
/// 2 - 'OK': The element is present and working with no issues detected. 
/// 3 - 'Degraded': The element detected one or more non-critical issues. 
/// 6 - 'Error': The element detected one or more critical issues. 
/// 7 - 'Non-Recoverable Error': The element detected one or more non-recoverable issues. 
/// 0xD009 - 'Not Installed': The element is not present. 
/// 0xD00A - 'Not Available': The element is present but has problems. 
/// 0xD00B - 'No Access Allowed': No access is allowed to the element. 
/// 0xD00C - 'Not Reported' 
    #[serde(rename = "FanOperationalStatus")]
    pub fan_operational_status: Vec<StorageNodeToStorageEnclosure_FanOperationalStatus>,

/// Denotes the current health status of the enclosure.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<StorageNodeToStorageEnclosure_HealthStatus>,

/// An array containing the operational status of each controller of the enclosure. 
/// 0 - 'Unknown' 
/// 2 - 'OK': The element is present and working with no issues detected. 
/// 3 - 'Degraded': The element detected one or more non-critical issues. 
/// 6 - 'Error': The element detected one or more critical issues. 
/// 7 - 'Non-Recoverable Error': The element detected one or more non-recoverable issues. 
/// 0xD009 - 'Not Installed': The element is not present. 
/// 0xD00A - 'Not Available': The element is present but has problems. 
/// 0xD00B - 'No Access Allowed': No access is allowed to the element. 
/// 0xD00C - 'Not Reported' 
    #[serde(rename = "IOControllerOperationalStatus")]
    pub iocontroller_operational_status: Vec<StorageNodeToStorageEnclosure_IOControllerOperationalStatus>,

/// Indicates whether the storage enclosure is physically connected to this storage node.
    #[serde(rename = "IsPhysicallyConnected")]
    pub is_physically_connected: Option<bool>,

/// An array containing the operational status of each power supply of the enclosure. 
/// 0 - 'Unknown' 
/// 2 - 'OK': The element is present and working with no issues detected. 
/// 3 - 'Degraded': The element detected one or more non-critical issues. 
/// 6 - 'Error': The element detected one or more critical issues. 
/// 7 - 'Non-Recoverable Error': The element detected one or more non-recoverable issues. 
/// 0xD009 - 'Not Installed': The element is not present. 
/// 0xD00A - 'Not Available': The element is present but has problems. 
/// 0xD00B - 'No Access Allowed': No access is allowed to the element. 
/// 0xD00C - 'Not Reported' 
    #[serde(rename = "PowerSupplyOperationalStatus")]
    pub power_supply_operational_status: Vec<StorageNodeToStorageEnclosure_PowerSupplyOperationalStatus>,

/// 
    #[serde(rename = "SlotOperationalStatus")]
    pub slot_operational_status: Vec<u16>,

/// 
    #[serde(rename = "StorageEnclosure")]
    pub storage_enclosure: Option<MSFT_StorageEnclosure>,

/// 
    #[serde(rename = "StorageNode")]
    pub storage_node: Option<MSFT_StorageNode>,

/// An array containing the operational status of each temperature sensor of the enclosure. 
/// 0 - 'Unknown' 
/// 2 - 'OK': The element is present and working with no issues detected. 
/// 3 - 'Degraded': The element detected one or more non-critical issues. 
/// 6 - 'Error': The element detected one or more critical issues. 
/// 7 - 'Non-Recoverable Error': The element detected one or more non-recoverable issues. 
/// 0xD009 - 'Not Installed': The element is not present. 
/// 0xD00A - 'Not Available': The element is present but has problems. 
/// 0xD00B - 'No Access Allowed': No access is allowed to the element. 
/// 0xD00C - 'Not Reported' 
    #[serde(rename = "TemperatureSensorOperationalStatus")]
    pub temperature_sensor_operational_status: Vec<StorageNodeToStorageEnclosure_TemperatureSensorOperationalStatus>,

/// An array containing the operational status of each voltage sensor of the enclosure. 
/// 0 - 'Unknown' 
/// 2 - 'OK': The element is present and working with no issues detected. 
/// 3 - 'Degraded': The element detected one or more non-critical issues. 
/// 6 - 'Error': The element detected one or more critical issues. 
/// 7 - 'Non-Recoverable Error': The element detected one or more non-recoverable issues. 
/// 0xD009 - 'Not Installed': The element is not present. 
/// 0xD00A - 'Not Available': The element is present but has problems. 
/// 0xD00B - 'No Access Allowed': No access is allowed to the element. 
/// 0xD00C - 'Not Reported' 
    #[serde(rename = "VoltageSensorOperationalStatus")]
    pub voltage_sensor_operational_status: Vec<StorageNodeToStorageEnclosure_VoltageSensorOperationalStatus>,
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
    pub fn set_current_sensor_operational_status(&mut self, value: Vec<StorageNodeToStorageEnclosure_CurrentSensorOperationalStatus>) {
        self.current_sensor_operational_status = value;
    }

    /// Gets the value of CurrentSensorOperationalStatus
    pub fn get_current_sensor_operational_status(&self) -> &Vec<StorageNodeToStorageEnclosure_CurrentSensorOperationalStatus> {
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
    pub fn set_fan_operational_status(&mut self, value: Vec<StorageNodeToStorageEnclosure_FanOperationalStatus>) {
        self.fan_operational_status = value;
    }

    /// Gets the value of FanOperationalStatus
    pub fn get_fan_operational_status(&self) -> &Vec<StorageNodeToStorageEnclosure_FanOperationalStatus> {
        &self.fan_operational_status
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: StorageNodeToStorageEnclosure_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&StorageNodeToStorageEnclosure_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of IOControllerOperationalStatus
    pub fn set_iocontroller_operational_status(&mut self, value: Vec<StorageNodeToStorageEnclosure_IOControllerOperationalStatus>) {
        self.iocontroller_operational_status = value;
    }

    /// Gets the value of IOControllerOperationalStatus
    pub fn get_iocontroller_operational_status(&self) -> &Vec<StorageNodeToStorageEnclosure_IOControllerOperationalStatus> {
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
    pub fn set_power_supply_operational_status(&mut self, value: Vec<StorageNodeToStorageEnclosure_PowerSupplyOperationalStatus>) {
        self.power_supply_operational_status = value;
    }

    /// Gets the value of PowerSupplyOperationalStatus
    pub fn get_power_supply_operational_status(&self) -> &Vec<StorageNodeToStorageEnclosure_PowerSupplyOperationalStatus> {
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
    pub fn set_temperature_sensor_operational_status(&mut self, value: Vec<StorageNodeToStorageEnclosure_TemperatureSensorOperationalStatus>) {
        self.temperature_sensor_operational_status = value;
    }

    /// Gets the value of TemperatureSensorOperationalStatus
    pub fn get_temperature_sensor_operational_status(&self) -> &Vec<StorageNodeToStorageEnclosure_TemperatureSensorOperationalStatus> {
        &self.temperature_sensor_operational_status
    }

    /// Sets the value of VoltageSensorOperationalStatus
    pub fn set_voltage_sensor_operational_status(&mut self, value: Vec<StorageNodeToStorageEnclosure_VoltageSensorOperationalStatus>) {
        self.voltage_sensor_operational_status = value;
    }

    /// Gets the value of VoltageSensorOperationalStatus
    pub fn get_voltage_sensor_operational_status(&self) -> &Vec<StorageNodeToStorageEnclosure_VoltageSensorOperationalStatus> {
        &self.voltage_sensor_operational_status
    }
}

