// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageEnclosure struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageEnclosure {
    #[serde(flatten)]
    pub base: MSFT_StorageFaultDomain,

/// 
    #[serde(rename = "BusType")]
    pub bus_type: Option<u16>,

/// 
    #[serde(rename = "CurrentSensorOperationalStatus")]
    pub current_sensor_operational_status: Vec<u16>,

/// 
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "FanOperationalStatus")]
    pub fan_operational_status: Vec<u16>,

/// 
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// 
    #[serde(rename = "IOControllerOperationalStatus")]
    pub iocontroller_operational_status: Vec<u16>,

/// 
    #[serde(rename = "NumberOfSlots")]
    pub number_of_slots: Option<u32>,

/// 
    #[serde(rename = "PowerSupplyOperationalStatus")]
    pub power_supply_operational_status: Vec<u16>,

/// 
    #[serde(rename = "SlotOperationalStatus")]
    pub slot_operational_status: Vec<u16>,

/// 
    #[serde(rename = "TemperatureSensorOperationalStatus")]
    pub temperature_sensor_operational_status: Vec<u16>,

/// 
    #[serde(rename = "VoltageSensorOperationalStatus")]
    pub voltage_sensor_operational_status: Vec<u16>,
}

impl MSFT_StorageEnclosure {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageFaultDomain::new(),
            bus_type: None,
            current_sensor_operational_status: Vec::new(),
            device_id: None,
            fan_operational_status: Vec::new(),
            firmware_version: None,
            iocontroller_operational_status: Vec::new(),
            number_of_slots: None,
            power_supply_operational_status: Vec::new(),
            slot_operational_status: Vec::new(),
            temperature_sensor_operational_status: Vec::new(),
            voltage_sensor_operational_status: Vec::new(),
        }
    }


    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: u16) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&u16> {
        self.bus_type.as_ref()
    }

    /// Sets the value of CurrentSensorOperationalStatus
    pub fn set_current_sensor_operational_status(&mut self, value: Vec<u16>) {
        self.current_sensor_operational_status = value;
    }

    /// Gets the value of CurrentSensorOperationalStatus
    pub fn get_current_sensor_operational_status(&self) -> &Vec<u16> {
        &self.current_sensor_operational_status
    }

    /// Sets the value of DeviceId
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceId
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of FanOperationalStatus
    pub fn set_fan_operational_status(&mut self, value: Vec<u16>) {
        self.fan_operational_status = value;
    }

    /// Gets the value of FanOperationalStatus
    pub fn get_fan_operational_status(&self) -> &Vec<u16> {
        &self.fan_operational_status
    }

    /// Sets the value of FirmwareVersion
    pub fn set_firmware_version(&mut self, value: String) {
        self.firmware_version = Some(value);
    }

    /// Gets the value of FirmwareVersion
    pub fn get_firmware_version(&self) -> Option<&String> {
        self.firmware_version.as_ref()
    }

    /// Sets the value of IOControllerOperationalStatus
    pub fn set_iocontroller_operational_status(&mut self, value: Vec<u16>) {
        self.iocontroller_operational_status = value;
    }

    /// Gets the value of IOControllerOperationalStatus
    pub fn get_iocontroller_operational_status(&self) -> &Vec<u16> {
        &self.iocontroller_operational_status
    }

    /// Sets the value of NumberOfSlots
    pub fn set_number_of_slots(&mut self, value: u32) {
        self.number_of_slots = Some(value);
    }

    /// Gets the value of NumberOfSlots
    pub fn get_number_of_slots(&self) -> Option<&u32> {
        self.number_of_slots.as_ref()
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

/// 

    /// * `enable` -  (bool)
    /// * `slot_numbers` -  (u32[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn identify_element(&self, enable: bool, slot_numbers: &Vec<u32>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Enable".to_string(), value: enable.into() });
        args.push(MethodParameter { name: "SlotNumbers".to_string(), value: slot_numbers.into() });

        let result = self.invoke_method("IdentifyElement", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `page_number` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `vendor_data` -  (String)
    pub fn get_vendor_data(&self, page_number: u16, vendor_data: &mut String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PageNumber".to_string(), value: page_number.into() });

        let result = self.invoke_method("GetVendorData", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let vendor_data = result.get_value("VendorData")?;
        Ok(result.return_value)

    }


/// 

    /// * `enable_maintenance_mode` -  (bool)
    /// * `ignore_detached_virtual_disks` -  (bool)
    /// * `manufacturer` -  (String)
    /// * `model` -  (String)
    /// * `timeout` -  (u32)
    /// * `validate_maintenance_mode` -  (bool)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn maintenance(&self, validate_maintenance_mode: bool, enable_maintenance_mode: bool, timeout: u32, model: &String, manufacturer: &String, ignore_detached_virtual_disks: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ValidateMaintenanceMode".to_string(), value: validate_maintenance_mode.into() });
        args.push(MethodParameter { name: "EnableMaintenanceMode".to_string(), value: enable_maintenance_mode.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        args.push(MethodParameter { name: "Model".to_string(), value: model.into() });
        args.push(MethodParameter { name: "Manufacturer".to_string(), value: manufacturer.into() });
        args.push(MethodParameter { name: "IgnoreDetachedVirtualDisks".to_string(), value: ignore_detached_virtual_disks.into() });

        let result = self.invoke_method("Maintenance", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `off` -  (bool)
    /// * `slot_numbers` -  (u32[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn power_element(&self, off: bool, slot_numbers: &Vec<u32>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Off".to_string(), value: off.into() });
        args.push(MethodParameter { name: "SlotNumbers".to_string(), value: slot_numbers.into() });

        let result = self.invoke_method("PowerElement", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `active_slot_number` -  (u16)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `firmware_version_in_slot` -  (String[])
    /// * `is_slot_writable` -  (bool[])
    /// * `number_of_slots` -  (u16)
    /// * `return_value` -  (u32)
    /// * `slot_number` -  (u16[])
    /// * `supports_update` -  (bool)
    pub fn get_firmware_information(&self, supports_update: &mut bool, number_of_slots: &mut u16, active_slot_number: &mut u16, slot_number: &mut Vec<u16>, is_slot_writable: &mut Vec<bool>, firmware_version_in_slot: &mut Vec<String>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetFirmwareInformation", &[])?;
        let active_slot_number = result.get_value("ActiveSlotNumber")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let firmware_version_in_slot = result.get_value("FirmwareVersionInSlot")?;
        let is_slot_writable = result.get_value("IsSlotWritable")?;
        let number_of_slots = result.get_value("NumberOfSlots")?;
        let slot_number = result.get_value("SlotNumber")?;
        let supports_update = result.get_value("SupportsUpdate")?;
        Ok(result.return_value)

    }


/// 

    /// * `image_path` -  (String)
    /// * `slot_number` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn update_firmware(&self, image_path: &String, slot_number: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ImagePath".to_string(), value: image_path.into() });
        args.push(MethodParameter { name: "SlotNumber".to_string(), value: slot_number.into() });

        let result = self.invoke_method("UpdateFirmware", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

