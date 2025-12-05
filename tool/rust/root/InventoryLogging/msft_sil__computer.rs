// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftSil_Computer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftSil_Computer {
    #[serde(flatten)]
    pub base: MsftSil_Data,

/// 
    #[serde(rename = "ChassisSerialNumber")]
    pub chassis_serial_number: Option<String>,

/// 
    #[serde(rename = "CollectedDateTime")]
    pub collected_date_time: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NumberOfCores")]
    pub number_of_cores: Option<u32>,

/// 
    #[serde(rename = "NumberOfLogicalProcessors")]
    pub number_of_logical_processors: Option<u32>,

/// 
    #[serde(rename = "NumberOfProcessors")]
    pub number_of_processors: Option<u32>,

/// 
    #[serde(rename = "OSName")]
    pub osname: Option<String>,

/// 
    #[serde(rename = "OSSku")]
    pub ossku: Option<u32>,

/// 
    #[serde(rename = "OSSuite")]
    pub ossuite: Option<u32>,

/// 
    #[serde(rename = "OSSuiteMask")]
    pub ossuite_mask: Option<u32>,

/// 
    #[serde(rename = "OSVersion")]
    pub osversion: Option<String>,

/// 
    #[serde(rename = "ProcessorFamily")]
    pub processor_family: Option<u32>,

/// 
    #[serde(rename = "ProcessorManufacturer")]
    pub processor_manufacturer: Option<String>,

/// 
    #[serde(rename = "ProcessorName")]
    pub processor_name: Option<String>,

/// 
    #[serde(rename = "SystemManufacturer")]
    pub system_manufacturer: Option<String>,
}

impl MsftSil_Computer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MsftSil_Data::new(),
            chassis_serial_number: None,
            collected_date_time: None,
            model: None,
            name: None,
            number_of_cores: None,
            number_of_logical_processors: None,
            number_of_processors: None,
            osname: None,
            ossku: None,
            ossuite: None,
            ossuite_mask: None,
            osversion: None,
            processor_family: None,
            processor_manufacturer: None,
            processor_name: None,
            system_manufacturer: None,
        }
    }


    /// Sets the value of ChassisSerialNumber
    pub fn set_chassis_serial_number(&mut self, value: String) {
        self.chassis_serial_number = Some(value);
    }

    /// Gets the value of ChassisSerialNumber
    pub fn get_chassis_serial_number(&self) -> Option<&String> {
        self.chassis_serial_number.as_ref()
    }

    /// Sets the value of CollectedDateTime
    pub fn set_collected_date_time(&mut self, value: String) {
        self.collected_date_time = Some(value);
    }

    /// Gets the value of CollectedDateTime
    pub fn get_collected_date_time(&self) -> Option<&String> {
        self.collected_date_time.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NumberOfCores
    pub fn set_number_of_cores(&mut self, value: u32) {
        self.number_of_cores = Some(value);
    }

    /// Gets the value of NumberOfCores
    pub fn get_number_of_cores(&self) -> Option<&u32> {
        self.number_of_cores.as_ref()
    }

    /// Sets the value of NumberOfLogicalProcessors
    pub fn set_number_of_logical_processors(&mut self, value: u32) {
        self.number_of_logical_processors = Some(value);
    }

    /// Gets the value of NumberOfLogicalProcessors
    pub fn get_number_of_logical_processors(&self) -> Option<&u32> {
        self.number_of_logical_processors.as_ref()
    }

    /// Sets the value of NumberOfProcessors
    pub fn set_number_of_processors(&mut self, value: u32) {
        self.number_of_processors = Some(value);
    }

    /// Gets the value of NumberOfProcessors
    pub fn get_number_of_processors(&self) -> Option<&u32> {
        self.number_of_processors.as_ref()
    }

    /// Sets the value of OSName
    pub fn set_osname(&mut self, value: String) {
        self.osname = Some(value);
    }

    /// Gets the value of OSName
    pub fn get_osname(&self) -> Option<&String> {
        self.osname.as_ref()
    }

    /// Sets the value of OSSku
    pub fn set_ossku(&mut self, value: u32) {
        self.ossku = Some(value);
    }

    /// Gets the value of OSSku
    pub fn get_ossku(&self) -> Option<&u32> {
        self.ossku.as_ref()
    }

    /// Sets the value of OSSuite
    pub fn set_ossuite(&mut self, value: u32) {
        self.ossuite = Some(value);
    }

    /// Gets the value of OSSuite
    pub fn get_ossuite(&self) -> Option<&u32> {
        self.ossuite.as_ref()
    }

    /// Sets the value of OSSuiteMask
    pub fn set_ossuite_mask(&mut self, value: u32) {
        self.ossuite_mask = Some(value);
    }

    /// Gets the value of OSSuiteMask
    pub fn get_ossuite_mask(&self) -> Option<&u32> {
        self.ossuite_mask.as_ref()
    }

    /// Sets the value of OSVersion
    pub fn set_osversion(&mut self, value: String) {
        self.osversion = Some(value);
    }

    /// Gets the value of OSVersion
    pub fn get_osversion(&self) -> Option<&String> {
        self.osversion.as_ref()
    }

    /// Sets the value of ProcessorFamily
    pub fn set_processor_family(&mut self, value: u32) {
        self.processor_family = Some(value);
    }

    /// Gets the value of ProcessorFamily
    pub fn get_processor_family(&self) -> Option<&u32> {
        self.processor_family.as_ref()
    }

    /// Sets the value of ProcessorManufacturer
    pub fn set_processor_manufacturer(&mut self, value: String) {
        self.processor_manufacturer = Some(value);
    }

    /// Gets the value of ProcessorManufacturer
    pub fn get_processor_manufacturer(&self) -> Option<&String> {
        self.processor_manufacturer.as_ref()
    }

    /// Sets the value of ProcessorName
    pub fn set_processor_name(&mut self, value: String) {
        self.processor_name = Some(value);
    }

    /// Gets the value of ProcessorName
    pub fn get_processor_name(&self) -> Option<&String> {
        self.processor_name.as_ref()
    }

    /// Sets the value of SystemManufacturer
    pub fn set_system_manufacturer(&mut self, value: String) {
        self.system_manufacturer = Some(value);
    }

    /// Gets the value of SystemManufacturer
    pub fn get_system_manufacturer(&self) -> Option<&String> {
        self.system_manufacturer.as_ref()
    }
}

