// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemManagementServiceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemManagementServiceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "BiosLockString")]
    pub bios_lock_string: Option<String>,

/// 
    #[serde(rename = "CurrentWWNNAddress")]
    pub current_wwnnaddress: Option<String>,

/// 
    #[serde(rename = "DefaultExternalDataRoot")]
    pub default_external_data_root: Option<String>,

/// 
    #[serde(rename = "DefaultVirtualHardDiskCachingMode")]
    pub default_virtual_hard_disk_caching_mode: Option<u16>,

/// 
    #[serde(rename = "DefaultVirtualHardDiskPath")]
    pub default_virtual_hard_disk_path: Option<String>,

/// 
    #[serde(rename = "EnhancedSessionModeEnabled")]
    pub enhanced_session_mode_enabled: Option<bool>,

/// 
    #[serde(rename = "HbaLunTimeout")]
    pub hba_lun_timeout: Option<u32>,

/// 
    #[serde(rename = "HypervisorRootSchedulerEnabled")]
    pub hypervisor_root_scheduler_enabled: Option<bool>,

/// 
    #[serde(rename = "HypervisorSnpStatus")]
    pub hypervisor_snp_status: Option<u16>,

/// 
    #[serde(rename = "HypervisorTdxStatus")]
    pub hypervisor_tdx_status: Option<u16>,

/// 
    #[serde(rename = "MaximumMacAddress")]
    pub maximum_mac_address: Option<String>,

/// 
    #[serde(rename = "MaximumWWPNAddress")]
    pub maximum_wwpnaddress: Option<String>,

/// 
    #[serde(rename = "MinimumMacAddress")]
    pub minimum_mac_address: Option<String>,

/// 
    #[serde(rename = "MinimumWWPNAddress")]
    pub minimum_wwpnaddress: Option<String>,

/// 
    #[serde(rename = "NumaSpanningEnabled")]
    pub numa_spanning_enabled: Option<bool>,

/// 
    #[serde(rename = "PrimaryOwnerContact")]
    pub primary_owner_contact: Option<String>,

/// 
    #[serde(rename = "PrimaryOwnerName")]
    pub primary_owner_name: Option<String>,
}

impl Msvm_VirtualSystemManagementServiceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            bios_lock_string: None,
            current_wwnnaddress: None,
            default_external_data_root: None,
            default_virtual_hard_disk_caching_mode: None,
            default_virtual_hard_disk_path: None,
            enhanced_session_mode_enabled: None,
            hba_lun_timeout: None,
            hypervisor_root_scheduler_enabled: None,
            hypervisor_snp_status: None,
            hypervisor_tdx_status: None,
            maximum_mac_address: None,
            maximum_wwpnaddress: None,
            minimum_mac_address: None,
            minimum_wwpnaddress: None,
            numa_spanning_enabled: None,
            primary_owner_contact: None,
            primary_owner_name: None,
        }
    }


    /// Sets the value of BiosLockString
    pub fn set_bios_lock_string(&mut self, value: String) {
        self.bios_lock_string = Some(value);
    }

    /// Gets the value of BiosLockString
    pub fn get_bios_lock_string(&self) -> Option<&String> {
        self.bios_lock_string.as_ref()
    }

    /// Sets the value of CurrentWWNNAddress
    pub fn set_current_wwnnaddress(&mut self, value: String) {
        self.current_wwnnaddress = Some(value);
    }

    /// Gets the value of CurrentWWNNAddress
    pub fn get_current_wwnnaddress(&self) -> Option<&String> {
        self.current_wwnnaddress.as_ref()
    }

    /// Sets the value of DefaultExternalDataRoot
    pub fn set_default_external_data_root(&mut self, value: String) {
        self.default_external_data_root = Some(value);
    }

    /// Gets the value of DefaultExternalDataRoot
    pub fn get_default_external_data_root(&self) -> Option<&String> {
        self.default_external_data_root.as_ref()
    }

    /// Sets the value of DefaultVirtualHardDiskCachingMode
    pub fn set_default_virtual_hard_disk_caching_mode(&mut self, value: u16) {
        self.default_virtual_hard_disk_caching_mode = Some(value);
    }

    /// Gets the value of DefaultVirtualHardDiskCachingMode
    pub fn get_default_virtual_hard_disk_caching_mode(&self) -> Option<&u16> {
        self.default_virtual_hard_disk_caching_mode.as_ref()
    }

    /// Sets the value of DefaultVirtualHardDiskPath
    pub fn set_default_virtual_hard_disk_path(&mut self, value: String) {
        self.default_virtual_hard_disk_path = Some(value);
    }

    /// Gets the value of DefaultVirtualHardDiskPath
    pub fn get_default_virtual_hard_disk_path(&self) -> Option<&String> {
        self.default_virtual_hard_disk_path.as_ref()
    }

    /// Sets the value of EnhancedSessionModeEnabled
    pub fn set_enhanced_session_mode_enabled(&mut self, value: bool) {
        self.enhanced_session_mode_enabled = Some(value);
    }

    /// Gets the value of EnhancedSessionModeEnabled
    pub fn get_enhanced_session_mode_enabled(&self) -> Option<&bool> {
        self.enhanced_session_mode_enabled.as_ref()
    }

    /// Sets the value of HbaLunTimeout
    pub fn set_hba_lun_timeout(&mut self, value: u32) {
        self.hba_lun_timeout = Some(value);
    }

    /// Gets the value of HbaLunTimeout
    pub fn get_hba_lun_timeout(&self) -> Option<&u32> {
        self.hba_lun_timeout.as_ref()
    }

    /// Sets the value of HypervisorRootSchedulerEnabled
    pub fn set_hypervisor_root_scheduler_enabled(&mut self, value: bool) {
        self.hypervisor_root_scheduler_enabled = Some(value);
    }

    /// Gets the value of HypervisorRootSchedulerEnabled
    pub fn get_hypervisor_root_scheduler_enabled(&self) -> Option<&bool> {
        self.hypervisor_root_scheduler_enabled.as_ref()
    }

    /// Sets the value of HypervisorSnpStatus
    pub fn set_hypervisor_snp_status(&mut self, value: u16) {
        self.hypervisor_snp_status = Some(value);
    }

    /// Gets the value of HypervisorSnpStatus
    pub fn get_hypervisor_snp_status(&self) -> Option<&u16> {
        self.hypervisor_snp_status.as_ref()
    }

    /// Sets the value of HypervisorTdxStatus
    pub fn set_hypervisor_tdx_status(&mut self, value: u16) {
        self.hypervisor_tdx_status = Some(value);
    }

    /// Gets the value of HypervisorTdxStatus
    pub fn get_hypervisor_tdx_status(&self) -> Option<&u16> {
        self.hypervisor_tdx_status.as_ref()
    }

    /// Sets the value of MaximumMacAddress
    pub fn set_maximum_mac_address(&mut self, value: String) {
        self.maximum_mac_address = Some(value);
    }

    /// Gets the value of MaximumMacAddress
    pub fn get_maximum_mac_address(&self) -> Option<&String> {
        self.maximum_mac_address.as_ref()
    }

    /// Sets the value of MaximumWWPNAddress
    pub fn set_maximum_wwpnaddress(&mut self, value: String) {
        self.maximum_wwpnaddress = Some(value);
    }

    /// Gets the value of MaximumWWPNAddress
    pub fn get_maximum_wwpnaddress(&self) -> Option<&String> {
        self.maximum_wwpnaddress.as_ref()
    }

    /// Sets the value of MinimumMacAddress
    pub fn set_minimum_mac_address(&mut self, value: String) {
        self.minimum_mac_address = Some(value);
    }

    /// Gets the value of MinimumMacAddress
    pub fn get_minimum_mac_address(&self) -> Option<&String> {
        self.minimum_mac_address.as_ref()
    }

    /// Sets the value of MinimumWWPNAddress
    pub fn set_minimum_wwpnaddress(&mut self, value: String) {
        self.minimum_wwpnaddress = Some(value);
    }

    /// Gets the value of MinimumWWPNAddress
    pub fn get_minimum_wwpnaddress(&self) -> Option<&String> {
        self.minimum_wwpnaddress.as_ref()
    }

    /// Sets the value of NumaSpanningEnabled
    pub fn set_numa_spanning_enabled(&mut self, value: bool) {
        self.numa_spanning_enabled = Some(value);
    }

    /// Gets the value of NumaSpanningEnabled
    pub fn get_numa_spanning_enabled(&self) -> Option<&bool> {
        self.numa_spanning_enabled.as_ref()
    }

    /// Sets the value of PrimaryOwnerContact
    pub fn set_primary_owner_contact(&mut self, value: String) {
        self.primary_owner_contact = Some(value);
    }

    /// Gets the value of PrimaryOwnerContact
    pub fn get_primary_owner_contact(&self) -> Option<&String> {
        self.primary_owner_contact.as_ref()
    }

    /// Sets the value of PrimaryOwnerName
    pub fn set_primary_owner_name(&mut self, value: String) {
        self.primary_owner_name = Some(value);
    }

    /// Gets the value of PrimaryOwnerName
    pub fn get_primary_owner_name(&self) -> Option<&String> {
        self.primary_owner_name.as_ref()
    }
}

impl Msvm_VirtualSystemManagementServiceSettingData {
    /// Gets the related Msvm_VirtualSystemManagementService object(s)
    pub fn get_related__virtual_system_management_service(&self) -> Result<Msvm_VirtualSystemManagementService, WmiError> {
        self.get_related("Msvm_VirtualSystemManagementService")
    }

}

