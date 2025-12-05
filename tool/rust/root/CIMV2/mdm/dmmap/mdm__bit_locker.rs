// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_BitLocker struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_BitLocker {

/// 
    #[serde(rename = "AllowStandardUserEncryption")]
    pub allow_standard_user_encryption: Option<i32>,

/// 
    #[serde(rename = "AllowWarningForOtherDiskEncryption")]
    pub allow_warning_for_other_disk_encryption: Option<i32>,

/// 
    #[serde(rename = "ConfigureRecoveryPasswordRotation")]
    pub configure_recovery_password_rotation: Option<i32>,

/// 
    #[serde(rename = "EncryptionMethodByDriveType")]
    pub encryption_method_by_drive_type: Option<String>,

/// 
    #[serde(rename = "FixedDrivesEncryptionType")]
    pub fixed_drives_encryption_type: Option<String>,

/// 
    #[serde(rename = "FixedDrivesRecoveryOptions")]
    pub fixed_drives_recovery_options: Option<String>,

/// 
    #[serde(rename = "FixedDrivesRequireEncryption")]
    pub fixed_drives_require_encryption: Option<String>,

/// 
    #[serde(rename = "IdentificationField")]
    pub identification_field: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RemovableDrivesConfigureBDE")]
    pub removable_drives_configure_bde: Option<String>,

/// 
    #[serde(rename = "RemovableDrivesEncryptionType")]
    pub removable_drives_encryption_type: Option<String>,

/// 
    #[serde(rename = "RemovableDrivesExcludedFromEncryption")]
    pub removable_drives_excluded_from_encryption: Option<String>,

/// 
    #[serde(rename = "RemovableDrivesRequireEncryption")]
    pub removable_drives_require_encryption: Option<String>,

/// 
    #[serde(rename = "RequireDeviceEncryption")]
    pub require_device_encryption: Option<i32>,

/// 
    #[serde(rename = "RequireStorageCardEncryption")]
    pub require_storage_card_encryption: Option<i32>,

/// 
    #[serde(rename = "SystemDrivesDisallowStandardUsersCanChangePIN")]
    pub system_drives_disallow_standard_users_can_change_pin: Option<String>,

/// 
    #[serde(rename = "SystemDrivesEnablePrebootInputProtectorsOnSlates")]
    pub system_drives_enable_preboot_input_protectors_on_slates: Option<String>,

/// 
    #[serde(rename = "SystemDrivesEnablePreBootPinExceptionOnDECapableDevice")]
    pub system_drives_enable_pre_boot_pin_exception_on_decapable_device: Option<String>,

/// 
    #[serde(rename = "SystemDrivesEncryptionType")]
    pub system_drives_encryption_type: Option<String>,

/// 
    #[serde(rename = "SystemDrivesEnhancedPIN")]
    pub system_drives_enhanced_pin: Option<String>,

/// 
    #[serde(rename = "SystemDrivesMinimumPINLength")]
    pub system_drives_minimum_pinlength: Option<String>,

/// 
    #[serde(rename = "SystemDrivesRecoveryMessage")]
    pub system_drives_recovery_message: Option<String>,

/// 
    #[serde(rename = "SystemDrivesRecoveryOptions")]
    pub system_drives_recovery_options: Option<String>,

/// 
    #[serde(rename = "SystemDrivesRequireStartupAuthentication")]
    pub system_drives_require_startup_authentication: Option<String>,
}

impl MDM_BitLocker {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_standard_user_encryption: None,
            allow_warning_for_other_disk_encryption: None,
            configure_recovery_password_rotation: None,
            encryption_method_by_drive_type: None,
            fixed_drives_encryption_type: None,
            fixed_drives_recovery_options: None,
            fixed_drives_require_encryption: None,
            identification_field: None,
            instance_id: None,
            parent_id: None,
            removable_drives_configure_bde: None,
            removable_drives_encryption_type: None,
            removable_drives_excluded_from_encryption: None,
            removable_drives_require_encryption: None,
            require_device_encryption: None,
            require_storage_card_encryption: None,
            system_drives_disallow_standard_users_can_change_pin: None,
            system_drives_enable_preboot_input_protectors_on_slates: None,
            system_drives_enable_pre_boot_pin_exception_on_decapable_device: None,
            system_drives_encryption_type: None,
            system_drives_enhanced_pin: None,
            system_drives_minimum_pinlength: None,
            system_drives_recovery_message: None,
            system_drives_recovery_options: None,
            system_drives_require_startup_authentication: None,
        }
    }


    /// Sets the value of AllowStandardUserEncryption
    pub fn set_allow_standard_user_encryption(&mut self, value: i32) {
        self.allow_standard_user_encryption = Some(value);
    }

    /// Gets the value of AllowStandardUserEncryption
    pub fn get_allow_standard_user_encryption(&self) -> Option<&i32> {
        self.allow_standard_user_encryption.as_ref()
    }

    /// Sets the value of AllowWarningForOtherDiskEncryption
    pub fn set_allow_warning_for_other_disk_encryption(&mut self, value: i32) {
        self.allow_warning_for_other_disk_encryption = Some(value);
    }

    /// Gets the value of AllowWarningForOtherDiskEncryption
    pub fn get_allow_warning_for_other_disk_encryption(&self) -> Option<&i32> {
        self.allow_warning_for_other_disk_encryption.as_ref()
    }

    /// Sets the value of ConfigureRecoveryPasswordRotation
    pub fn set_configure_recovery_password_rotation(&mut self, value: i32) {
        self.configure_recovery_password_rotation = Some(value);
    }

    /// Gets the value of ConfigureRecoveryPasswordRotation
    pub fn get_configure_recovery_password_rotation(&self) -> Option<&i32> {
        self.configure_recovery_password_rotation.as_ref()
    }

    /// Sets the value of EncryptionMethodByDriveType
    pub fn set_encryption_method_by_drive_type(&mut self, value: String) {
        self.encryption_method_by_drive_type = Some(value);
    }

    /// Gets the value of EncryptionMethodByDriveType
    pub fn get_encryption_method_by_drive_type(&self) -> Option<&String> {
        self.encryption_method_by_drive_type.as_ref()
    }

    /// Sets the value of FixedDrivesEncryptionType
    pub fn set_fixed_drives_encryption_type(&mut self, value: String) {
        self.fixed_drives_encryption_type = Some(value);
    }

    /// Gets the value of FixedDrivesEncryptionType
    pub fn get_fixed_drives_encryption_type(&self) -> Option<&String> {
        self.fixed_drives_encryption_type.as_ref()
    }

    /// Sets the value of FixedDrivesRecoveryOptions
    pub fn set_fixed_drives_recovery_options(&mut self, value: String) {
        self.fixed_drives_recovery_options = Some(value);
    }

    /// Gets the value of FixedDrivesRecoveryOptions
    pub fn get_fixed_drives_recovery_options(&self) -> Option<&String> {
        self.fixed_drives_recovery_options.as_ref()
    }

    /// Sets the value of FixedDrivesRequireEncryption
    pub fn set_fixed_drives_require_encryption(&mut self, value: String) {
        self.fixed_drives_require_encryption = Some(value);
    }

    /// Gets the value of FixedDrivesRequireEncryption
    pub fn get_fixed_drives_require_encryption(&self) -> Option<&String> {
        self.fixed_drives_require_encryption.as_ref()
    }

    /// Sets the value of IdentificationField
    pub fn set_identification_field(&mut self, value: String) {
        self.identification_field = Some(value);
    }

    /// Gets the value of IdentificationField
    pub fn get_identification_field(&self) -> Option<&String> {
        self.identification_field.as_ref()
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

    /// Sets the value of RemovableDrivesConfigureBDE
    pub fn set_removable_drives_configure_bde(&mut self, value: String) {
        self.removable_drives_configure_bde = Some(value);
    }

    /// Gets the value of RemovableDrivesConfigureBDE
    pub fn get_removable_drives_configure_bde(&self) -> Option<&String> {
        self.removable_drives_configure_bde.as_ref()
    }

    /// Sets the value of RemovableDrivesEncryptionType
    pub fn set_removable_drives_encryption_type(&mut self, value: String) {
        self.removable_drives_encryption_type = Some(value);
    }

    /// Gets the value of RemovableDrivesEncryptionType
    pub fn get_removable_drives_encryption_type(&self) -> Option<&String> {
        self.removable_drives_encryption_type.as_ref()
    }

    /// Sets the value of RemovableDrivesExcludedFromEncryption
    pub fn set_removable_drives_excluded_from_encryption(&mut self, value: String) {
        self.removable_drives_excluded_from_encryption = Some(value);
    }

    /// Gets the value of RemovableDrivesExcludedFromEncryption
    pub fn get_removable_drives_excluded_from_encryption(&self) -> Option<&String> {
        self.removable_drives_excluded_from_encryption.as_ref()
    }

    /// Sets the value of RemovableDrivesRequireEncryption
    pub fn set_removable_drives_require_encryption(&mut self, value: String) {
        self.removable_drives_require_encryption = Some(value);
    }

    /// Gets the value of RemovableDrivesRequireEncryption
    pub fn get_removable_drives_require_encryption(&self) -> Option<&String> {
        self.removable_drives_require_encryption.as_ref()
    }

    /// Sets the value of RequireDeviceEncryption
    pub fn set_require_device_encryption(&mut self, value: i32) {
        self.require_device_encryption = Some(value);
    }

    /// Gets the value of RequireDeviceEncryption
    pub fn get_require_device_encryption(&self) -> Option<&i32> {
        self.require_device_encryption.as_ref()
    }

    /// Sets the value of RequireStorageCardEncryption
    pub fn set_require_storage_card_encryption(&mut self, value: i32) {
        self.require_storage_card_encryption = Some(value);
    }

    /// Gets the value of RequireStorageCardEncryption
    pub fn get_require_storage_card_encryption(&self) -> Option<&i32> {
        self.require_storage_card_encryption.as_ref()
    }

    /// Sets the value of SystemDrivesDisallowStandardUsersCanChangePIN
    pub fn set_system_drives_disallow_standard_users_can_change_pin(&mut self, value: String) {
        self.system_drives_disallow_standard_users_can_change_pin = Some(value);
    }

    /// Gets the value of SystemDrivesDisallowStandardUsersCanChangePIN
    pub fn get_system_drives_disallow_standard_users_can_change_pin(&self) -> Option<&String> {
        self.system_drives_disallow_standard_users_can_change_pin.as_ref()
    }

    /// Sets the value of SystemDrivesEnablePrebootInputProtectorsOnSlates
    pub fn set_system_drives_enable_preboot_input_protectors_on_slates(&mut self, value: String) {
        self.system_drives_enable_preboot_input_protectors_on_slates = Some(value);
    }

    /// Gets the value of SystemDrivesEnablePrebootInputProtectorsOnSlates
    pub fn get_system_drives_enable_preboot_input_protectors_on_slates(&self) -> Option<&String> {
        self.system_drives_enable_preboot_input_protectors_on_slates.as_ref()
    }

    /// Sets the value of SystemDrivesEnablePreBootPinExceptionOnDECapableDevice
    pub fn set_system_drives_enable_pre_boot_pin_exception_on_decapable_device(&mut self, value: String) {
        self.system_drives_enable_pre_boot_pin_exception_on_decapable_device = Some(value);
    }

    /// Gets the value of SystemDrivesEnablePreBootPinExceptionOnDECapableDevice
    pub fn get_system_drives_enable_pre_boot_pin_exception_on_decapable_device(&self) -> Option<&String> {
        self.system_drives_enable_pre_boot_pin_exception_on_decapable_device.as_ref()
    }

    /// Sets the value of SystemDrivesEncryptionType
    pub fn set_system_drives_encryption_type(&mut self, value: String) {
        self.system_drives_encryption_type = Some(value);
    }

    /// Gets the value of SystemDrivesEncryptionType
    pub fn get_system_drives_encryption_type(&self) -> Option<&String> {
        self.system_drives_encryption_type.as_ref()
    }

    /// Sets the value of SystemDrivesEnhancedPIN
    pub fn set_system_drives_enhanced_pin(&mut self, value: String) {
        self.system_drives_enhanced_pin = Some(value);
    }

    /// Gets the value of SystemDrivesEnhancedPIN
    pub fn get_system_drives_enhanced_pin(&self) -> Option<&String> {
        self.system_drives_enhanced_pin.as_ref()
    }

    /// Sets the value of SystemDrivesMinimumPINLength
    pub fn set_system_drives_minimum_pinlength(&mut self, value: String) {
        self.system_drives_minimum_pinlength = Some(value);
    }

    /// Gets the value of SystemDrivesMinimumPINLength
    pub fn get_system_drives_minimum_pinlength(&self) -> Option<&String> {
        self.system_drives_minimum_pinlength.as_ref()
    }

    /// Sets the value of SystemDrivesRecoveryMessage
    pub fn set_system_drives_recovery_message(&mut self, value: String) {
        self.system_drives_recovery_message = Some(value);
    }

    /// Gets the value of SystemDrivesRecoveryMessage
    pub fn get_system_drives_recovery_message(&self) -> Option<&String> {
        self.system_drives_recovery_message.as_ref()
    }

    /// Sets the value of SystemDrivesRecoveryOptions
    pub fn set_system_drives_recovery_options(&mut self, value: String) {
        self.system_drives_recovery_options = Some(value);
    }

    /// Gets the value of SystemDrivesRecoveryOptions
    pub fn get_system_drives_recovery_options(&self) -> Option<&String> {
        self.system_drives_recovery_options.as_ref()
    }

    /// Sets the value of SystemDrivesRequireStartupAuthentication
    pub fn set_system_drives_require_startup_authentication(&mut self, value: String) {
        self.system_drives_require_startup_authentication = Some(value);
    }

    /// Gets the value of SystemDrivesRequireStartupAuthentication
    pub fn get_system_drives_require_startup_authentication(&self) -> Option<&String> {
        self.system_drives_require_startup_authentication.as_ref()
    }

/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn rotate_recovery_passwords_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("RotateRecoveryPasswordsMethod", &args)

    }

}

