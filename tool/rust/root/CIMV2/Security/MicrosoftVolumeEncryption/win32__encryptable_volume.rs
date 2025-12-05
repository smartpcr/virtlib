// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.Security.MicrosoftVolumeEncryption
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_EncryptableVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_EncryptableVolume {

/// 
    #[serde(rename = "ConversionStatus")]
    pub conversion_status: Option<u32>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "DriveLetter")]
    pub drive_letter: Option<String>,

/// 
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<u32>,

/// 
    #[serde(rename = "IsVolumeInitializedForProtection")]
    pub is_volume_initialized_for_protection: Option<bool>,

/// 
    #[serde(rename = "PersistentVolumeID")]
    pub persistent_volume_id: Option<String>,

/// 
    #[serde(rename = "ProtectionStatus")]
    pub protection_status: Option<u32>,

/// 
    #[serde(rename = "VolumeType")]
    pub volume_type: Option<EncryptableVolume_VolumeType>,
}

impl Win32_EncryptableVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            conversion_status: None,
            device_id: None,
            drive_letter: None,
            encryption_method: None,
            is_volume_initialized_for_protection: None,
            persistent_volume_id: None,
            protection_status: None,
            volume_type: None,
        }
    }


    /// Sets the value of ConversionStatus
    pub fn set_conversion_status(&mut self, value: u32) {
        self.conversion_status = Some(value);
    }

    /// Gets the value of ConversionStatus
    pub fn get_conversion_status(&self) -> Option<&u32> {
        self.conversion_status.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of DriveLetter
    pub fn set_drive_letter(&mut self, value: String) {
        self.drive_letter = Some(value);
    }

    /// Gets the value of DriveLetter
    pub fn get_drive_letter(&self) -> Option<&String> {
        self.drive_letter.as_ref()
    }

    /// Sets the value of EncryptionMethod
    pub fn set_encryption_method(&mut self, value: u32) {
        self.encryption_method = Some(value);
    }

    /// Gets the value of EncryptionMethod
    pub fn get_encryption_method(&self) -> Option<&u32> {
        self.encryption_method.as_ref()
    }

    /// Sets the value of IsVolumeInitializedForProtection
    pub fn set_is_volume_initialized_for_protection(&mut self, value: bool) {
        self.is_volume_initialized_for_protection = Some(value);
    }

    /// Gets the value of IsVolumeInitializedForProtection
    pub fn get_is_volume_initialized_for_protection(&self) -> Option<&bool> {
        self.is_volume_initialized_for_protection.as_ref()
    }

    /// Sets the value of PersistentVolumeID
    pub fn set_persistent_volume_id(&mut self, value: String) {
        self.persistent_volume_id = Some(value);
    }

    /// Gets the value of PersistentVolumeID
    pub fn get_persistent_volume_id(&self) -> Option<&String> {
        self.persistent_volume_id.as_ref()
    }

    /// Sets the value of ProtectionStatus
    pub fn set_protection_status(&mut self, value: u32) {
        self.protection_status = Some(value);
    }

    /// Gets the value of ProtectionStatus
    pub fn get_protection_status(&self) -> Option<&u32> {
        self.protection_status.as_ref()
    }

    /// Sets the value of VolumeType
    pub fn set_volume_type(&mut self, value: EncryptableVolume_VolumeType) {
        self.volume_type = Some(value);
    }

    /// Gets the value of VolumeType
    pub fn get_volume_type(&self) -> Option<&EncryptableVolume_VolumeType> {
        self.volume_type.as_ref()
    }

/// 

    /// * `precision_factor` -  (EncryptableVolume_PrecisionFactor)

    /// * `conversion_status` -  (EncryptableVolume_ConversionStatus)
    /// * `encryption_flags` -  (u32)
    /// * `encryption_percentage` -  (u32)
    /// * `return_value` -  (u32)
    /// * `wiping_percentage` -  (u32)
    /// * `wiping_status` -  (EncryptableVolume_WipingStatus)
    pub fn get_conversion_status(&self, conversion_status: &mut EncryptableVolume_ConversionStatus, encryption_percentage: &mut u32, encryption_flags: &mut u32, wiping_status: &mut EncryptableVolume_WipingStatus, wiping_percentage: &mut u32, precision_factor: Option<EncryptableVolume_PrecisionFactor>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = precision_factor {
            args.push(MethodParameter { name: "PrecisionFactor".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetConversionStatus", &args)?;
        let conversion_status = result.get_value("ConversionStatus")?;
        let encryption_flags = result.get_value("EncryptionFlags")?;
        let encryption_percentage = result.get_value("EncryptionPercentage")?;
        let wiping_percentage = result.get_value("WipingPercentage")?;
        let wiping_status = result.get_value("WipingStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `test_error` -  (u32)
    /// * `test_status` -  (EncryptableVolume_TestStatus)
    pub fn get_hardware_test_status(&self, test_status: &mut EncryptableVolume_TestStatus, test_error: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetHardwareTestStatus", &[])?;
        let test_error = result.get_value("TestError")?;
        let test_status = result.get_value("TestStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `lock_status` -  (EncryptableVolume_LockStatus)
    /// * `return_value` -  (u32)
    pub fn get_lock_status(&self, lock_status: &mut EncryptableVolume_LockStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetLockStatus", &[])?;
        let lock_status = result.get_value("LockStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `protection_status` -  (EncryptableVolume_ProtectionStatus)
    /// * `return_value` -  (u32)
    pub fn get_protection_status(&self, protection_status: &mut EncryptableVolume_ProtectionStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetProtectionStatus", &[])?;
        let protection_status = result.get_value("ProtectionStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `suspend_count` -  (u32)
    pub fn get_suspend_count(&self, suspend_count: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSuspendCount", &[])?;
        let suspend_count = result.get_value("SuspendCount")?;
        Ok(result.return_value)

    }


/// 

    /// * `hardware_encryption_status` -  (EncryptableVolume_HardwareEncryptionStatus)
    /// * `return_value` -  (u32)
    pub fn get_hardware_encryption_status(&self, hardware_encryption_status: &mut EncryptableVolume_HardwareEncryptionStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetHardwareEncryptionStatus", &[])?;
        let hardware_encryption_status = result.get_value("HardwareEncryptionStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `encryption_flags` -  (u32)
    /// * `encryption_method` -  (EncryptableVolume_EncryptionMethod)

    /// * `return_value` -  (u32)
    pub fn encrypt(&self, encryption_method: EncryptableVolume_EncryptionMethod, encryption_flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EncryptionMethod".to_string(), value: encryption_method.into() });
        args.push(MethodParameter { name: "EncryptionFlags".to_string(), value: encryption_flags.into() });
        self.invoke_method("Encrypt", &args)

    }


/// 

    /// * `encryption_flags` -  (u32)
    /// * `encryption_method` -  (EncryptableVolume_EncryptionMethod)

    /// * `return_value` -  (u32)
    pub fn encrypt_after_hardware_test(&self, encryption_method: EncryptableVolume_EncryptionMethod, encryption_flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EncryptionMethod".to_string(), value: encryption_method.into() });
        args.push(MethodParameter { name: "EncryptionFlags".to_string(), value: encryption_flags.into() });
        self.invoke_method("EncryptAfterHardwareTest", &args)

    }


/// 

    /// * `encryption_method` -  (EncryptableVolume_EncryptionMethod)
    /// * `encryption_method_flags` -  (u32)
    /// * `return_value` -  (u32)
    /// * `self_encryption_drive_encryption_method` -  (String)
    pub fn get_encryption_method(&self, encryption_method: &mut EncryptableVolume_EncryptionMethod, self_encryption_drive_encryption_method: &mut String, encryption_method_flags: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetEncryptionMethod", &[])?;
        let encryption_method = result.get_value("EncryptionMethod")?;
        let encryption_method_flags = result.get_value("EncryptionMethodFlags")?;
        let self_encryption_drive_encryption_method = result.get_value("SelfEncryptionDriveEncryptionMethod")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn decrypt(&self) -> Result<(), WmiError> {
        self.invoke_method("Decrypt", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn pause_conversion(&self) -> Result<(), WmiError> {
        self.invoke_method("PauseConversion", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn resume_conversion(&self) -> Result<(), WmiError> {
        self.invoke_method("ResumeConversion", &[])

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `platform_validation_profile` -  (u8[])

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_tpm(&self, friendly_name: &String, platform_validation_profile: &Vec<u8>, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PlatformValidationProfile".to_string(), value: platform_validation_profile.into() });

        let result = self.invoke_method("ProtectKeyWithTPM", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `external_key` -  (u8[])
    /// * `friendly_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_external_key(&self, friendly_name: &String, external_key: &Vec<u8>, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "ExternalKey".to_string(), value: external_key.into() });

        let result = self.invoke_method("ProtectKeyWithExternalKey", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `numerical_password` -  (String)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_numerical_password(&self, friendly_name: &String, numerical_password: &String, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "NumericalPassword".to_string(), value: numerical_password.into() });

        let result = self.invoke_method("ProtectKeyWithNumericalPassword", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `pin` -  (String)
    /// * `platform_validation_profile` -  (u8[])

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_tpmand_pin(&self, friendly_name: &String, platform_validation_profile: &Vec<u8>, pin: &String, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PlatformValidationProfile".to_string(), value: platform_validation_profile.into() });
        args.push(MethodParameter { name: "PIN".to_string(), value: pin.into() });

        let result = self.invoke_method("ProtectKeyWithTPMAndPIN", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `external_key` -  (u8[])
    /// * `friendly_name` -  (String)
    /// * `platform_validation_profile` -  (u8[])

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_tpmand_startup_key(&self, friendly_name: &String, platform_validation_profile: &Vec<u8>, external_key: &Vec<u8>, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PlatformValidationProfile".to_string(), value: platform_validation_profile.into() });
        args.push(MethodParameter { name: "ExternalKey".to_string(), value: external_key.into() });

        let result = self.invoke_method("ProtectKeyWithTPMAndStartupKey", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `external_key` -  (u8[])
    /// * `friendly_name` -  (String)
    /// * `pin` -  (String)
    /// * `platform_validation_profile` -  (u8[])

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_tpmand_pinand_startup_key(&self, friendly_name: &String, platform_validation_profile: &Vec<u8>, pin: &String, external_key: &Vec<u8>, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PlatformValidationProfile".to_string(), value: platform_validation_profile.into() });
        args.push(MethodParameter { name: "PIN".to_string(), value: pin.into() });
        args.push(MethodParameter { name: "ExternalKey".to_string(), value: external_key.into() });

        let result = self.invoke_method("ProtectKeyWithTPMAndPINAndStartupKey", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `path_with_file_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_certificate_file(&self, friendly_name: &String, path_with_file_name: &String, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PathWithFileName".to_string(), value: path_with_file_name.into() });

        let result = self.invoke_method("ProtectKeyWithCertificateFile", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `cert_thumbprint` -  (String)
    /// * `friendly_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_certificate_thumbprint(&self, friendly_name: &String, cert_thumbprint: &String, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "CertThumbprint".to_string(), value: cert_thumbprint.into() });

        let result = self.invoke_method("ProtectKeyWithCertificateThumbprint", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `pass_phrase` -  (String)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_pass_phrase(&self, friendly_name: &String, pass_phrase: &String, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PassPhrase".to_string(), value: pass_phrase.into() });

        let result = self.invoke_method("ProtectKeyWithPassPhrase", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `friendly_name` -  (String)
    /// * `sid_string` -  (String)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_ad_sid(&self, friendly_name: &String, sid_string: &String, flags: u32, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "SidString".to_string(), value: sid_string.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("ProtectKeyWithAdSid", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn enable_auto_unlock(&self, volume_key_protector_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        self.invoke_method("EnableAutoUnlock", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable_auto_unlock(&self) -> Result<(), WmiError> {
        self.invoke_method("DisableAutoUnlock", &[])

    }


/// 

    /// * `is_auto_unlock_enabled` -  (bool)
    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn is_auto_unlock_enabled(&self, is_auto_unlock_enabled: &mut bool, volume_key_protector_id: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("IsAutoUnlockEnabled", &[])?;
        let is_auto_unlock_enabled = result.get_value("IsAutoUnlockEnabled")?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn clear_all_auto_unlock_keys(&self) -> Result<(), WmiError> {
        self.invoke_method("ClearAllAutoUnlockKeys", &[])

    }


/// 

    /// * `is_auto_unlock_key_stored` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_auto_unlock_key_stored(&self, is_auto_unlock_key_stored: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsAutoUnlockKeyStored", &[])?;
        let is_auto_unlock_key_stored = result.get_value("IsAutoUnlockKeyStored")?;
        Ok(result.return_value)

    }


/// 

    /// * `path` -  (String)
    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn save_external_key_to_file(&self, volume_key_protector_id: &String, path: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        self.invoke_method("SaveExternalKeyToFile", &args)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `file_name` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_external_key_file_name(&self, volume_key_protector_id: &String, file_name: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetExternalKeyFileName", &args)?;
        let file_name = result.get_value("FileName")?;
        Ok(result.return_value)

    }


/// 

    /// * `path_with_file_name` -  (String)

    /// * `external_key` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_external_key_from_file(&self, path_with_file_name: &String, external_key: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathWithFileName".to_string(), value: path_with_file_name.into() });

        let result = self.invoke_method("GetExternalKeyFromFile", &args)?;
        let external_key = result.get_value("ExternalKey")?;
        Ok(result.return_value)

    }


/// 

    /// * `key_protector_type` -  (EncryptableVolume_KeyProtectorType)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String[])
    pub fn get_key_protectors(&self, key_protector_type: EncryptableVolume_KeyProtectorType, volume_key_protector_id: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyProtectorType".to_string(), value: key_protector_type.into() });

        let result = self.invoke_method("GetKeyProtectors", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `disable_count` -  (u32)

    /// * `return_value` -  (u32)
    pub fn disable_key_protectors(&self, disable_count: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisableCount".to_string(), value: disable_count.into() });
        self.invoke_method("DisableKeyProtectors", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn enable_key_protectors(&self) -> Result<(), WmiError> {
        self.invoke_method("EnableKeyProtectors", &[])

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete_key_protector(&self, volume_key_protector_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        self.invoke_method("DeleteKeyProtector", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn delete_key_protectors(&self) -> Result<(), WmiError> {
        self.invoke_method("DeleteKeyProtectors", &[])

    }


/// 

    /// * `numerical_password` -  (String)

    /// * `return_value` -  (u32)
    pub fn unlock_with_numerical_password(&self, numerical_password: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NumericalPassword".to_string(), value: numerical_password.into() });
        self.invoke_method("UnlockWithNumericalPassword", &args)

    }


/// 

    /// * `external_key` -  (u8[])

    /// * `return_value` -  (u32)
    pub fn unlock_with_external_key(&self, external_key: &Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ExternalKey".to_string(), value: external_key.into() });
        self.invoke_method("UnlockWithExternalKey", &args)

    }


/// 

    /// * `path_with_file_name` -  (String)
    /// * `pin` -  (String)

    /// * `return_value` -  (u32)
    pub fn unlock_with_certificate_file(&self, path_with_file_name: &String, pin: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathWithFileName".to_string(), value: path_with_file_name.into() });
        args.push(MethodParameter { name: "Pin".to_string(), value: pin.into() });
        self.invoke_method("UnlockWithCertificateFile", &args)

    }


/// 

    /// * `cert_thumbprint` -  (String)
    /// * `pin` -  (String)

    /// * `return_value` -  (u32)
    pub fn unlock_with_certificate_thumbprint(&self, cert_thumbprint: &String, pin: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CertThumbprint".to_string(), value: cert_thumbprint.into() });
        args.push(MethodParameter { name: "Pin".to_string(), value: pin.into() });
        self.invoke_method("UnlockWithCertificateThumbprint", &args)

    }


/// 

    /// * `pass_phrase` -  (String)

    /// * `return_value` -  (u32)
    pub fn unlock_with_pass_phrase(&self, pass_phrase: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PassPhrase".to_string(), value: pass_phrase.into() });
        self.invoke_method("UnlockWithPassPhrase", &args)

    }


/// 

    /// * `sid_string` -  (String)

    /// * `return_value` -  (u32)
    pub fn unlock_with_ad_sid(&self, sid_string: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = sid_string {
            args.push(MethodParameter { name: "SidString".to_string(), value: val.into() });
        }
        self.invoke_method("UnlockWithAdSid", &args)

    }


/// 

    /// * `force_dismount` -  (bool)

    /// * `return_value` -  (u32)
    pub fn lock(&self, force_dismount: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ForceDismount".to_string(), value: force_dismount.into() });
        self.invoke_method("Lock", &args)

    }


/// 

    /// * `numerical_password` -  (String)

    /// * `is_numerical_password_valid` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_numerical_password_valid(&self, numerical_password: &String, is_numerical_password_valid: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NumericalPassword".to_string(), value: numerical_password.into() });

        let result = self.invoke_method("IsNumericalPasswordValid", &args)?;
        let is_numerical_password_valid = result.get_value("IsNumericalPasswordValid")?;
        Ok(result.return_value)

    }


/// 

    /// * `key_protector_type` -  (EncryptableVolume_KeyProtectorType)

    /// * `is_key_protector_available` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_key_protector_available(&self, key_protector_type: EncryptableVolume_KeyProtectorType, is_key_protector_available: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyProtectorType".to_string(), value: key_protector_type.into() });

        let result = self.invoke_method("IsKeyProtectorAvailable", &args)?;
        let is_key_protector_available = result.get_value("IsKeyProtectorAvailable")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `key_protector_type` -  (EncryptableVolume_KeyProtectorType)
    /// * `return_value` -  (u32)
    pub fn get_key_protector_type(&self, volume_key_protector_id: &String, key_protector_type: &mut EncryptableVolume_KeyProtectorType) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyProtectorType", &args)?;
        let key_protector_type = result.get_value("KeyProtectorType")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `friendly_name` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_key_protector_friendly_name(&self, volume_key_protector_id: &String, friendly_name: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyProtectorFriendlyName", &args)?;
        let friendly_name = result.get_value("FriendlyName")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `external_key` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_key_protector_external_key(&self, volume_key_protector_id: &String, external_key: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyProtectorExternalKey", &args)?;
        let external_key = result.get_value("ExternalKey")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `numerical_password` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_key_protector_numerical_password(&self, volume_key_protector_id: &String, numerical_password: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyProtectorNumericalPassword", &args)?;
        let numerical_password = result.get_value("NumericalPassword")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `platform_validation_profile` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_key_protector_platform_validation_profile(&self, volume_key_protector_id: &String, platform_validation_profile: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyProtectorPlatformValidationProfile", &args)?;
        let platform_validation_profile = result.get_value("PlatformValidationProfile")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `cert_thumbprint` -  (String)
    /// * `cert_type` -  (u32)
    /// * `public_key` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_key_protector_certificate(&self, volume_key_protector_id: &String, public_key: &mut Vec<u8>, cert_thumbprint: &mut String, cert_type: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyProtectorCertificate", &args)?;
        let cert_thumbprint = result.get_value("CertThumbprint")?;
        let cert_type = result.get_value("CertType")?;
        let public_key = result.get_value("PublicKey")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `flags` -  (u32)
    /// * `return_value` -  (u32)
    /// * `sid_string` -  (String)
    pub fn get_key_protector_ad_sid_information(&self, volume_key_protector_id: &String, sid_string: &mut String, flags: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyProtectorAdSidInformation", &args)?;
        let flags = result.get_value("Flags")?;
        let sid_string = result.get_value("SidString")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `key_package` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_key_package(&self, volume_key_protector_id: &String, key_package: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetKeyPackage", &args)?;
        let key_package = result.get_value("KeyPackage")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn backup_recovery_information_to_active_directory(&self, volume_key_protector_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        self.invoke_method("BackupRecoveryInformationToActiveDirectory", &args)

    }


/// 

    /// * `new_pass_phrase` -  (String)
    /// * `volume_key_protector_id` -  (String)

    /// * `new_protector_id` -  (String)
    /// * `return_value` -  (u32)
    pub fn change_pass_phrase(&self, volume_key_protector_id: &String, new_pass_phrase: &String, new_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        args.push(MethodParameter { name: "NewPassPhrase".to_string(), value: new_pass_phrase.into() });

        let result = self.invoke_method("ChangePassPhrase", &args)?;
        let new_protector_id = result.get_value("NewProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `new_pin` -  (String)
    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn change_pin(&self, volume_key_protector_id: &String, new_pin: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        args.push(MethodParameter { name: "NewPIN".to_string(), value: new_pin.into() });
        self.invoke_method("ChangePIN", &args)

    }


/// 

    /// * `external_key` -  (u8[])
    /// * `volume_key_protector_id` -  (String)

    /// * `new_volume_key_protector_id` -  (String)
    /// * `return_value` -  (u32)
    pub fn change_external_key(&self, volume_key_protector_id: &String, external_key: &Vec<u8>, new_volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        args.push(MethodParameter { name: "ExternalKey".to_string(), value: external_key.into() });

        let result = self.invoke_method("ChangeExternalKey", &args)?;
        let new_volume_key_protector_id = result.get_value("NewVolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `discovery_volume_type` -  (String)
    /// * `force_encryption_type` -  (EncryptableVolume_ForceEncryptionType)

    /// * `return_value` -  (u32)
    pub fn prepare_volume(&self, discovery_volume_type: &String, force_encryption_type: EncryptableVolume_ForceEncryptionType) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiscoveryVolumeType".to_string(), value: discovery_volume_type.into() });
        args.push(MethodParameter { name: "ForceEncryptionType".to_string(), value: force_encryption_type.into() });
        self.invoke_method("PrepareVolume", &args)

    }


/// 

    /// * `discovery_volume_type` -  (String)
    /// * `initialization_flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn prepare_volume_ex(&self, discovery_volume_type: &String, initialization_flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiscoveryVolumeType".to_string(), value: discovery_volume_type.into() });
        args.push(MethodParameter { name: "InitializationFlags".to_string(), value: initialization_flags.into() });
        self.invoke_method("PrepareVolumeEx", &args)

    }


/// 

    /// * `identification_field` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_identification_field(&self, identification_field: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetIdentificationField", &[])?;
        let identification_field = result.get_value("IdentificationField")?;
        Ok(result.return_value)

    }


/// 

    /// * `identification_field` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_identification_field(&self, identification_field: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IdentificationField".to_string(), value: identification_field.into() });
        self.invoke_method("SetIdentificationField", &args)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `version` -  (EncryptableVolume_Version)
    pub fn get_version(&self, version: &mut EncryptableVolume_Version) -> Result<(), WmiError> {

        let result = self.invoke_method("GetVersion", &[])?;
        let version = result.get_value("Version")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn upgrade_volume(&self) -> Result<(), WmiError> {
        self.invoke_method("UpgradeVolume", &[])

    }


/// 

    /// * `cert_thumbprint` -  (String[])
    /// * `return_value` -  (u32)
    pub fn find_valid_certificates(&self, cert_thumbprint: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("FindValidCertificates", &[])?;
        let cert_thumbprint = result.get_value("CertThumbprint")?;
        Ok(result.return_value)

    }


/// 

    /// * `binding_state` -  (EncryptableVolume_BindingState)
    /// * `return_value` -  (u32)
    pub fn get_secure_boot_binding_state(&self, binding_state: &mut EncryptableVolume_BindingState) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecureBootBindingState", &[])?;
        let binding_state = result.get_value("BindingState")?;
        Ok(result.return_value)

    }


/// 

    /// * `cert_thumbprint` -  (String)
    /// * `friendly_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `volume_key_protector_id` -  (String)
    pub fn protect_key_with_network_certificate(&self, friendly_name: &String, cert_thumbprint: &String, volume_key_protector_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "CertThumbprint".to_string(), value: cert_thumbprint.into() });

        let result = self.invoke_method("ProtectKeyWithNetworkCertificate", &args)?;
        let volume_key_protector_id = result.get_value("VolumeKeyProtectorID")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn backup_recovery_information_to_cloud_domain(&self, volume_key_protector_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        self.invoke_method("BackupRecoveryInformationToCloudDomain", &args)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn backup_recovery_information_to_microsoft_account(&self, volume_key_protector_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        self.invoke_method("BackupRecoveryInformationToMicrosoftAccount", &args)

    }


/// 

    /// * `local_ipaddress` -  (String)
    /// * `server_ipaddresses` -  (String[])

    /// * `return_value` -  (u32)
    pub fn unlock_with_network_server_key(&self, server_ipaddresses: &Vec<String>, local_ipaddress: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerIPAddresses".to_string(), value: server_ipaddresses.into() });
        args.push(MethodParameter { name: "LocalIPAddress".to_string(), value: local_ipaddress.into() });
        self.invoke_method("UnlockWithNetworkServerKey", &args)

    }


/// 

    /// * `backup_info_type` -  (u32)
    /// * `volume_key_protector_id` -  (String)

    /// * `backup_accounts` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_numerical_password_backup_accounts(&self, volume_key_protector_id: &String, backup_info_type: u32, backup_accounts: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });
        args.push(MethodParameter { name: "BackupInfoType".to_string(), value: backup_info_type.into() });

        let result = self.invoke_method("GetNumericalPasswordBackupAccounts", &args)?;
        let backup_accounts = result.get_value("BackupAccounts")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `backup_info_type` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_numerical_password_backup_type(&self, volume_key_protector_id: &String, backup_info_type: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetNumericalPasswordBackupType", &args)?;
        let backup_info_type = result.get_value("BackupInfoType")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_key_protector_id` -  (String)

    /// * `return_value` -  (u32)
    /// * `tpm_protector_version` -  (EncryptableVolume_TpmProtectorVersion)
    pub fn get_tpm_protector_version(&self, volume_key_protector_id: &String, tpm_protector_version: &mut EncryptableVolume_TpmProtectorVersion) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeKeyProtectorID".to_string(), value: volume_key_protector_id.into() });

        let result = self.invoke_method("GetTpmProtectorVersion", &args)?;
        let tpm_protector_version = result.get_value("TpmProtectorVersion")?;
        Ok(result.return_value)

    }


/// 

    /// * `volume_tpm_key_protector_id` -  (String)

    /// * `contains_secure_boot_binding` -  (bool)
    /// * `return_value` -  (u32)
    pub fn tpm_protector_contains_secure_boot_binding(&self, volume_tpm_key_protector_id: &String, contains_secure_boot_binding: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VolumeTpmKeyProtectorID".to_string(), value: volume_tpm_key_protector_id.into() });

        let result = self.invoke_method("TpmProtectorContainsSecureBootBinding", &args)?;
        let contains_secure_boot_binding = result.get_value("ContainsSecureBootBinding")?;
        Ok(result.return_value)

    }

}

