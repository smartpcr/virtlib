// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_AdapterInformationQuery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_AdapterInformationQuery {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// 
    #[serde(rename = "DriverVersion")]
    pub driver_version: Option<String>,

/// 
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// 
    #[serde(rename = "HardwareVersion")]
    pub hardware_version: Option<String>,

/// 
    #[serde(rename = "HBAStatus")]
    pub hbastatus: Option<u32>,

/// 
    #[serde(rename = "HBASymbolicName")]
    pub hbasymbolic_name: Option<String>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MfgDomain")]
    pub mfg_domain: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "ModelDescription")]
    pub model_description: Option<String>,

/// 
    #[serde(rename = "NumberOfPorts")]
    pub number_of_ports: Option<u32>,

/// 
    #[serde(rename = "OptionROMVersion")]
    pub option_romversion: Option<String>,

/// 
    #[serde(rename = "RedundantFirmwareVersion")]
    pub redundant_firmware_version: Option<String>,

/// 
    #[serde(rename = "RedundantOptionROMVersion")]
    pub redundant_option_romversion: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,

/// 
    #[serde(rename = "VendorSpecificID")]
    pub vendor_specific_id: Option<u32>,
}

impl MS_SM_AdapterInformationQuery {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            driver_name: None,
            driver_version: None,
            firmware_version: None,
            hardware_version: None,
            hbastatus: None,
            hbasymbolic_name: None,
            instance_name: None,
            manufacturer: None,
            mfg_domain: None,
            model: None,
            model_description: None,
            number_of_ports: None,
            option_romversion: None,
            redundant_firmware_version: None,
            redundant_option_romversion: None,
            serial_number: None,
            unique_adapter_id: None,
            vendor_specific_id: None,
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

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of DriverVersion
    pub fn set_driver_version(&mut self, value: String) {
        self.driver_version = Some(value);
    }

    /// Gets the value of DriverVersion
    pub fn get_driver_version(&self) -> Option<&String> {
        self.driver_version.as_ref()
    }

    /// Sets the value of FirmwareVersion
    pub fn set_firmware_version(&mut self, value: String) {
        self.firmware_version = Some(value);
    }

    /// Gets the value of FirmwareVersion
    pub fn get_firmware_version(&self) -> Option<&String> {
        self.firmware_version.as_ref()
    }

    /// Sets the value of HardwareVersion
    pub fn set_hardware_version(&mut self, value: String) {
        self.hardware_version = Some(value);
    }

    /// Gets the value of HardwareVersion
    pub fn get_hardware_version(&self) -> Option<&String> {
        self.hardware_version.as_ref()
    }

    /// Sets the value of HBAStatus
    pub fn set_hbastatus(&mut self, value: u32) {
        self.hbastatus = Some(value);
    }

    /// Gets the value of HBAStatus
    pub fn get_hbastatus(&self) -> Option<&u32> {
        self.hbastatus.as_ref()
    }

    /// Sets the value of HBASymbolicName
    pub fn set_hbasymbolic_name(&mut self, value: String) {
        self.hbasymbolic_name = Some(value);
    }

    /// Gets the value of HBASymbolicName
    pub fn get_hbasymbolic_name(&self) -> Option<&String> {
        self.hbasymbolic_name.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MfgDomain
    pub fn set_mfg_domain(&mut self, value: String) {
        self.mfg_domain = Some(value);
    }

    /// Gets the value of MfgDomain
    pub fn get_mfg_domain(&self) -> Option<&String> {
        self.mfg_domain.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of ModelDescription
    pub fn set_model_description(&mut self, value: String) {
        self.model_description = Some(value);
    }

    /// Gets the value of ModelDescription
    pub fn get_model_description(&self) -> Option<&String> {
        self.model_description.as_ref()
    }

    /// Sets the value of NumberOfPorts
    pub fn set_number_of_ports(&mut self, value: u32) {
        self.number_of_ports = Some(value);
    }

    /// Gets the value of NumberOfPorts
    pub fn get_number_of_ports(&self) -> Option<&u32> {
        self.number_of_ports.as_ref()
    }

    /// Sets the value of OptionROMVersion
    pub fn set_option_romversion(&mut self, value: String) {
        self.option_romversion = Some(value);
    }

    /// Gets the value of OptionROMVersion
    pub fn get_option_romversion(&self) -> Option<&String> {
        self.option_romversion.as_ref()
    }

    /// Sets the value of RedundantFirmwareVersion
    pub fn set_redundant_firmware_version(&mut self, value: String) {
        self.redundant_firmware_version = Some(value);
    }

    /// Gets the value of RedundantFirmwareVersion
    pub fn get_redundant_firmware_version(&self) -> Option<&String> {
        self.redundant_firmware_version.as_ref()
    }

    /// Sets the value of RedundantOptionROMVersion
    pub fn set_redundant_option_romversion(&mut self, value: String) {
        self.redundant_option_romversion = Some(value);
    }

    /// Gets the value of RedundantOptionROMVersion
    pub fn get_redundant_option_romversion(&self) -> Option<&String> {
        self.redundant_option_romversion.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }

    /// Sets the value of VendorSpecificID
    pub fn set_vendor_specific_id(&mut self, value: u32) {
        self.vendor_specific_id = Some(value);
    }

    /// Gets the value of VendorSpecificID
    pub fn get_vendor_specific_id(&self) -> Option<&u32> {
        self.vendor_specific_id.as_ref()
    }
}

