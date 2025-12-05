// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_PassportForWork_Biometrics01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_PassportForWork_Biometrics01 {

/// 
    #[serde(rename = "FacialFeaturesUseEnhancedAntiSpoofing")]
    pub facial_features_use_enhanced_anti_spoofing: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "UseBiometrics")]
    pub use_biometrics: Option<bool>,
}

impl MDM_PassportForWork_Biometrics01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            facial_features_use_enhanced_anti_spoofing: None,
            instance_id: None,
            parent_id: None,
            use_biometrics: None,
        }
    }


    /// Sets the value of FacialFeaturesUseEnhancedAntiSpoofing
    pub fn set_facial_features_use_enhanced_anti_spoofing(&mut self, value: bool) {
        self.facial_features_use_enhanced_anti_spoofing = Some(value);
    }

    /// Gets the value of FacialFeaturesUseEnhancedAntiSpoofing
    pub fn get_facial_features_use_enhanced_anti_spoofing(&self) -> Option<&bool> {
        self.facial_features_use_enhanced_anti_spoofing.as_ref()
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

    /// Sets the value of UseBiometrics
    pub fn set_use_biometrics(&mut self, value: bool) {
        self.use_biometrics = Some(value);
    }

    /// Gets the value of UseBiometrics
    pub fn get_use_biometrics(&self) -> Option<&bool> {
        self.use_biometrics.as_ref()
    }
}

