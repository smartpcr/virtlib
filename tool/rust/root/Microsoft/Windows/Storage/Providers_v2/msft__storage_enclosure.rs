// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
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
    pub bus_type: Option<StorageEnclosure_BusType>,

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
    pub current_sensor_operational_status: Vec<StorageEnclosure_CurrentSensorOperationalStatus>,

/// DeviceId is an address or other identifier that uniquely names the enclosure. For example, DeviceId is the enclosure GUID in Storage Spaces provider.
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,

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
    pub fan_operational_status: Vec<StorageEnclosure_FanOperationalStatus>,

/// This field is a string representation of the enclosure's firmware version.
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

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
    pub iocontroller_operational_status: Vec<StorageEnclosure_IOControllerOperationalStatus>,

/// Number of slots hosted within the enclosure
    #[serde(rename = "NumberOfSlots")]
    pub number_of_slots: Option<u32>,

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
    pub power_supply_operational_status: Vec<StorageEnclosure_PowerSupplyOperationalStatus>,

/// 
    #[serde(rename = "SlotOperationalStatus")]
    pub slot_operational_status: Vec<u16>,

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
    pub temperature_sensor_operational_status: Vec<StorageEnclosure_TemperatureSensorOperationalStatus>,

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
    pub voltage_sensor_operational_status: Vec<StorageEnclosure_VoltageSensorOperationalStatus>,
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
    pub fn set_bus_type(&mut self, value: StorageEnclosure_BusType) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&StorageEnclosure_BusType> {
        self.bus_type.as_ref()
    }

    /// Sets the value of CurrentSensorOperationalStatus
    pub fn set_current_sensor_operational_status(&mut self, value: Vec<StorageEnclosure_CurrentSensorOperationalStatus>) {
        self.current_sensor_operational_status = value;
    }

    /// Gets the value of CurrentSensorOperationalStatus
    pub fn get_current_sensor_operational_status(&self) -> &Vec<StorageEnclosure_CurrentSensorOperationalStatus> {
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
    pub fn set_fan_operational_status(&mut self, value: Vec<StorageEnclosure_FanOperationalStatus>) {
        self.fan_operational_status = value;
    }

    /// Gets the value of FanOperationalStatus
    pub fn get_fan_operational_status(&self) -> &Vec<StorageEnclosure_FanOperationalStatus> {
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
    pub fn set_iocontroller_operational_status(&mut self, value: Vec<StorageEnclosure_IOControllerOperationalStatus>) {
        self.iocontroller_operational_status = value;
    }

    /// Gets the value of IOControllerOperationalStatus
    pub fn get_iocontroller_operational_status(&self) -> &Vec<StorageEnclosure_IOControllerOperationalStatus> {
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
    pub fn set_power_supply_operational_status(&mut self, value: Vec<StorageEnclosure_PowerSupplyOperationalStatus>) {
        self.power_supply_operational_status = value;
    }

    /// Gets the value of PowerSupplyOperationalStatus
    pub fn get_power_supply_operational_status(&self) -> &Vec<StorageEnclosure_PowerSupplyOperationalStatus> {
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
    pub fn set_temperature_sensor_operational_status(&mut self, value: Vec<StorageEnclosure_TemperatureSensorOperationalStatus>) {
        self.temperature_sensor_operational_status = value;
    }

    /// Gets the value of TemperatureSensorOperationalStatus
    pub fn get_temperature_sensor_operational_status(&self) -> &Vec<StorageEnclosure_TemperatureSensorOperationalStatus> {
        &self.temperature_sensor_operational_status
    }

    /// Sets the value of VoltageSensorOperationalStatus
    pub fn set_voltage_sensor_operational_status(&mut self, value: Vec<StorageEnclosure_VoltageSensorOperationalStatus>) {
        self.voltage_sensor_operational_status = value;
    }

    /// Gets the value of VoltageSensorOperationalStatus
    pub fn get_voltage_sensor_operational_status(&self) -> &Vec<StorageEnclosure_VoltageSensorOperationalStatus> {
        &self.voltage_sensor_operational_status
    }

/// This method allows a user to perform certain identification tasks on the enclosure and its elements.

    /// * `enable` - If set to TRUE, this instructs the enclosure to enable its identification LED on the specified element. The identification LED should remain enabled until a second call to IdentifyElement on the same element is made with this parameter specified as FALSE. (bool)
    /// * `slot_numbers` - The numbers of the slots on which to enable or disable identification. (u32[])

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn identify_element(&self, enable: bool, slot_numbers: &Vec<u32>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Enable".to_string(), value: enable.into() });
        args.push(MethodParameter { name: "SlotNumbers".to_string(), value: slot_numbers.into() });

        let result = self.invoke_method("IdentifyElement", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method returns the vendor specific data from an enclosure.

    /// * `page_number` - Denotes the page number for which vendor data is requested. (u16)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `vendor_data` - The vendor specific data (page 04h for example) from an enclosure. (String)
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

