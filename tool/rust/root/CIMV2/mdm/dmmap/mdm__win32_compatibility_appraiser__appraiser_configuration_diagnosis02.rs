// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Win32CompatibilityAppraiser_AppraiserConfigurationDiagnosis02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Win32CompatibilityAppraiser_AppraiserConfigurationDiagnosis02 {

/// 
    #[serde(rename = "AllTargetOsVersionsRequested")]
    pub all_target_os_versions_requested: Option<bool>,

/// 
    #[serde(rename = "AppraiserCodeAndDataVersionsAboveMinimum")]
    pub appraiser_code_and_data_versions_above_minimum: Option<i32>,

/// 
    #[serde(rename = "CommercialId")]
    pub commercial_id: Option<String>,

/// 
    #[serde(rename = "CommercialIdSetAndValid")]
    pub commercial_id_set_and_valid: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "OsSkuIsValidForAppraiser")]
    pub os_sku_is_valid_for_appraiser: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RebootPending")]
    pub reboot_pending: Option<bool>,
}

impl MDM_Win32CompatibilityAppraiser_AppraiserConfigurationDiagnosis02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            all_target_os_versions_requested: None,
            appraiser_code_and_data_versions_above_minimum: None,
            commercial_id: None,
            commercial_id_set_and_valid: None,
            instance_id: None,
            os_sku_is_valid_for_appraiser: None,
            parent_id: None,
            reboot_pending: None,
        }
    }


    /// Sets the value of AllTargetOsVersionsRequested
    pub fn set_all_target_os_versions_requested(&mut self, value: bool) {
        self.all_target_os_versions_requested = Some(value);
    }

    /// Gets the value of AllTargetOsVersionsRequested
    pub fn get_all_target_os_versions_requested(&self) -> Option<&bool> {
        self.all_target_os_versions_requested.as_ref()
    }

    /// Sets the value of AppraiserCodeAndDataVersionsAboveMinimum
    pub fn set_appraiser_code_and_data_versions_above_minimum(&mut self, value: i32) {
        self.appraiser_code_and_data_versions_above_minimum = Some(value);
    }

    /// Gets the value of AppraiserCodeAndDataVersionsAboveMinimum
    pub fn get_appraiser_code_and_data_versions_above_minimum(&self) -> Option<&i32> {
        self.appraiser_code_and_data_versions_above_minimum.as_ref()
    }

    /// Sets the value of CommercialId
    pub fn set_commercial_id(&mut self, value: String) {
        self.commercial_id = Some(value);
    }

    /// Gets the value of CommercialId
    pub fn get_commercial_id(&self) -> Option<&String> {
        self.commercial_id.as_ref()
    }

    /// Sets the value of CommercialIdSetAndValid
    pub fn set_commercial_id_set_and_valid(&mut self, value: bool) {
        self.commercial_id_set_and_valid = Some(value);
    }

    /// Gets the value of CommercialIdSetAndValid
    pub fn get_commercial_id_set_and_valid(&self) -> Option<&bool> {
        self.commercial_id_set_and_valid.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of OsSkuIsValidForAppraiser
    pub fn set_os_sku_is_valid_for_appraiser(&mut self, value: bool) {
        self.os_sku_is_valid_for_appraiser = Some(value);
    }

    /// Gets the value of OsSkuIsValidForAppraiser
    pub fn get_os_sku_is_valid_for_appraiser(&self) -> Option<&bool> {
        self.os_sku_is_valid_for_appraiser.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RebootPending
    pub fn set_reboot_pending(&mut self, value: bool) {
        self.reboot_pending = Some(value);
    }

    /// Gets the value of RebootPending
    pub fn get_reboot_pending(&self) -> Option<&bool> {
        self.reboot_pending.as_ref()
    }
}

