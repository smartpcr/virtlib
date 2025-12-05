// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Security02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Security02 {

/// 
    #[serde(rename = "AllowAddProvisioningPackage")]
    pub allow_add_provisioning_package: Option<i32>,

/// 
    #[serde(rename = "AllowRemoveProvisioningPackage")]
    pub allow_remove_provisioning_package: Option<i32>,

/// 
    #[serde(rename = "ClearTPMIfNotReady")]
    pub clear_tpmif_not_ready: Option<i32>,

/// 
    #[serde(rename = "ConfigureWindowsPasswords")]
    pub configure_windows_passwords: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventAutomaticDeviceEncryptionForAzureADJoinedDevices")]
    pub prevent_automatic_device_encryption_for_azure_adjoined_devices: Option<i32>,

/// 
    #[serde(rename = "RecoveryEnvironmentAuthentication")]
    pub recovery_environment_authentication: Option<i32>,

/// 
    #[serde(rename = "RequireDeviceEncryption")]
    pub require_device_encryption: Option<i32>,

/// 
    #[serde(rename = "RequireProvisioningPackageSignature")]
    pub require_provisioning_package_signature: Option<i32>,

/// 
    #[serde(rename = "RequireRetrieveHealthCertificateOnBoot")]
    pub require_retrieve_health_certificate_on_boot: Option<i32>,
}

impl MDM_Policy_Config01_Security02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_add_provisioning_package: None,
            allow_remove_provisioning_package: None,
            clear_tpmif_not_ready: None,
            configure_windows_passwords: None,
            instance_id: None,
            parent_id: None,
            prevent_automatic_device_encryption_for_azure_adjoined_devices: None,
            recovery_environment_authentication: None,
            require_device_encryption: None,
            require_provisioning_package_signature: None,
            require_retrieve_health_certificate_on_boot: None,
        }
    }


    /// Sets the value of AllowAddProvisioningPackage
    pub fn set_allow_add_provisioning_package(&mut self, value: i32) {
        self.allow_add_provisioning_package = Some(value);
    }

    /// Gets the value of AllowAddProvisioningPackage
    pub fn get_allow_add_provisioning_package(&self) -> Option<&i32> {
        self.allow_add_provisioning_package.as_ref()
    }

    /// Sets the value of AllowRemoveProvisioningPackage
    pub fn set_allow_remove_provisioning_package(&mut self, value: i32) {
        self.allow_remove_provisioning_package = Some(value);
    }

    /// Gets the value of AllowRemoveProvisioningPackage
    pub fn get_allow_remove_provisioning_package(&self) -> Option<&i32> {
        self.allow_remove_provisioning_package.as_ref()
    }

    /// Sets the value of ClearTPMIfNotReady
    pub fn set_clear_tpmif_not_ready(&mut self, value: i32) {
        self.clear_tpmif_not_ready = Some(value);
    }

    /// Gets the value of ClearTPMIfNotReady
    pub fn get_clear_tpmif_not_ready(&self) -> Option<&i32> {
        self.clear_tpmif_not_ready.as_ref()
    }

    /// Sets the value of ConfigureWindowsPasswords
    pub fn set_configure_windows_passwords(&mut self, value: i32) {
        self.configure_windows_passwords = Some(value);
    }

    /// Gets the value of ConfigureWindowsPasswords
    pub fn get_configure_windows_passwords(&self) -> Option<&i32> {
        self.configure_windows_passwords.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PreventAutomaticDeviceEncryptionForAzureADJoinedDevices
    pub fn set_prevent_automatic_device_encryption_for_azure_adjoined_devices(&mut self, value: i32) {
        self.prevent_automatic_device_encryption_for_azure_adjoined_devices = Some(value);
    }

    /// Gets the value of PreventAutomaticDeviceEncryptionForAzureADJoinedDevices
    pub fn get_prevent_automatic_device_encryption_for_azure_adjoined_devices(&self) -> Option<&i32> {
        self.prevent_automatic_device_encryption_for_azure_adjoined_devices.as_ref()
    }

    /// Sets the value of RecoveryEnvironmentAuthentication
    pub fn set_recovery_environment_authentication(&mut self, value: i32) {
        self.recovery_environment_authentication = Some(value);
    }

    /// Gets the value of RecoveryEnvironmentAuthentication
    pub fn get_recovery_environment_authentication(&self) -> Option<&i32> {
        self.recovery_environment_authentication.as_ref()
    }

    /// Sets the value of RequireDeviceEncryption
    pub fn set_require_device_encryption(&mut self, value: i32) {
        self.require_device_encryption = Some(value);
    }

    /// Gets the value of RequireDeviceEncryption
    pub fn get_require_device_encryption(&self) -> Option<&i32> {
        self.require_device_encryption.as_ref()
    }

    /// Sets the value of RequireProvisioningPackageSignature
    pub fn set_require_provisioning_package_signature(&mut self, value: i32) {
        self.require_provisioning_package_signature = Some(value);
    }

    /// Gets the value of RequireProvisioningPackageSignature
    pub fn get_require_provisioning_package_signature(&self) -> Option<&i32> {
        self.require_provisioning_package_signature.as_ref()
    }

    /// Sets the value of RequireRetrieveHealthCertificateOnBoot
    pub fn set_require_retrieve_health_certificate_on_boot(&mut self, value: i32) {
        self.require_retrieve_health_certificate_on_boot = Some(value);
    }

    /// Gets the value of RequireRetrieveHealthCertificateOnBoot
    pub fn get_require_retrieve_health_certificate_on_boot(&self) -> Option<&i32> {
        self.require_retrieve_health_certificate_on_boot.as_ref()
    }
}

