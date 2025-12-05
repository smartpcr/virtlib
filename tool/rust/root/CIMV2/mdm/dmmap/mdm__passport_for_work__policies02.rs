// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_PassportForWork_Policies02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_PassportForWork_Policies02 {

/// 
    #[serde(rename = "EnablePinRecovery")]
    pub enable_pin_recovery: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequireSecurityDevice")]
    pub require_security_device: Option<bool>,

/// 
    #[serde(rename = "UseHelloCertificatesAsSmartCardCertificates")]
    pub use_hello_certificates_as_smart_card_certificates: Option<bool>,

/// 
    #[serde(rename = "UsePassportForWork")]
    pub use_passport_for_work: Option<bool>,
}

impl MDM_PassportForWork_Policies02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enable_pin_recovery: None,
            instance_id: None,
            parent_id: None,
            require_security_device: None,
            use_hello_certificates_as_smart_card_certificates: None,
            use_passport_for_work: None,
        }
    }


    /// Sets the value of EnablePinRecovery
    pub fn set_enable_pin_recovery(&mut self, value: bool) {
        self.enable_pin_recovery = Some(value);
    }

    /// Gets the value of EnablePinRecovery
    pub fn get_enable_pin_recovery(&self) -> Option<&bool> {
        self.enable_pin_recovery.as_ref()
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

    /// Sets the value of RequireSecurityDevice
    pub fn set_require_security_device(&mut self, value: bool) {
        self.require_security_device = Some(value);
    }

    /// Gets the value of RequireSecurityDevice
    pub fn get_require_security_device(&self) -> Option<&bool> {
        self.require_security_device.as_ref()
    }

    /// Sets the value of UseHelloCertificatesAsSmartCardCertificates
    pub fn set_use_hello_certificates_as_smart_card_certificates(&mut self, value: bool) {
        self.use_hello_certificates_as_smart_card_certificates = Some(value);
    }

    /// Gets the value of UseHelloCertificatesAsSmartCardCertificates
    pub fn get_use_hello_certificates_as_smart_card_certificates(&self) -> Option<&bool> {
        self.use_hello_certificates_as_smart_card_certificates.as_ref()
    }

    /// Sets the value of UsePassportForWork
    pub fn set_use_passport_for_work(&mut self, value: bool) {
        self.use_passport_for_work = Some(value);
    }

    /// Gets the value of UsePassportForWork
    pub fn get_use_passport_for_work(&self) -> Option<&bool> {
        self.use_passport_for_work.as_ref()
    }
}

