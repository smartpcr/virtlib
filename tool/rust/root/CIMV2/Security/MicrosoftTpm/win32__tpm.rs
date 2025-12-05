// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.Security.MicrosoftTpm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Tpm struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Tpm {

/// 
    #[serde(rename = "IsActivated_InitialValue")]
    pub is_activated__initial_value: Option<bool>,

/// 
    #[serde(rename = "IsEnabled_InitialValue")]
    pub is_enabled__initial_value: Option<bool>,

/// 
    #[serde(rename = "IsOwned_InitialValue")]
    pub is_owned__initial_value: Option<bool>,

/// 
    #[serde(rename = "ManufacturerId")]
    pub manufacturer_id: Option<u32>,

/// 
    #[serde(rename = "ManufacturerIdTxt")]
    pub manufacturer_id_txt: Option<String>,

/// 
    #[serde(rename = "ManufacturerVersion")]
    pub manufacturer_version: Option<String>,

/// 
    #[serde(rename = "ManufacturerVersionFull20")]
    pub manufacturer_version_full20: Option<String>,

/// 
    #[serde(rename = "ManufacturerVersionInfo")]
    pub manufacturer_version_info: Option<String>,

/// 
    #[serde(rename = "PhysicalPresenceVersionInfo")]
    pub physical_presence_version_info: Option<String>,

/// 
    #[serde(rename = "SpecVersion")]
    pub spec_version: Option<String>,
}

impl Win32_Tpm {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            is_activated__initial_value: None,
            is_enabled__initial_value: None,
            is_owned__initial_value: None,
            manufacturer_id: None,
            manufacturer_id_txt: None,
            manufacturer_version: None,
            manufacturer_version_full20: None,
            manufacturer_version_info: None,
            physical_presence_version_info: None,
            spec_version: None,
        }
    }


    /// Sets the value of IsActivated_InitialValue
    pub fn set_is_activated__initial_value(&mut self, value: bool) {
        self.is_activated__initial_value = Some(value);
    }

    /// Gets the value of IsActivated_InitialValue
    pub fn get_is_activated__initial_value(&self) -> Option<&bool> {
        self.is_activated__initial_value.as_ref()
    }

    /// Sets the value of IsEnabled_InitialValue
    pub fn set_is_enabled__initial_value(&mut self, value: bool) {
        self.is_enabled__initial_value = Some(value);
    }

    /// Gets the value of IsEnabled_InitialValue
    pub fn get_is_enabled__initial_value(&self) -> Option<&bool> {
        self.is_enabled__initial_value.as_ref()
    }

    /// Sets the value of IsOwned_InitialValue
    pub fn set_is_owned__initial_value(&mut self, value: bool) {
        self.is_owned__initial_value = Some(value);
    }

    /// Gets the value of IsOwned_InitialValue
    pub fn get_is_owned__initial_value(&self) -> Option<&bool> {
        self.is_owned__initial_value.as_ref()
    }

    /// Sets the value of ManufacturerId
    pub fn set_manufacturer_id(&mut self, value: u32) {
        self.manufacturer_id = Some(value);
    }

    /// Gets the value of ManufacturerId
    pub fn get_manufacturer_id(&self) -> Option<&u32> {
        self.manufacturer_id.as_ref()
    }

    /// Sets the value of ManufacturerIdTxt
    pub fn set_manufacturer_id_txt(&mut self, value: String) {
        self.manufacturer_id_txt = Some(value);
    }

    /// Gets the value of ManufacturerIdTxt
    pub fn get_manufacturer_id_txt(&self) -> Option<&String> {
        self.manufacturer_id_txt.as_ref()
    }

    /// Sets the value of ManufacturerVersion
    pub fn set_manufacturer_version(&mut self, value: String) {
        self.manufacturer_version = Some(value);
    }

    /// Gets the value of ManufacturerVersion
    pub fn get_manufacturer_version(&self) -> Option<&String> {
        self.manufacturer_version.as_ref()
    }

    /// Sets the value of ManufacturerVersionFull20
    pub fn set_manufacturer_version_full20(&mut self, value: String) {
        self.manufacturer_version_full20 = Some(value);
    }

    /// Gets the value of ManufacturerVersionFull20
    pub fn get_manufacturer_version_full20(&self) -> Option<&String> {
        self.manufacturer_version_full20.as_ref()
    }

    /// Sets the value of ManufacturerVersionInfo
    pub fn set_manufacturer_version_info(&mut self, value: String) {
        self.manufacturer_version_info = Some(value);
    }

    /// Gets the value of ManufacturerVersionInfo
    pub fn get_manufacturer_version_info(&self) -> Option<&String> {
        self.manufacturer_version_info.as_ref()
    }

    /// Sets the value of PhysicalPresenceVersionInfo
    pub fn set_physical_presence_version_info(&mut self, value: String) {
        self.physical_presence_version_info = Some(value);
    }

    /// Gets the value of PhysicalPresenceVersionInfo
    pub fn get_physical_presence_version_info(&self) -> Option<&String> {
        self.physical_presence_version_info.as_ref()
    }

    /// Sets the value of SpecVersion
    pub fn set_spec_version(&mut self, value: String) {
        self.spec_version = Some(value);
    }

    /// Gets the value of SpecVersion
    pub fn get_spec_version(&self) -> Option<&String> {
        self.spec_version.as_ref()
    }

/// 

    /// * `is_enabled` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_enabled(&self, is_enabled: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsEnabled", &[])?;
        let is_enabled = result.get_value("IsEnabled")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_owned` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_owned(&self, is_owned: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsOwned", &[])?;
        let is_owned = result.get_value("IsOwned")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_activated` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_activated(&self, is_activated: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsActivated", &[])?;
        let is_activated = result.get_value("IsActivated")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_physical_clear_disabled` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_physical_clear_disabled(&self, is_physical_clear_disabled: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsPhysicalClearDisabled", &[])?;
        let is_physical_clear_disabled = result.get_value("IsPhysicalClearDisabled")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_owner_clear_disabled` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_owner_clear_disabled(&self, is_owner_clear_disabled: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsOwnerClearDisabled", &[])?;
        let is_owner_clear_disabled = result.get_value("IsOwnerClearDisabled")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_physical_presence_hardware_enabled` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_physical_presence_hardware_enabled(&self, is_physical_presence_hardware_enabled: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsPhysicalPresenceHardwareEnabled", &[])?;
        let is_physical_presence_hardware_enabled = result.get_value("IsPhysicalPresenceHardwareEnabled")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_ownership_allowed` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_ownership_allowed(&self, is_ownership_allowed: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsOwnershipAllowed", &[])?;
        let is_ownership_allowed = result.get_value("IsOwnershipAllowed")?;
        Ok(result.return_value)

    }


/// 

    /// * `command_ordinal` -  (u32)

    /// * `is_command_present` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_command_present(&self, command_ordinal: u32, is_command_present: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CommandOrdinal".to_string(), value: command_ordinal.into() });

        let result = self.invoke_method("IsCommandPresent", &args)?;
        let is_command_present = result.get_value("IsCommandPresent")?;
        Ok(result.return_value)

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn enable(&self, owner_auth: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = owner_auth {
            args.push(MethodParameter { name: "OwnerAuth".to_string(), value: val.into() });
        }
        self.invoke_method("Enable", &args)

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn disable(&self, owner_auth: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = owner_auth {
            args.push(MethodParameter { name: "OwnerAuth".to_string(), value: val.into() });
        }
        self.invoke_method("Disable", &args)

    }


/// 

    /// * `is_endorsement_key_pair_present` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_endorsement_key_pair_present(&self, is_endorsement_key_pair_present: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsEndorsementKeyPairPresent", &[])?;
        let is_endorsement_key_pair_present = result.get_value("IsEndorsementKeyPairPresent")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn create_endorsement_key_pair(&self) -> Result<(), WmiError> {
        self.invoke_method("CreateEndorsementKeyPair", &[])

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn take_ownership(&self, owner_auth: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = owner_auth {
            args.push(MethodParameter { name: "OwnerAuth".to_string(), value: val.into() });
        }
        self.invoke_method("TakeOwnership", &args)

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn clear(&self, owner_auth: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = owner_auth {
            args.push(MethodParameter { name: "OwnerAuth".to_string(), value: val.into() });
        }
        self.invoke_method("Clear", &args)

    }


/// 

    /// * `is_srk_auth_compatible` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_srk_auth_compatible(&self, is_srk_auth_compatible: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsSrkAuthCompatible", &[])?;
        let is_srk_auth_compatible = result.get_value("IsSrkAuthCompatible")?;
        Ok(result.return_value)

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn reset_srk_auth(&self, owner_auth: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = owner_auth {
            args.push(MethodParameter { name: "OwnerAuth".to_string(), value: val.into() });
        }
        self.invoke_method("ResetSrkAuth", &args)

    }


/// 

    /// * `new_owner_auth` -  (String)
    /// * `old_owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn change_owner_auth(&self, old_owner_auth: &Option<String>, new_owner_auth: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = old_owner_auth {
            args.push(MethodParameter { name: "OldOwnerAuth".to_string(), value: val.into() });
        }
        if let Some(val) = new_owner_auth {
            args.push(MethodParameter { name: "NewOwnerAuth".to_string(), value: val.into() });
        }
        self.invoke_method("ChangeOwnerAuth", &args)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `self_test_result` -  (u8[])
    pub fn self_test(&self, self_test_result: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("SelfTest", &[])?;
        let self_test_result = result.get_value("SelfTestResult")?;
        Ok(result.return_value)

    }


/// 

    /// * `owner_pass_phrase` -  (String)

    /// * `owner_auth` -  (String)
    /// * `return_value` -  (u32)
    pub fn convert_to_owner_auth(&self, owner_pass_phrase: &String, owner_auth: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OwnerPassPhrase".to_string(), value: owner_pass_phrase.into() });

        let result = self.invoke_method("ConvertToOwnerAuth", &args)?;
        let owner_auth = result.get_value("OwnerAuth")?;
        Ok(result.return_value)

    }


/// 

    /// * `request` -  (u32)
    /// * `request_parameter` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_physical_presence_request(&self, request: u32, request_parameter: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Request".to_string(), value: request.into() });
        if let Some(val) = request_parameter {
            args.push(MethodParameter { name: "RequestParameter".to_string(), value: val.into() });
        }
        self.invoke_method("SetPhysicalPresenceRequest", &args)

    }


/// 

    /// * `request` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_physical_presence_request(&self, request: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPhysicalPresenceRequest", &[])?;
        let request = result.get_value("Request")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `transition` -  (u32)
    pub fn get_physical_presence_transition(&self, transition: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPhysicalPresenceTransition", &[])?;
        let transition = result.get_value("Transition")?;
        Ok(result.return_value)

    }


/// 

    /// * `request` -  (u32)
    /// * `response` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_physical_presence_response(&self, request: &mut u32, response: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPhysicalPresenceResponse", &[])?;
        let request = result.get_value("Request")?;
        let response = result.get_value("Response")?;
        Ok(result.return_value)

    }


/// 

    /// * `command_ordinal` -  (u32)

    /// * `return_value` -  (u32)
    pub fn add_blocked_command(&self, command_ordinal: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CommandOrdinal".to_string(), value: command_ordinal.into() });
        self.invoke_method("AddBlockedCommand", &args)

    }


/// 

    /// * `command_ordinal` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_blocked_command(&self, command_ordinal: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CommandOrdinal".to_string(), value: command_ordinal.into() });
        self.invoke_method("RemoveBlockedCommand", &args)

    }


/// 

    /// * `command_ordinal` -  (u32)

    /// * `is_command_blocked` -  (u32)
    /// * `return_value` -  (u32)
    pub fn is_command_blocked(&self, command_ordinal: u32, is_command_blocked: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CommandOrdinal".to_string(), value: command_ordinal.into() });

        let result = self.invoke_method("IsCommandBlocked", &args)?;
        let is_command_blocked = result.get_value("IsCommandBlocked")?;
        Ok(result.return_value)

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn reset_auth_lock_out(&self, owner_auth: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = owner_auth {
            args.push(MethodParameter { name: "OwnerAuth".to_string(), value: val.into() });
        }
        self.invoke_method("ResetAuthLockOut", &args)

    }


/// 

    /// * `is_ready` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_ready(&self, is_ready: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsReady", &[])?;
        let is_ready = result.get_value("IsReady")?;
        Ok(result.return_value)

    }


/// 

    /// * `information` -  (u32)
    /// * `is_ready` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_ready_information(&self, is_ready: &mut bool, information: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("IsReadyInformation", &[])?;
        let information = result.get_value("Information")?;
        let is_ready = result.get_value("IsReady")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_auto_provisioning_enabled` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_auto_provisioning_enabled(&self, is_auto_provisioning_enabled: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsAutoProvisioningEnabled", &[])?;
        let is_auto_provisioning_enabled = result.get_value("IsAutoProvisioningEnabled")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn enable_auto_provisioning(&self) -> Result<(), WmiError> {
        self.invoke_method("EnableAutoProvisioning", &[])

    }


/// 

    /// * `only_for_next_boot` -  (bool)

    /// * `return_value` -  (u32)
    pub fn disable_auto_provisioning(&self, only_for_next_boot: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = only_for_next_boot {
            args.push(MethodParameter { name: "OnlyForNextBoot".to_string(), value: val.into() });
        }
        self.invoke_method("DisableAutoProvisioning", &args)

    }


/// 

    /// * `owner_auth` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_owner_auth(&self, owner_auth: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetOwnerAuth", &[])?;
        let owner_auth = result.get_value("OwnerAuth")?;
        Ok(result.return_value)

    }


/// 

    /// * `force_clear__allowed` -  (bool)
    /// * `physical_presence_prompts__allowed` -  (bool)

    /// * `information` -  (u32)
    /// * `return_value` -  (u32)
    pub fn provision(&self, force_clear__allowed: bool, physical_presence_prompts__allowed: bool, information: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ForceClear_Allowed".to_string(), value: force_clear__allowed.into() });
        args.push(MethodParameter { name: "PhysicalPresencePrompts_Allowed".to_string(), value: physical_presence_prompts__allowed.into() });

        let result = self.invoke_method("Provision", &args)?;
        let information = result.get_value("Information")?;
        Ok(result.return_value)

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn import_owner_auth(&self, owner_auth: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OwnerAuth".to_string(), value: owner_auth.into() });
        self.invoke_method("ImportOwnerAuth", &args)

    }


/// 

    /// * `operation` -  (u32)

    /// * `confirmation_status` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_physical_presence_confirmation_status(&self, operation: u32, confirmation_status: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Operation".to_string(), value: operation.into() });

        let result = self.invoke_method("GetPhysicalPresenceConfirmationStatus", &args)?;
        let confirmation_status = result.get_value("ConfirmationStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `srk_public_key_modulus` -  (u8[])
    pub fn get_srk_public_key_modulus(&self, srk_public_key_modulus: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSrkPublicKeyModulus", &[])?;
        let srk_public_key_modulus = result.get_value("SrkPublicKeyModulus")?;
        Ok(result.return_value)

    }


/// 

    /// * `srk_public_key_modulus` -  (u8[])

    /// * `return_value` -  (u32)
    /// * `srk_adthumbprint` -  (u8[])
    pub fn get_srk_adthumbprint(&self, srk_public_key_modulus: &Vec<u8>, srk_adthumbprint: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SrkPublicKeyModulus".to_string(), value: srk_public_key_modulus.into() });

        let result = self.invoke_method("GetSrkADThumbprint", &args)?;
        let srk_adthumbprint = result.get_value("SrkADThumbprint")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `tcg_log` -  (u8[])
    pub fn get_tcg_log(&self, tcg_log: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetTcgLog", &[])?;
        let tcg_log = result.get_value("TcgLog")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `test_result` -  (u32)
    pub fn is_key_attestation_capable(&self, test_result: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("IsKeyAttestationCapable", &[])?;
        let test_result = result.get_value("TestResult")?;
        Ok(result.return_value)

    }


/// 

    /// * `owner_auth` -  (String)
    /// * `owner_auth_status` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_owner_auth_for_escrow(&self, owner_auth_status: &mut u32, owner_auth: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetOwnerAuthForEscrow", &[])?;
        let owner_auth = result.get_value("OwnerAuth")?;
        let owner_auth_status = result.get_value("OwnerAuthStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `owner_auth` -  (String)

    /// * `return_value` -  (u32)
    pub fn owner_auth_escrowed(&self, owner_auth: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OwnerAuth".to_string(), value: owner_auth.into() });
        self.invoke_method("OwnerAuthEscrowed", &args)

    }


/// 

    /// * `owner_auth_status` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_owner_auth_status(&self, owner_auth_status: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetOwnerAuthStatus", &[])?;
        let owner_auth_status = result.get_value("OwnerAuthStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_fips` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_fips(&self, is_fips: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsFIPS", &[])?;
        let is_fips = result.get_value("IsFIPS")?;
        Ok(result.return_value)

    }


/// 

    /// * `lockout_recovery` -  (u32)
    /// * `max_tries` -  (u32)
    /// * `recovery_time` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_dictionary_attack_parameters(&self, max_tries: &mut u32, recovery_time: &mut u32, lockout_recovery: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetDictionaryAttackParameters", &[])?;
        let lockout_recovery = result.get_value("LockoutRecovery")?;
        let max_tries = result.get_value("MaxTries")?;
        let recovery_time = result.get_value("RecoveryTime")?;
        Ok(result.return_value)

    }


/// 

    /// * `lockout_counter` -  (u32)
    /// * `max_tries` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_cap_lockout_info(&self, lockout_counter: &mut u32, max_tries: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetCapLockoutInfo", &[])?;
        let lockout_counter = result.get_value("LockoutCounter")?;
        let max_tries = result.get_value("MaxTries")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_locked_out` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_locked_out(&self, is_locked_out: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("IsLockedOut", &[])?;
        let is_locked_out = result.get_value("IsLockedOut")?;
        Ok(result.return_value)

    }

}

