// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Processor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Processor {
    #[serde(flatten)]
    pub base: CIM_Processor,

/// 
    #[serde(rename = "Architecture")]
    pub architecture: Option<u16>,

/// 
    #[serde(rename = "AssetTag")]
    pub asset_tag: Option<String>,

/// 
    #[serde(rename = "Characteristics")]
    pub characteristics: Option<u32>,

/// 
    #[serde(rename = "CpuStatus")]
    pub cpu_status: Option<u16>,

/// 
    #[serde(rename = "CurrentVoltage")]
    pub current_voltage: Option<u16>,

/// 
    #[serde(rename = "ExtClock")]
    pub ext_clock: Option<u32>,

/// 
    #[serde(rename = "L2CacheSize")]
    pub l2_cache_size: Option<u32>,

/// 
    #[serde(rename = "L2CacheSpeed")]
    pub l2_cache_speed: Option<u32>,

/// 
    #[serde(rename = "L3CacheSize")]
    pub l3_cache_size: Option<u32>,

/// 
    #[serde(rename = "L3CacheSpeed")]
    pub l3_cache_speed: Option<u32>,

/// 
    #[serde(rename = "Level")]
    pub level: Option<u16>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "NumberOfCores")]
    pub number_of_cores: Option<u32>,

/// 
    #[serde(rename = "NumberOfEnabledCore")]
    pub number_of_enabled_core: Option<u32>,

/// 
    #[serde(rename = "NumberOfLogicalProcessors")]
    pub number_of_logical_processors: Option<u32>,

/// 
    #[serde(rename = "PartNumber")]
    pub part_number: Option<String>,

/// 
    #[serde(rename = "ProcessorId")]
    pub processor_id: Option<String>,

/// 
    #[serde(rename = "ProcessorType")]
    pub processor_type: Option<u16>,

/// 
    #[serde(rename = "Revision")]
    pub revision: Option<u16>,

/// 
    #[serde(rename = "SecondLevelAddressTranslationExtensions")]
    pub second_level_address_translation_extensions: Option<bool>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "SocketDesignation")]
    pub socket_designation: Option<String>,

/// 
    #[serde(rename = "ThreadCount")]
    pub thread_count: Option<u32>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VirtualizationFirmwareEnabled")]
    pub virtualization_firmware_enabled: Option<bool>,

/// 
    #[serde(rename = "VMMonitorModeExtensions")]
    pub vmmonitor_mode_extensions: Option<bool>,

/// 
    #[serde(rename = "VoltageCaps")]
    pub voltage_caps: Option<u32>,
}

impl Win32_Processor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Processor::new(),
            architecture: None,
            asset_tag: None,
            characteristics: None,
            cpu_status: None,
            current_voltage: None,
            ext_clock: None,
            l2_cache_size: None,
            l2_cache_speed: None,
            l3_cache_size: None,
            l3_cache_speed: None,
            level: None,
            manufacturer: None,
            number_of_cores: None,
            number_of_enabled_core: None,
            number_of_logical_processors: None,
            part_number: None,
            processor_id: None,
            processor_type: None,
            revision: None,
            second_level_address_translation_extensions: None,
            serial_number: None,
            socket_designation: None,
            thread_count: None,
            version: None,
            virtualization_firmware_enabled: None,
            vmmonitor_mode_extensions: None,
            voltage_caps: None,
        }
    }


    /// Sets the value of Architecture
    pub fn set_architecture(&mut self, value: u16) {
        self.architecture = Some(value);
    }

    /// Gets the value of Architecture
    pub fn get_architecture(&self) -> Option<&u16> {
        self.architecture.as_ref()
    }

    /// Sets the value of AssetTag
    pub fn set_asset_tag(&mut self, value: String) {
        self.asset_tag = Some(value);
    }

    /// Gets the value of AssetTag
    pub fn get_asset_tag(&self) -> Option<&String> {
        self.asset_tag.as_ref()
    }

    /// Sets the value of Characteristics
    pub fn set_characteristics(&mut self, value: u32) {
        self.characteristics = Some(value);
    }

    /// Gets the value of Characteristics
    pub fn get_characteristics(&self) -> Option<&u32> {
        self.characteristics.as_ref()
    }

    /// Sets the value of CpuStatus
    pub fn set_cpu_status(&mut self, value: u16) {
        self.cpu_status = Some(value);
    }

    /// Gets the value of CpuStatus
    pub fn get_cpu_status(&self) -> Option<&u16> {
        self.cpu_status.as_ref()
    }

    /// Sets the value of CurrentVoltage
    pub fn set_current_voltage(&mut self, value: u16) {
        self.current_voltage = Some(value);
    }

    /// Gets the value of CurrentVoltage
    pub fn get_current_voltage(&self) -> Option<&u16> {
        self.current_voltage.as_ref()
    }

    /// Sets the value of ExtClock
    pub fn set_ext_clock(&mut self, value: u32) {
        self.ext_clock = Some(value);
    }

    /// Gets the value of ExtClock
    pub fn get_ext_clock(&self) -> Option<&u32> {
        self.ext_clock.as_ref()
    }

    /// Sets the value of L2CacheSize
    pub fn set_l2_cache_size(&mut self, value: u32) {
        self.l2_cache_size = Some(value);
    }

    /// Gets the value of L2CacheSize
    pub fn get_l2_cache_size(&self) -> Option<&u32> {
        self.l2_cache_size.as_ref()
    }

    /// Sets the value of L2CacheSpeed
    pub fn set_l2_cache_speed(&mut self, value: u32) {
        self.l2_cache_speed = Some(value);
    }

    /// Gets the value of L2CacheSpeed
    pub fn get_l2_cache_speed(&self) -> Option<&u32> {
        self.l2_cache_speed.as_ref()
    }

    /// Sets the value of L3CacheSize
    pub fn set_l3_cache_size(&mut self, value: u32) {
        self.l3_cache_size = Some(value);
    }

    /// Gets the value of L3CacheSize
    pub fn get_l3_cache_size(&self) -> Option<&u32> {
        self.l3_cache_size.as_ref()
    }

    /// Sets the value of L3CacheSpeed
    pub fn set_l3_cache_speed(&mut self, value: u32) {
        self.l3_cache_speed = Some(value);
    }

    /// Gets the value of L3CacheSpeed
    pub fn get_l3_cache_speed(&self) -> Option<&u32> {
        self.l3_cache_speed.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: u16) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&u16> {
        self.level.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of NumberOfCores
    pub fn set_number_of_cores(&mut self, value: u32) {
        self.number_of_cores = Some(value);
    }

    /// Gets the value of NumberOfCores
    pub fn get_number_of_cores(&self) -> Option<&u32> {
        self.number_of_cores.as_ref()
    }

    /// Sets the value of NumberOfEnabledCore
    pub fn set_number_of_enabled_core(&mut self, value: u32) {
        self.number_of_enabled_core = Some(value);
    }

    /// Gets the value of NumberOfEnabledCore
    pub fn get_number_of_enabled_core(&self) -> Option<&u32> {
        self.number_of_enabled_core.as_ref()
    }

    /// Sets the value of NumberOfLogicalProcessors
    pub fn set_number_of_logical_processors(&mut self, value: u32) {
        self.number_of_logical_processors = Some(value);
    }

    /// Gets the value of NumberOfLogicalProcessors
    pub fn get_number_of_logical_processors(&self) -> Option<&u32> {
        self.number_of_logical_processors.as_ref()
    }

    /// Sets the value of PartNumber
    pub fn set_part_number(&mut self, value: String) {
        self.part_number = Some(value);
    }

    /// Gets the value of PartNumber
    pub fn get_part_number(&self) -> Option<&String> {
        self.part_number.as_ref()
    }

    /// Sets the value of ProcessorId
    pub fn set_processor_id(&mut self, value: String) {
        self.processor_id = Some(value);
    }

    /// Gets the value of ProcessorId
    pub fn get_processor_id(&self) -> Option<&String> {
        self.processor_id.as_ref()
    }

    /// Sets the value of ProcessorType
    pub fn set_processor_type(&mut self, value: u16) {
        self.processor_type = Some(value);
    }

    /// Gets the value of ProcessorType
    pub fn get_processor_type(&self) -> Option<&u16> {
        self.processor_type.as_ref()
    }

    /// Sets the value of Revision
    pub fn set_revision(&mut self, value: u16) {
        self.revision = Some(value);
    }

    /// Gets the value of Revision
    pub fn get_revision(&self) -> Option<&u16> {
        self.revision.as_ref()
    }

    /// Sets the value of SecondLevelAddressTranslationExtensions
    pub fn set_second_level_address_translation_extensions(&mut self, value: bool) {
        self.second_level_address_translation_extensions = Some(value);
    }

    /// Gets the value of SecondLevelAddressTranslationExtensions
    pub fn get_second_level_address_translation_extensions(&self) -> Option<&bool> {
        self.second_level_address_translation_extensions.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of SocketDesignation
    pub fn set_socket_designation(&mut self, value: String) {
        self.socket_designation = Some(value);
    }

    /// Gets the value of SocketDesignation
    pub fn get_socket_designation(&self) -> Option<&String> {
        self.socket_designation.as_ref()
    }

    /// Sets the value of ThreadCount
    pub fn set_thread_count(&mut self, value: u32) {
        self.thread_count = Some(value);
    }

    /// Gets the value of ThreadCount
    pub fn get_thread_count(&self) -> Option<&u32> {
        self.thread_count.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of VirtualizationFirmwareEnabled
    pub fn set_virtualization_firmware_enabled(&mut self, value: bool) {
        self.virtualization_firmware_enabled = Some(value);
    }

    /// Gets the value of VirtualizationFirmwareEnabled
    pub fn get_virtualization_firmware_enabled(&self) -> Option<&bool> {
        self.virtualization_firmware_enabled.as_ref()
    }

    /// Sets the value of VMMonitorModeExtensions
    pub fn set_vmmonitor_mode_extensions(&mut self, value: bool) {
        self.vmmonitor_mode_extensions = Some(value);
    }

    /// Gets the value of VMMonitorModeExtensions
    pub fn get_vmmonitor_mode_extensions(&self) -> Option<&bool> {
        self.vmmonitor_mode_extensions.as_ref()
    }

    /// Sets the value of VoltageCaps
    pub fn set_voltage_caps(&mut self, value: u32) {
        self.voltage_caps = Some(value);
    }

    /// Gets the value of VoltageCaps
    pub fn get_voltage_caps(&self) -> Option<&u32> {
        self.voltage_caps.as_ref()
    }
}

