// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SecuritySettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SecuritySettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "AppContainerLaunchOptOut")]
    pub app_container_launch_opt_out: Option<bool>,

/// 
    #[serde(rename = "BindToHostTpm")]
    pub bind_to_host_tpm: Option<bool>,

/// 
    #[serde(rename = "DataProtectionRequested")]
    pub data_protection_requested: Option<bool>,

/// 
    #[serde(rename = "EncryptStateAndVmMigrationTraffic")]
    pub encrypt_state_and_vm_migration_traffic: Option<bool>,

/// 
    #[serde(rename = "KsdEnabled")]
    pub ksd_enabled: Option<bool>,

/// 
    #[serde(rename = "ShieldingRequested")]
    pub shielding_requested: Option<bool>,

/// 
    #[serde(rename = "TpmEnabled")]
    pub tpm_enabled: Option<bool>,

/// 
    #[serde(rename = "VirtualizationBasedSecurityOptOut")]
    pub virtualization_based_security_opt_out: Option<bool>,
}

impl Msvm_SecuritySettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            app_container_launch_opt_out: None,
            bind_to_host_tpm: None,
            data_protection_requested: None,
            encrypt_state_and_vm_migration_traffic: None,
            ksd_enabled: None,
            shielding_requested: None,
            tpm_enabled: None,
            virtualization_based_security_opt_out: None,
        }
    }


    /// Sets the value of AppContainerLaunchOptOut
    pub fn set_app_container_launch_opt_out(&mut self, value: bool) {
        self.app_container_launch_opt_out = Some(value);
    }

    /// Gets the value of AppContainerLaunchOptOut
    pub fn get_app_container_launch_opt_out(&self) -> Option<&bool> {
        self.app_container_launch_opt_out.as_ref()
    }

    /// Sets the value of BindToHostTpm
    pub fn set_bind_to_host_tpm(&mut self, value: bool) {
        self.bind_to_host_tpm = Some(value);
    }

    /// Gets the value of BindToHostTpm
    pub fn get_bind_to_host_tpm(&self) -> Option<&bool> {
        self.bind_to_host_tpm.as_ref()
    }

    /// Sets the value of DataProtectionRequested
    pub fn set_data_protection_requested(&mut self, value: bool) {
        self.data_protection_requested = Some(value);
    }

    /// Gets the value of DataProtectionRequested
    pub fn get_data_protection_requested(&self) -> Option<&bool> {
        self.data_protection_requested.as_ref()
    }

    /// Sets the value of EncryptStateAndVmMigrationTraffic
    pub fn set_encrypt_state_and_vm_migration_traffic(&mut self, value: bool) {
        self.encrypt_state_and_vm_migration_traffic = Some(value);
    }

    /// Gets the value of EncryptStateAndVmMigrationTraffic
    pub fn get_encrypt_state_and_vm_migration_traffic(&self) -> Option<&bool> {
        self.encrypt_state_and_vm_migration_traffic.as_ref()
    }

    /// Sets the value of KsdEnabled
    pub fn set_ksd_enabled(&mut self, value: bool) {
        self.ksd_enabled = Some(value);
    }

    /// Gets the value of KsdEnabled
    pub fn get_ksd_enabled(&self) -> Option<&bool> {
        self.ksd_enabled.as_ref()
    }

    /// Sets the value of ShieldingRequested
    pub fn set_shielding_requested(&mut self, value: bool) {
        self.shielding_requested = Some(value);
    }

    /// Gets the value of ShieldingRequested
    pub fn get_shielding_requested(&self) -> Option<&bool> {
        self.shielding_requested.as_ref()
    }

    /// Sets the value of TpmEnabled
    pub fn set_tpm_enabled(&mut self, value: bool) {
        self.tpm_enabled = Some(value);
    }

    /// Gets the value of TpmEnabled
    pub fn get_tpm_enabled(&self) -> Option<&bool> {
        self.tpm_enabled.as_ref()
    }

    /// Sets the value of VirtualizationBasedSecurityOptOut
    pub fn set_virtualization_based_security_opt_out(&mut self, value: bool) {
        self.virtualization_based_security_opt_out = Some(value);
    }

    /// Gets the value of VirtualizationBasedSecurityOptOut
    pub fn get_virtualization_based_security_opt_out(&self) -> Option<&bool> {
        self.virtualization_based_security_opt_out.as_ref()
    }
}

impl Msvm_SecuritySettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

