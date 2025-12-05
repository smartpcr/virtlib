// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeviceGuard
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DeviceGuard struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DeviceGuard {

/// 
    #[serde(rename = "AvailableSecurityProperties")]
    pub available_security_properties: Vec<u32>,

/// 
    #[serde(rename = "CodeIntegrityPolicyEnforcementStatus")]
    pub code_integrity_policy_enforcement_status: Option<u32>,

/// 
    #[serde(rename = "InstanceIdentifier")]
    pub instance_identifier: Option<String>,

/// 
    #[serde(rename = "RequiredSecurityProperties")]
    pub required_security_properties: Vec<u32>,

/// 
    #[serde(rename = "SecurityFeaturesEnabled")]
    pub security_features_enabled: Vec<u32>,

/// 
    #[serde(rename = "SecurityServicesConfigured")]
    pub security_services_configured: Vec<u32>,

/// 
    #[serde(rename = "SecurityServicesRunning")]
    pub security_services_running: Vec<u32>,

/// 
    #[serde(rename = "SmmIsolationLevel")]
    pub smm_isolation_level: Option<u8>,

/// 
    #[serde(rename = "UsermodeCodeIntegrityPolicyEnforcementStatus")]
    pub usermode_code_integrity_policy_enforcement_status: Option<u32>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VirtualizationBasedSecurityStatus")]
    pub virtualization_based_security_status: Option<u32>,

/// 
    #[serde(rename = "VirtualMachineIsolation")]
    pub virtual_machine_isolation: Option<bool>,

/// 
    #[serde(rename = "VirtualMachineIsolationProperties")]
    pub virtual_machine_isolation_properties: Vec<u32>,
}

impl Win32_DeviceGuard {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            available_security_properties: Vec::new(),
            code_integrity_policy_enforcement_status: None,
            instance_identifier: None,
            required_security_properties: Vec::new(),
            security_features_enabled: Vec::new(),
            security_services_configured: Vec::new(),
            security_services_running: Vec::new(),
            smm_isolation_level: None,
            usermode_code_integrity_policy_enforcement_status: None,
            version: None,
            virtualization_based_security_status: None,
            virtual_machine_isolation: None,
            virtual_machine_isolation_properties: Vec::new(),
        }
    }


    /// Sets the value of AvailableSecurityProperties
    pub fn set_available_security_properties(&mut self, value: Vec<u32>) {
        self.available_security_properties = value;
    }

    /// Gets the value of AvailableSecurityProperties
    pub fn get_available_security_properties(&self) -> &Vec<u32> {
        &self.available_security_properties
    }

    /// Sets the value of CodeIntegrityPolicyEnforcementStatus
    pub fn set_code_integrity_policy_enforcement_status(&mut self, value: u32) {
        self.code_integrity_policy_enforcement_status = Some(value);
    }

    /// Gets the value of CodeIntegrityPolicyEnforcementStatus
    pub fn get_code_integrity_policy_enforcement_status(&self) -> Option<&u32> {
        self.code_integrity_policy_enforcement_status.as_ref()
    }

    /// Sets the value of InstanceIdentifier
    pub fn set_instance_identifier(&mut self, value: String) {
        self.instance_identifier = Some(value);
    }

    /// Gets the value of InstanceIdentifier
    pub fn get_instance_identifier(&self) -> Option<&String> {
        self.instance_identifier.as_ref()
    }

    /// Sets the value of RequiredSecurityProperties
    pub fn set_required_security_properties(&mut self, value: Vec<u32>) {
        self.required_security_properties = value;
    }

    /// Gets the value of RequiredSecurityProperties
    pub fn get_required_security_properties(&self) -> &Vec<u32> {
        &self.required_security_properties
    }

    /// Sets the value of SecurityFeaturesEnabled
    pub fn set_security_features_enabled(&mut self, value: Vec<u32>) {
        self.security_features_enabled = value;
    }

    /// Gets the value of SecurityFeaturesEnabled
    pub fn get_security_features_enabled(&self) -> &Vec<u32> {
        &self.security_features_enabled
    }

    /// Sets the value of SecurityServicesConfigured
    pub fn set_security_services_configured(&mut self, value: Vec<u32>) {
        self.security_services_configured = value;
    }

    /// Gets the value of SecurityServicesConfigured
    pub fn get_security_services_configured(&self) -> &Vec<u32> {
        &self.security_services_configured
    }

    /// Sets the value of SecurityServicesRunning
    pub fn set_security_services_running(&mut self, value: Vec<u32>) {
        self.security_services_running = value;
    }

    /// Gets the value of SecurityServicesRunning
    pub fn get_security_services_running(&self) -> &Vec<u32> {
        &self.security_services_running
    }

    /// Sets the value of SmmIsolationLevel
    pub fn set_smm_isolation_level(&mut self, value: u8) {
        self.smm_isolation_level = Some(value);
    }

    /// Gets the value of SmmIsolationLevel
    pub fn get_smm_isolation_level(&self) -> Option<&u8> {
        self.smm_isolation_level.as_ref()
    }

    /// Sets the value of UsermodeCodeIntegrityPolicyEnforcementStatus
    pub fn set_usermode_code_integrity_policy_enforcement_status(&mut self, value: u32) {
        self.usermode_code_integrity_policy_enforcement_status = Some(value);
    }

    /// Gets the value of UsermodeCodeIntegrityPolicyEnforcementStatus
    pub fn get_usermode_code_integrity_policy_enforcement_status(&self) -> Option<&u32> {
        self.usermode_code_integrity_policy_enforcement_status.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of VirtualizationBasedSecurityStatus
    pub fn set_virtualization_based_security_status(&mut self, value: u32) {
        self.virtualization_based_security_status = Some(value);
    }

    /// Gets the value of VirtualizationBasedSecurityStatus
    pub fn get_virtualization_based_security_status(&self) -> Option<&u32> {
        self.virtualization_based_security_status.as_ref()
    }

    /// Sets the value of VirtualMachineIsolation
    pub fn set_virtual_machine_isolation(&mut self, value: bool) {
        self.virtual_machine_isolation = Some(value);
    }

    /// Gets the value of VirtualMachineIsolation
    pub fn get_virtual_machine_isolation(&self) -> Option<&bool> {
        self.virtual_machine_isolation.as_ref()
    }

    /// Sets the value of VirtualMachineIsolationProperties
    pub fn set_virtual_machine_isolation_properties(&mut self, value: Vec<u32>) {
        self.virtual_machine_isolation_properties = value;
    }

    /// Gets the value of VirtualMachineIsolationProperties
    pub fn get_virtual_machine_isolation_properties(&self) -> &Vec<u32> {
        &self.virtual_machine_isolation_properties
    }
}

