// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_PhysicalComputerSystemView struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_PhysicalComputerSystemView {
    #[serde(flatten)]
    pub base: CIM_View,

/// 
    #[serde(rename = "CurrentBIOSBuildNumber")]
    pub current_biosbuild_number: Option<u16>,

/// 
    #[serde(rename = "CurrentBIOSMajorVersion")]
    pub current_biosmajor_version: Option<u16>,

/// 
    #[serde(rename = "CurrentBIOSMinorVersion")]
    pub current_biosminor_version: Option<u16>,

/// 
    #[serde(rename = "CurrentBIOSRevisionNumber")]
    pub current_biosrevision_number: Option<u16>,

/// 
    #[serde(rename = "CurrentBIOSVersionString")]
    pub current_biosversion_string: Option<String>,

/// 
    #[serde(rename = "CurrentManagementFirmwareBuildNumber")]
    pub current_management_firmware_build_number: Option<u16>,

/// 
    #[serde(rename = "CurrentManagementFirmwareElementName")]
    pub current_management_firmware_element_name: Option<String>,

/// 
    #[serde(rename = "CurrentManagementFirmwareMajorVersion")]
    pub current_management_firmware_major_version: Option<u16>,

/// 
    #[serde(rename = "CurrentManagementFirmwareMinorVersion")]
    pub current_management_firmware_minor_version: Option<u16>,

/// 
    #[serde(rename = "CurrentManagementFirmwareRevisionNumber")]
    pub current_management_firmware_revision_number: Option<u16>,

/// 
    #[serde(rename = "CurrentManagementFirmwareVersionString")]
    pub current_management_firmware_version_string: Option<String>,

/// 
    #[serde(rename = "Dedicated")]
    pub dedicated: Vec<u16>,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<u16>,

/// 
    #[serde(rename = "FRUInfoSupported")]
    pub fruinfo_supported: Option<bool>,

/// 
    #[serde(rename = "HealthState")]
    pub health_state: Option<u16>,

/// 
    #[serde(rename = "IdentifyingDescriptions")]
    pub identifying_descriptions: Vec<String>,

/// 
    #[serde(rename = "LogCurrentNumberOfRecords")]
    pub log_current_number_of_records: Vec<u64>,

/// 
    #[serde(rename = "LogInstanceID")]
    pub log_instance_id: Vec<String>,

/// 
    #[serde(rename = "LogMaxNumberOfRecords")]
    pub log_max_number_of_records: Vec<u64>,

/// 
    #[serde(rename = "LogOverwritePolicy")]
    pub log_overwrite_policy: Vec<u16>,

/// 
    #[serde(rename = "LogState")]
    pub log_state: Vec<u16>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MemoryBlockSize")]
    pub memory_block_size: Option<u64>,

/// 
    #[serde(rename = "MemoryConsumableBlocks")]
    pub memory_consumable_blocks: Option<u64>,

/// 
    #[serde(rename = "MemoryNumberOfBlocks")]
    pub memory_number_of_blocks: Option<u64>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "NumberOfProcessorCores")]
    pub number_of_processor_cores: Option<u16>,

/// 
    #[serde(rename = "NumberOfProcessors")]
    pub number_of_processors: Option<u16>,

/// 
    #[serde(rename = "NumberOfProcessorThreads")]
    pub number_of_processor_threads: Option<u16>,

/// 
    #[serde(rename = "NumericSensorBaseUnits")]
    pub numeric_sensor_base_units: Vec<u16>,

/// 
    #[serde(rename = "NumericSensorContext")]
    pub numeric_sensor_context: Vec<String>,

/// 
    #[serde(rename = "NumericSensorCurrentReading")]
    pub numeric_sensor_current_reading: Vec<i32>,

/// 
    #[serde(rename = "NumericSensorCurrentState")]
    pub numeric_sensor_current_state: Vec<String>,

/// 
    #[serde(rename = "NumericSensorElementName")]
    pub numeric_sensor_element_name: Vec<String>,

/// 
    #[serde(rename = "NumericSensorEnabledState")]
    pub numeric_sensor_enabled_state: Vec<u16>,

/// 
    #[serde(rename = "NumericSensorHealthState")]
    pub numeric_sensor_health_state: Vec<u16>,

/// 
    #[serde(rename = "NumericSensorLowerThresholdCritical")]
    pub numeric_sensor_lower_threshold_critical: Vec<i32>,

/// 
    #[serde(rename = "NumericSensorLowerThresholdFatal")]
    pub numeric_sensor_lower_threshold_fatal: Vec<i32>,

/// 
    #[serde(rename = "NumericSensorLowerThresholdNonCritical")]
    pub numeric_sensor_lower_threshold_non_critical: Vec<i32>,

/// 
    #[serde(rename = "NumericSensorOtherSensorTypeDescription")]
    pub numeric_sensor_other_sensor_type_description: Vec<String>,

/// 
    #[serde(rename = "NumericSensorPrimaryStatus")]
    pub numeric_sensor_primary_status: Vec<u16>,

/// 
    #[serde(rename = "NumericSensorRateUnits")]
    pub numeric_sensor_rate_units: Vec<u16>,

/// 
    #[serde(rename = "NumericSensorSensorType")]
    pub numeric_sensor_sensor_type: Vec<u16>,

/// 
    #[serde(rename = "NumericSensorUnitModifier")]
    pub numeric_sensor_unit_modifier: Vec<i32>,

/// 
    #[serde(rename = "NumericSensorUpperThresholdCritical")]
    pub numeric_sensor_upper_threshold_critical: Vec<i32>,

/// 
    #[serde(rename = "NumericSensorUpperThresholdFatal")]
    pub numeric_sensor_upper_threshold_fatal: Vec<i32>,

/// 
    #[serde(rename = "NumericSensorUpperThresholdNonCritical")]
    pub numeric_sensor_upper_threshold_non_critical: Vec<i32>,

/// 
    #[serde(rename = "OneTimeBootSource")]
    pub one_time_boot_source: Option<u8>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OSEnabledState")]
    pub osenabled_state: Option<u16>,

/// 
    #[serde(rename = "OSType")]
    pub ostype: Option<u16>,

/// 
    #[serde(rename = "OSVersion")]
    pub osversion: Option<String>,

/// 
    #[serde(rename = "OtherDedicatedDescriptions")]
    pub other_dedicated_descriptions: Vec<String>,

/// 
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,

/// 
    #[serde(rename = "PartNumber")]
    pub part_number: Option<String>,

/// 
    #[serde(rename = "PersistentBootConfigOrder")]
    pub persistent_boot_config_order: Vec<u8>,

/// 
    #[serde(rename = "PowerAllocationLimit")]
    pub power_allocation_limit: Option<u64>,

/// 
    #[serde(rename = "PowerUtilizationMode")]
    pub power_utilization_mode: Option<u16>,

/// 
    #[serde(rename = "PowerUtilizationModesSupported")]
    pub power_utilization_modes_supported: Vec<u16>,

/// 
    #[serde(rename = "ProcessorCurrentClockSpeed")]
    pub processor_current_clock_speed: Option<u32>,

/// 
    #[serde(rename = "ProcessorFamily")]
    pub processor_family: Option<u16>,

/// 
    #[serde(rename = "ProcessorMaxClockSpeed")]
    pub processor_max_clock_speed: Option<u32>,

/// 
    #[serde(rename = "RequestedState")]
    pub requested_state: Option<u16>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "SKU")]
    pub sku: Option<String>,

/// 
    #[serde(rename = "StructuredBootString")]
    pub structured_boot_string: Vec<String>,

/// 
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl CIM_PhysicalComputerSystemView {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_View::new(),
            current_biosbuild_number: None,
            current_biosmajor_version: None,
            current_biosminor_version: None,
            current_biosrevision_number: None,
            current_biosversion_string: None,
            current_management_firmware_build_number: None,
            current_management_firmware_element_name: None,
            current_management_firmware_major_version: None,
            current_management_firmware_minor_version: None,
            current_management_firmware_revision_number: None,
            current_management_firmware_version_string: None,
            dedicated: Vec::new(),
            enabled_state: None,
            fruinfo_supported: None,
            health_state: None,
            identifying_descriptions: Vec::new(),
            log_current_number_of_records: Vec::new(),
            log_instance_id: Vec::new(),
            log_max_number_of_records: Vec::new(),
            log_overwrite_policy: Vec::new(),
            log_state: Vec::new(),
            manufacturer: None,
            memory_block_size: None,
            memory_consumable_blocks: None,
            memory_number_of_blocks: None,
            model: None,
            number_of_processor_cores: None,
            number_of_processors: None,
            number_of_processor_threads: None,
            numeric_sensor_base_units: Vec::new(),
            numeric_sensor_context: Vec::new(),
            numeric_sensor_current_reading: Vec::new(),
            numeric_sensor_current_state: Vec::new(),
            numeric_sensor_element_name: Vec::new(),
            numeric_sensor_enabled_state: Vec::new(),
            numeric_sensor_health_state: Vec::new(),
            numeric_sensor_lower_threshold_critical: Vec::new(),
            numeric_sensor_lower_threshold_fatal: Vec::new(),
            numeric_sensor_lower_threshold_non_critical: Vec::new(),
            numeric_sensor_other_sensor_type_description: Vec::new(),
            numeric_sensor_primary_status: Vec::new(),
            numeric_sensor_rate_units: Vec::new(),
            numeric_sensor_sensor_type: Vec::new(),
            numeric_sensor_unit_modifier: Vec::new(),
            numeric_sensor_upper_threshold_critical: Vec::new(),
            numeric_sensor_upper_threshold_fatal: Vec::new(),
            numeric_sensor_upper_threshold_non_critical: Vec::new(),
            one_time_boot_source: None,
            operational_status: Vec::new(),
            osenabled_state: None,
            ostype: None,
            osversion: None,
            other_dedicated_descriptions: Vec::new(),
            other_identifying_info: Vec::new(),
            part_number: None,
            persistent_boot_config_order: Vec::new(),
            power_allocation_limit: None,
            power_utilization_mode: None,
            power_utilization_modes_supported: Vec::new(),
            processor_current_clock_speed: None,
            processor_family: None,
            processor_max_clock_speed: None,
            requested_state: None,
            serial_number: None,
            sku: None,
            structured_boot_string: Vec::new(),
            tag: None,
            version: None,
        }
    }


    /// Sets the value of CurrentBIOSBuildNumber
    pub fn set_current_biosbuild_number(&mut self, value: u16) {
        self.current_biosbuild_number = Some(value);
    }

    /// Gets the value of CurrentBIOSBuildNumber
    pub fn get_current_biosbuild_number(&self) -> Option<&u16> {
        self.current_biosbuild_number.as_ref()
    }

    /// Sets the value of CurrentBIOSMajorVersion
    pub fn set_current_biosmajor_version(&mut self, value: u16) {
        self.current_biosmajor_version = Some(value);
    }

    /// Gets the value of CurrentBIOSMajorVersion
    pub fn get_current_biosmajor_version(&self) -> Option<&u16> {
        self.current_biosmajor_version.as_ref()
    }

    /// Sets the value of CurrentBIOSMinorVersion
    pub fn set_current_biosminor_version(&mut self, value: u16) {
        self.current_biosminor_version = Some(value);
    }

    /// Gets the value of CurrentBIOSMinorVersion
    pub fn get_current_biosminor_version(&self) -> Option<&u16> {
        self.current_biosminor_version.as_ref()
    }

    /// Sets the value of CurrentBIOSRevisionNumber
    pub fn set_current_biosrevision_number(&mut self, value: u16) {
        self.current_biosrevision_number = Some(value);
    }

    /// Gets the value of CurrentBIOSRevisionNumber
    pub fn get_current_biosrevision_number(&self) -> Option<&u16> {
        self.current_biosrevision_number.as_ref()
    }

    /// Sets the value of CurrentBIOSVersionString
    pub fn set_current_biosversion_string(&mut self, value: String) {
        self.current_biosversion_string = Some(value);
    }

    /// Gets the value of CurrentBIOSVersionString
    pub fn get_current_biosversion_string(&self) -> Option<&String> {
        self.current_biosversion_string.as_ref()
    }

    /// Sets the value of CurrentManagementFirmwareBuildNumber
    pub fn set_current_management_firmware_build_number(&mut self, value: u16) {
        self.current_management_firmware_build_number = Some(value);
    }

    /// Gets the value of CurrentManagementFirmwareBuildNumber
    pub fn get_current_management_firmware_build_number(&self) -> Option<&u16> {
        self.current_management_firmware_build_number.as_ref()
    }

    /// Sets the value of CurrentManagementFirmwareElementName
    pub fn set_current_management_firmware_element_name(&mut self, value: String) {
        self.current_management_firmware_element_name = Some(value);
    }

    /// Gets the value of CurrentManagementFirmwareElementName
    pub fn get_current_management_firmware_element_name(&self) -> Option<&String> {
        self.current_management_firmware_element_name.as_ref()
    }

    /// Sets the value of CurrentManagementFirmwareMajorVersion
    pub fn set_current_management_firmware_major_version(&mut self, value: u16) {
        self.current_management_firmware_major_version = Some(value);
    }

    /// Gets the value of CurrentManagementFirmwareMajorVersion
    pub fn get_current_management_firmware_major_version(&self) -> Option<&u16> {
        self.current_management_firmware_major_version.as_ref()
    }

    /// Sets the value of CurrentManagementFirmwareMinorVersion
    pub fn set_current_management_firmware_minor_version(&mut self, value: u16) {
        self.current_management_firmware_minor_version = Some(value);
    }

    /// Gets the value of CurrentManagementFirmwareMinorVersion
    pub fn get_current_management_firmware_minor_version(&self) -> Option<&u16> {
        self.current_management_firmware_minor_version.as_ref()
    }

    /// Sets the value of CurrentManagementFirmwareRevisionNumber
    pub fn set_current_management_firmware_revision_number(&mut self, value: u16) {
        self.current_management_firmware_revision_number = Some(value);
    }

    /// Gets the value of CurrentManagementFirmwareRevisionNumber
    pub fn get_current_management_firmware_revision_number(&self) -> Option<&u16> {
        self.current_management_firmware_revision_number.as_ref()
    }

    /// Sets the value of CurrentManagementFirmwareVersionString
    pub fn set_current_management_firmware_version_string(&mut self, value: String) {
        self.current_management_firmware_version_string = Some(value);
    }

    /// Gets the value of CurrentManagementFirmwareVersionString
    pub fn get_current_management_firmware_version_string(&self) -> Option<&String> {
        self.current_management_firmware_version_string.as_ref()
    }

    /// Sets the value of Dedicated
    pub fn set_dedicated(&mut self, value: Vec<u16>) {
        self.dedicated = value;
    }

    /// Gets the value of Dedicated
    pub fn get_dedicated(&self) -> &Vec<u16> {
        &self.dedicated
    }

    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: u16) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&u16> {
        self.enabled_state.as_ref()
    }

    /// Sets the value of FRUInfoSupported
    pub fn set_fruinfo_supported(&mut self, value: bool) {
        self.fruinfo_supported = Some(value);
    }

    /// Gets the value of FRUInfoSupported
    pub fn get_fruinfo_supported(&self) -> Option<&bool> {
        self.fruinfo_supported.as_ref()
    }

    /// Sets the value of HealthState
    pub fn set_health_state(&mut self, value: u16) {
        self.health_state = Some(value);
    }

    /// Gets the value of HealthState
    pub fn get_health_state(&self) -> Option<&u16> {
        self.health_state.as_ref()
    }

    /// Sets the value of IdentifyingDescriptions
    pub fn set_identifying_descriptions(&mut self, value: Vec<String>) {
        self.identifying_descriptions = value;
    }

    /// Gets the value of IdentifyingDescriptions
    pub fn get_identifying_descriptions(&self) -> &Vec<String> {
        &self.identifying_descriptions
    }

    /// Sets the value of LogCurrentNumberOfRecords
    pub fn set_log_current_number_of_records(&mut self, value: Vec<u64>) {
        self.log_current_number_of_records = value;
    }

    /// Gets the value of LogCurrentNumberOfRecords
    pub fn get_log_current_number_of_records(&self) -> &Vec<u64> {
        &self.log_current_number_of_records
    }

    /// Sets the value of LogInstanceID
    pub fn set_log_instance_id(&mut self, value: Vec<String>) {
        self.log_instance_id = value;
    }

    /// Gets the value of LogInstanceID
    pub fn get_log_instance_id(&self) -> &Vec<String> {
        &self.log_instance_id
    }

    /// Sets the value of LogMaxNumberOfRecords
    pub fn set_log_max_number_of_records(&mut self, value: Vec<u64>) {
        self.log_max_number_of_records = value;
    }

    /// Gets the value of LogMaxNumberOfRecords
    pub fn get_log_max_number_of_records(&self) -> &Vec<u64> {
        &self.log_max_number_of_records
    }

    /// Sets the value of LogOverwritePolicy
    pub fn set_log_overwrite_policy(&mut self, value: Vec<u16>) {
        self.log_overwrite_policy = value;
    }

    /// Gets the value of LogOverwritePolicy
    pub fn get_log_overwrite_policy(&self) -> &Vec<u16> {
        &self.log_overwrite_policy
    }

    /// Sets the value of LogState
    pub fn set_log_state(&mut self, value: Vec<u16>) {
        self.log_state = value;
    }

    /// Gets the value of LogState
    pub fn get_log_state(&self) -> &Vec<u16> {
        &self.log_state
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MemoryBlockSize
    pub fn set_memory_block_size(&mut self, value: u64) {
        self.memory_block_size = Some(value);
    }

    /// Gets the value of MemoryBlockSize
    pub fn get_memory_block_size(&self) -> Option<&u64> {
        self.memory_block_size.as_ref()
    }

    /// Sets the value of MemoryConsumableBlocks
    pub fn set_memory_consumable_blocks(&mut self, value: u64) {
        self.memory_consumable_blocks = Some(value);
    }

    /// Gets the value of MemoryConsumableBlocks
    pub fn get_memory_consumable_blocks(&self) -> Option<&u64> {
        self.memory_consumable_blocks.as_ref()
    }

    /// Sets the value of MemoryNumberOfBlocks
    pub fn set_memory_number_of_blocks(&mut self, value: u64) {
        self.memory_number_of_blocks = Some(value);
    }

    /// Gets the value of MemoryNumberOfBlocks
    pub fn get_memory_number_of_blocks(&self) -> Option<&u64> {
        self.memory_number_of_blocks.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of NumberOfProcessorCores
    pub fn set_number_of_processor_cores(&mut self, value: u16) {
        self.number_of_processor_cores = Some(value);
    }

    /// Gets the value of NumberOfProcessorCores
    pub fn get_number_of_processor_cores(&self) -> Option<&u16> {
        self.number_of_processor_cores.as_ref()
    }

    /// Sets the value of NumberOfProcessors
    pub fn set_number_of_processors(&mut self, value: u16) {
        self.number_of_processors = Some(value);
    }

    /// Gets the value of NumberOfProcessors
    pub fn get_number_of_processors(&self) -> Option<&u16> {
        self.number_of_processors.as_ref()
    }

    /// Sets the value of NumberOfProcessorThreads
    pub fn set_number_of_processor_threads(&mut self, value: u16) {
        self.number_of_processor_threads = Some(value);
    }

    /// Gets the value of NumberOfProcessorThreads
    pub fn get_number_of_processor_threads(&self) -> Option<&u16> {
        self.number_of_processor_threads.as_ref()
    }

    /// Sets the value of NumericSensorBaseUnits
    pub fn set_numeric_sensor_base_units(&mut self, value: Vec<u16>) {
        self.numeric_sensor_base_units = value;
    }

    /// Gets the value of NumericSensorBaseUnits
    pub fn get_numeric_sensor_base_units(&self) -> &Vec<u16> {
        &self.numeric_sensor_base_units
    }

    /// Sets the value of NumericSensorContext
    pub fn set_numeric_sensor_context(&mut self, value: Vec<String>) {
        self.numeric_sensor_context = value;
    }

    /// Gets the value of NumericSensorContext
    pub fn get_numeric_sensor_context(&self) -> &Vec<String> {
        &self.numeric_sensor_context
    }

    /// Sets the value of NumericSensorCurrentReading
    pub fn set_numeric_sensor_current_reading(&mut self, value: Vec<i32>) {
        self.numeric_sensor_current_reading = value;
    }

    /// Gets the value of NumericSensorCurrentReading
    pub fn get_numeric_sensor_current_reading(&self) -> &Vec<i32> {
        &self.numeric_sensor_current_reading
    }

    /// Sets the value of NumericSensorCurrentState
    pub fn set_numeric_sensor_current_state(&mut self, value: Vec<String>) {
        self.numeric_sensor_current_state = value;
    }

    /// Gets the value of NumericSensorCurrentState
    pub fn get_numeric_sensor_current_state(&self) -> &Vec<String> {
        &self.numeric_sensor_current_state
    }

    /// Sets the value of NumericSensorElementName
    pub fn set_numeric_sensor_element_name(&mut self, value: Vec<String>) {
        self.numeric_sensor_element_name = value;
    }

    /// Gets the value of NumericSensorElementName
    pub fn get_numeric_sensor_element_name(&self) -> &Vec<String> {
        &self.numeric_sensor_element_name
    }

    /// Sets the value of NumericSensorEnabledState
    pub fn set_numeric_sensor_enabled_state(&mut self, value: Vec<u16>) {
        self.numeric_sensor_enabled_state = value;
    }

    /// Gets the value of NumericSensorEnabledState
    pub fn get_numeric_sensor_enabled_state(&self) -> &Vec<u16> {
        &self.numeric_sensor_enabled_state
    }

    /// Sets the value of NumericSensorHealthState
    pub fn set_numeric_sensor_health_state(&mut self, value: Vec<u16>) {
        self.numeric_sensor_health_state = value;
    }

    /// Gets the value of NumericSensorHealthState
    pub fn get_numeric_sensor_health_state(&self) -> &Vec<u16> {
        &self.numeric_sensor_health_state
    }

    /// Sets the value of NumericSensorLowerThresholdCritical
    pub fn set_numeric_sensor_lower_threshold_critical(&mut self, value: Vec<i32>) {
        self.numeric_sensor_lower_threshold_critical = value;
    }

    /// Gets the value of NumericSensorLowerThresholdCritical
    pub fn get_numeric_sensor_lower_threshold_critical(&self) -> &Vec<i32> {
        &self.numeric_sensor_lower_threshold_critical
    }

    /// Sets the value of NumericSensorLowerThresholdFatal
    pub fn set_numeric_sensor_lower_threshold_fatal(&mut self, value: Vec<i32>) {
        self.numeric_sensor_lower_threshold_fatal = value;
    }

    /// Gets the value of NumericSensorLowerThresholdFatal
    pub fn get_numeric_sensor_lower_threshold_fatal(&self) -> &Vec<i32> {
        &self.numeric_sensor_lower_threshold_fatal
    }

    /// Sets the value of NumericSensorLowerThresholdNonCritical
    pub fn set_numeric_sensor_lower_threshold_non_critical(&mut self, value: Vec<i32>) {
        self.numeric_sensor_lower_threshold_non_critical = value;
    }

    /// Gets the value of NumericSensorLowerThresholdNonCritical
    pub fn get_numeric_sensor_lower_threshold_non_critical(&self) -> &Vec<i32> {
        &self.numeric_sensor_lower_threshold_non_critical
    }

    /// Sets the value of NumericSensorOtherSensorTypeDescription
    pub fn set_numeric_sensor_other_sensor_type_description(&mut self, value: Vec<String>) {
        self.numeric_sensor_other_sensor_type_description = value;
    }

    /// Gets the value of NumericSensorOtherSensorTypeDescription
    pub fn get_numeric_sensor_other_sensor_type_description(&self) -> &Vec<String> {
        &self.numeric_sensor_other_sensor_type_description
    }

    /// Sets the value of NumericSensorPrimaryStatus
    pub fn set_numeric_sensor_primary_status(&mut self, value: Vec<u16>) {
        self.numeric_sensor_primary_status = value;
    }

    /// Gets the value of NumericSensorPrimaryStatus
    pub fn get_numeric_sensor_primary_status(&self) -> &Vec<u16> {
        &self.numeric_sensor_primary_status
    }

    /// Sets the value of NumericSensorRateUnits
    pub fn set_numeric_sensor_rate_units(&mut self, value: Vec<u16>) {
        self.numeric_sensor_rate_units = value;
    }

    /// Gets the value of NumericSensorRateUnits
    pub fn get_numeric_sensor_rate_units(&self) -> &Vec<u16> {
        &self.numeric_sensor_rate_units
    }

    /// Sets the value of NumericSensorSensorType
    pub fn set_numeric_sensor_sensor_type(&mut self, value: Vec<u16>) {
        self.numeric_sensor_sensor_type = value;
    }

    /// Gets the value of NumericSensorSensorType
    pub fn get_numeric_sensor_sensor_type(&self) -> &Vec<u16> {
        &self.numeric_sensor_sensor_type
    }

    /// Sets the value of NumericSensorUnitModifier
    pub fn set_numeric_sensor_unit_modifier(&mut self, value: Vec<i32>) {
        self.numeric_sensor_unit_modifier = value;
    }

    /// Gets the value of NumericSensorUnitModifier
    pub fn get_numeric_sensor_unit_modifier(&self) -> &Vec<i32> {
        &self.numeric_sensor_unit_modifier
    }

    /// Sets the value of NumericSensorUpperThresholdCritical
    pub fn set_numeric_sensor_upper_threshold_critical(&mut self, value: Vec<i32>) {
        self.numeric_sensor_upper_threshold_critical = value;
    }

    /// Gets the value of NumericSensorUpperThresholdCritical
    pub fn get_numeric_sensor_upper_threshold_critical(&self) -> &Vec<i32> {
        &self.numeric_sensor_upper_threshold_critical
    }

    /// Sets the value of NumericSensorUpperThresholdFatal
    pub fn set_numeric_sensor_upper_threshold_fatal(&mut self, value: Vec<i32>) {
        self.numeric_sensor_upper_threshold_fatal = value;
    }

    /// Gets the value of NumericSensorUpperThresholdFatal
    pub fn get_numeric_sensor_upper_threshold_fatal(&self) -> &Vec<i32> {
        &self.numeric_sensor_upper_threshold_fatal
    }

    /// Sets the value of NumericSensorUpperThresholdNonCritical
    pub fn set_numeric_sensor_upper_threshold_non_critical(&mut self, value: Vec<i32>) {
        self.numeric_sensor_upper_threshold_non_critical = value;
    }

    /// Gets the value of NumericSensorUpperThresholdNonCritical
    pub fn get_numeric_sensor_upper_threshold_non_critical(&self) -> &Vec<i32> {
        &self.numeric_sensor_upper_threshold_non_critical
    }

    /// Sets the value of OneTimeBootSource
    pub fn set_one_time_boot_source(&mut self, value: u8) {
        self.one_time_boot_source = Some(value);
    }

    /// Gets the value of OneTimeBootSource
    pub fn get_one_time_boot_source(&self) -> Option<&u8> {
        self.one_time_boot_source.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of OSEnabledState
    pub fn set_osenabled_state(&mut self, value: u16) {
        self.osenabled_state = Some(value);
    }

    /// Gets the value of OSEnabledState
    pub fn get_osenabled_state(&self) -> Option<&u16> {
        self.osenabled_state.as_ref()
    }

    /// Sets the value of OSType
    pub fn set_ostype(&mut self, value: u16) {
        self.ostype = Some(value);
    }

    /// Gets the value of OSType
    pub fn get_ostype(&self) -> Option<&u16> {
        self.ostype.as_ref()
    }

    /// Sets the value of OSVersion
    pub fn set_osversion(&mut self, value: String) {
        self.osversion = Some(value);
    }

    /// Gets the value of OSVersion
    pub fn get_osversion(&self) -> Option<&String> {
        self.osversion.as_ref()
    }

    /// Sets the value of OtherDedicatedDescriptions
    pub fn set_other_dedicated_descriptions(&mut self, value: Vec<String>) {
        self.other_dedicated_descriptions = value;
    }

    /// Gets the value of OtherDedicatedDescriptions
    pub fn get_other_dedicated_descriptions(&self) -> &Vec<String> {
        &self.other_dedicated_descriptions
    }

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: Vec<String>) {
        self.other_identifying_info = value;
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> &Vec<String> {
        &self.other_identifying_info
    }

    /// Sets the value of PartNumber
    pub fn set_part_number(&mut self, value: String) {
        self.part_number = Some(value);
    }

    /// Gets the value of PartNumber
    pub fn get_part_number(&self) -> Option<&String> {
        self.part_number.as_ref()
    }

    /// Sets the value of PersistentBootConfigOrder
    pub fn set_persistent_boot_config_order(&mut self, value: Vec<u8>) {
        self.persistent_boot_config_order = value;
    }

    /// Gets the value of PersistentBootConfigOrder
    pub fn get_persistent_boot_config_order(&self) -> &Vec<u8> {
        &self.persistent_boot_config_order
    }

    /// Sets the value of PowerAllocationLimit
    pub fn set_power_allocation_limit(&mut self, value: u64) {
        self.power_allocation_limit = Some(value);
    }

    /// Gets the value of PowerAllocationLimit
    pub fn get_power_allocation_limit(&self) -> Option<&u64> {
        self.power_allocation_limit.as_ref()
    }

    /// Sets the value of PowerUtilizationMode
    pub fn set_power_utilization_mode(&mut self, value: u16) {
        self.power_utilization_mode = Some(value);
    }

    /// Gets the value of PowerUtilizationMode
    pub fn get_power_utilization_mode(&self) -> Option<&u16> {
        self.power_utilization_mode.as_ref()
    }

    /// Sets the value of PowerUtilizationModesSupported
    pub fn set_power_utilization_modes_supported(&mut self, value: Vec<u16>) {
        self.power_utilization_modes_supported = value;
    }

    /// Gets the value of PowerUtilizationModesSupported
    pub fn get_power_utilization_modes_supported(&self) -> &Vec<u16> {
        &self.power_utilization_modes_supported
    }

    /// Sets the value of ProcessorCurrentClockSpeed
    pub fn set_processor_current_clock_speed(&mut self, value: u32) {
        self.processor_current_clock_speed = Some(value);
    }

    /// Gets the value of ProcessorCurrentClockSpeed
    pub fn get_processor_current_clock_speed(&self) -> Option<&u32> {
        self.processor_current_clock_speed.as_ref()
    }

    /// Sets the value of ProcessorFamily
    pub fn set_processor_family(&mut self, value: u16) {
        self.processor_family = Some(value);
    }

    /// Gets the value of ProcessorFamily
    pub fn get_processor_family(&self) -> Option<&u16> {
        self.processor_family.as_ref()
    }

    /// Sets the value of ProcessorMaxClockSpeed
    pub fn set_processor_max_clock_speed(&mut self, value: u32) {
        self.processor_max_clock_speed = Some(value);
    }

    /// Gets the value of ProcessorMaxClockSpeed
    pub fn get_processor_max_clock_speed(&self) -> Option<&u32> {
        self.processor_max_clock_speed.as_ref()
    }

    /// Sets the value of RequestedState
    pub fn set_requested_state(&mut self, value: u16) {
        self.requested_state = Some(value);
    }

    /// Gets the value of RequestedState
    pub fn get_requested_state(&self) -> Option<&u16> {
        self.requested_state.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of SKU
    pub fn set_sku(&mut self, value: String) {
        self.sku = Some(value);
    }

    /// Gets the value of SKU
    pub fn get_sku(&self) -> Option<&String> {
        self.sku.as_ref()
    }

    /// Sets the value of StructuredBootString
    pub fn set_structured_boot_string(&mut self, value: Vec<String>) {
        self.structured_boot_string = value;
    }

    /// Gets the value of StructuredBootString
    pub fn get_structured_boot_string(&self) -> &Vec<String> {
        &self.structured_boot_string
    }

    /// Sets the value of Tag
    pub fn set_tag(&mut self, value: String) {
        self.tag = Some(value);
    }

    /// Gets the value of Tag
    pub fn get_tag(&self) -> Option<&String> {
        self.tag.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

/// 

    /// * `job` -  (CIM_ConcreteJob)
    /// * `requested_state` -  (u16)
    /// * `timeout_period` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn request_state_change(&self, requested_state: u16, job: &mut CIM_ConcreteJob, timeout_period: &String, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });
        args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: timeout_period.into() });

        let result = self.invoke_method_with_job("RequestStateChange", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `log_instance_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn clear_log(&self, log_instance_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LogInstanceID".to_string(), value: log_instance_id.into() });
        self.invoke_method("ClearLog", &args)

    }


/// 

    /// * `classifications` -  (u16[])
    /// * `install_options` -  (u16[])
    /// * `install_options_values` -  (String[])
    /// * `job` -  (CIM_ConcreteJob)
    /// * `uri` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn install_software_from_uri(&self, job: &mut CIM_ConcreteJob, classifications: &Vec<u16>, uri: &String, install_options: &Vec<u16>, install_options_values: &Vec<String>, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Classifications".to_string(), value: classifications.into() });
        args.push(MethodParameter { name: "URI".to_string(), value: uri.into() });
        args.push(MethodParameter { name: "InstallOptions".to_string(), value: install_options.into() });
        args.push(MethodParameter { name: "InstallOptionsValues".to_string(), value: install_options_values.into() });

        let result = self.invoke_method_with_job("InstallSoftwareFromURI", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `job` -  (CIM_ConcreteJob)
    /// * `structured_boot_string` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_persistent_boot_config_order(&self, structured_boot_string: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StructuredBootString".to_string(), value: structured_boot_string.into() });

        let result = self.invoke_method_with_job("ModifyPersistentBootConfigOrder", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `job` -  (CIM_ConcreteJob)
    /// * `structured_boot_string` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_one_time_boot_source(&self, structured_boot_string: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StructuredBootString".to_string(), value: structured_boot_string.into() });

        let result = self.invoke_method_with_job("SetOneTimeBootSource", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

