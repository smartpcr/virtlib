// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemMigrationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemMigrationSettingData {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemMigrationSettingData,

/// 
    #[serde(rename = "AdvancedOptions")]
    pub advanced_options: Option<String>,

/// 
    #[serde(rename = "AllowOverwriteExistingFile")]
    pub allow_overwrite_existing_file: Option<bool>,

/// 
    #[serde(rename = "AvoidRemovingVHDs")]
    pub avoid_removing_vhds: Option<bool>,

/// 
    #[serde(rename = "CancelIfBlackoutThresholdExceeded")]
    pub cancel_if_blackout_threshold_exceeded: Option<bool>,

/// 
    #[serde(rename = "CPUCappingMagnitude")]
    pub cpucapping_magnitude: Option<VirtualSystemMigrationSettingData_CPUCappingMagnitude>,

/// 
    #[serde(rename = "DestinationIPAddressList")]
    pub destination_ipaddress_list: Vec<String>,

/// 
    #[serde(rename = "DestinationPlannedVirtualSystemId")]
    pub destination_planned_virtual_system_id: Option<String>,

/// 
    #[serde(rename = "EnableCompression")]
    pub enable_compression: Option<bool>,

/// 
    #[serde(rename = "EnableVhdCompression")]
    pub enable_vhd_compression: Option<bool>,

/// 
    #[serde(rename = "RemoveSourceUnmanagedVhds")]
    pub remove_source_unmanaged_vhds: Option<bool>,

/// 
    #[serde(rename = "RetainVhdCopiesOnSource")]
    pub retain_vhd_copies_on_source: Option<bool>,

/// 
    #[serde(rename = "UnmanagedVhds")]
    pub unmanaged_vhds: Vec<String>,
}

impl Msvm_VirtualSystemMigrationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemMigrationSettingData::new(),
            advanced_options: None,
            allow_overwrite_existing_file: None,
            avoid_removing_vhds: None,
            cancel_if_blackout_threshold_exceeded: None,
            cpucapping_magnitude: None,
            destination_ipaddress_list: Vec::new(),
            destination_planned_virtual_system_id: None,
            enable_compression: None,
            enable_vhd_compression: None,
            remove_source_unmanaged_vhds: None,
            retain_vhd_copies_on_source: None,
            unmanaged_vhds: Vec::new(),
        }
    }


    /// Sets the value of AdvancedOptions
    pub fn set_advanced_options(&mut self, value: String) {
        self.advanced_options = Some(value);
    }

    /// Gets the value of AdvancedOptions
    pub fn get_advanced_options(&self) -> Option<&String> {
        self.advanced_options.as_ref()
    }

    /// Sets the value of AllowOverwriteExistingFile
    pub fn set_allow_overwrite_existing_file(&mut self, value: bool) {
        self.allow_overwrite_existing_file = Some(value);
    }

    /// Gets the value of AllowOverwriteExistingFile
    pub fn get_allow_overwrite_existing_file(&self) -> Option<&bool> {
        self.allow_overwrite_existing_file.as_ref()
    }

    /// Sets the value of AvoidRemovingVHDs
    pub fn set_avoid_removing_vhds(&mut self, value: bool) {
        self.avoid_removing_vhds = Some(value);
    }

    /// Gets the value of AvoidRemovingVHDs
    pub fn get_avoid_removing_vhds(&self) -> Option<&bool> {
        self.avoid_removing_vhds.as_ref()
    }

    /// Sets the value of CancelIfBlackoutThresholdExceeded
    pub fn set_cancel_if_blackout_threshold_exceeded(&mut self, value: bool) {
        self.cancel_if_blackout_threshold_exceeded = Some(value);
    }

    /// Gets the value of CancelIfBlackoutThresholdExceeded
    pub fn get_cancel_if_blackout_threshold_exceeded(&self) -> Option<&bool> {
        self.cancel_if_blackout_threshold_exceeded.as_ref()
    }

    /// Sets the value of CPUCappingMagnitude
    pub fn set_cpucapping_magnitude(&mut self, value: VirtualSystemMigrationSettingData_CPUCappingMagnitude) {
        self.cpucapping_magnitude = Some(value);
    }

    /// Gets the value of CPUCappingMagnitude
    pub fn get_cpucapping_magnitude(&self) -> Option<&VirtualSystemMigrationSettingData_CPUCappingMagnitude> {
        self.cpucapping_magnitude.as_ref()
    }

    /// Sets the value of DestinationIPAddressList
    pub fn set_destination_ipaddress_list(&mut self, value: Vec<String>) {
        self.destination_ipaddress_list = value;
    }

    /// Gets the value of DestinationIPAddressList
    pub fn get_destination_ipaddress_list(&self) -> &Vec<String> {
        &self.destination_ipaddress_list
    }

    /// Sets the value of DestinationPlannedVirtualSystemId
    pub fn set_destination_planned_virtual_system_id(&mut self, value: String) {
        self.destination_planned_virtual_system_id = Some(value);
    }

    /// Gets the value of DestinationPlannedVirtualSystemId
    pub fn get_destination_planned_virtual_system_id(&self) -> Option<&String> {
        self.destination_planned_virtual_system_id.as_ref()
    }

    /// Sets the value of EnableCompression
    pub fn set_enable_compression(&mut self, value: bool) {
        self.enable_compression = Some(value);
    }

    /// Gets the value of EnableCompression
    pub fn get_enable_compression(&self) -> Option<&bool> {
        self.enable_compression.as_ref()
    }

    /// Sets the value of EnableVhdCompression
    pub fn set_enable_vhd_compression(&mut self, value: bool) {
        self.enable_vhd_compression = Some(value);
    }

    /// Gets the value of EnableVhdCompression
    pub fn get_enable_vhd_compression(&self) -> Option<&bool> {
        self.enable_vhd_compression.as_ref()
    }

    /// Sets the value of RemoveSourceUnmanagedVhds
    pub fn set_remove_source_unmanaged_vhds(&mut self, value: bool) {
        self.remove_source_unmanaged_vhds = Some(value);
    }

    /// Gets the value of RemoveSourceUnmanagedVhds
    pub fn get_remove_source_unmanaged_vhds(&self) -> Option<&bool> {
        self.remove_source_unmanaged_vhds.as_ref()
    }

    /// Sets the value of RetainVhdCopiesOnSource
    pub fn set_retain_vhd_copies_on_source(&mut self, value: bool) {
        self.retain_vhd_copies_on_source = Some(value);
    }

    /// Gets the value of RetainVhdCopiesOnSource
    pub fn get_retain_vhd_copies_on_source(&self) -> Option<&bool> {
        self.retain_vhd_copies_on_source.as_ref()
    }

    /// Sets the value of UnmanagedVhds
    pub fn set_unmanaged_vhds(&mut self, value: Vec<String>) {
        self.unmanaged_vhds = value;
    }

    /// Gets the value of UnmanagedVhds
    pub fn get_unmanaged_vhds(&self) -> &Vec<String> {
        &self.unmanaged_vhds
    }
}

impl Msvm_VirtualSystemMigrationSettingData {
    /// Gets the related Msvm_VirtualSystemMigrationCapabilities object(s)
    pub fn get_related__virtual_system_migration_capabilities(&self) -> Result<Msvm_VirtualSystemMigrationCapabilities, WmiError> {
        self.get_related("Msvm_VirtualSystemMigrationCapabilities")
    }

}

