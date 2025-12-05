// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseModernAppManagement_AppManagement01_02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseModernAppManagement_AppManagement01_02 {

/// 
    #[serde(rename = "DoNotUpdate")]
    pub do_not_update: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MaintainProcessorArchitectureOnUpdate")]
    pub maintain_processor_architecture_on_update: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_EnterpriseModernAppManagement_AppManagement01_02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            do_not_update: None,
            instance_id: None,
            maintain_processor_architecture_on_update: None,
            parent_id: None,
        }
    }


    /// Sets the value of DoNotUpdate
    pub fn set_do_not_update(&mut self, value: i32) {
        self.do_not_update = Some(value);
    }

    /// Gets the value of DoNotUpdate
    pub fn get_do_not_update(&self) -> Option<&i32> {
        self.do_not_update.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MaintainProcessorArchitectureOnUpdate
    pub fn set_maintain_processor_architecture_on_update(&mut self, value: i32) {
        self.maintain_processor_architecture_on_update = Some(value);
    }

    /// Gets the value of MaintainProcessorArchitectureOnUpdate
    pub fn get_maintain_processor_architecture_on_update(&self) -> Option<&i32> {
        self.maintain_processor_architecture_on_update.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

