// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Update_Rollback01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Update_Rollback01 {

/// 
    #[serde(rename = "FeatureUpdateStatus")]
    pub feature_update_status: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "QualityUpdateStatus")]
    pub quality_update_status: Option<String>,
}

impl MDM_Update_Rollback01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            feature_update_status: None,
            instance_id: None,
            parent_id: None,
            quality_update_status: None,
        }
    }


    /// Sets the value of FeatureUpdateStatus
    pub fn set_feature_update_status(&mut self, value: String) {
        self.feature_update_status = Some(value);
    }

    /// Gets the value of FeatureUpdateStatus
    pub fn get_feature_update_status(&self) -> Option<&String> {
        self.feature_update_status.as_ref()
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

    /// Sets the value of QualityUpdateStatus
    pub fn set_quality_update_status(&mut self, value: String) {
        self.quality_update_status = Some(value);
    }

    /// Gets the value of QualityUpdateStatus
    pub fn get_quality_update_status(&self) -> Option<&String> {
        self.quality_update_status.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn quality_update_method(&self) -> Result<(), WmiError> {
        self.invoke_method("QualityUpdateMethod", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn feature_update_method(&self) -> Result<(), WmiError> {
        self.invoke_method("FeatureUpdateMethod", &[])

    }

}

