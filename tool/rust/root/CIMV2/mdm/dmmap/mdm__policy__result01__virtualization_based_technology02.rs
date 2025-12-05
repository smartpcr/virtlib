// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_VirtualizationBasedTechnology02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_VirtualizationBasedTechnology02 {

/// 
    #[serde(rename = "HypervisorEnforcedCodeIntegrity")]
    pub hypervisor_enforced_code_integrity: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequireUEFIMemoryAttributesTable")]
    pub require_uefimemory_attributes_table: Option<i32>,
}

impl MDM_Policy_Result01_VirtualizationBasedTechnology02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            hypervisor_enforced_code_integrity: None,
            instance_id: None,
            parent_id: None,
            require_uefimemory_attributes_table: None,
        }
    }


    /// Sets the value of HypervisorEnforcedCodeIntegrity
    pub fn set_hypervisor_enforced_code_integrity(&mut self, value: i32) {
        self.hypervisor_enforced_code_integrity = Some(value);
    }

    /// Gets the value of HypervisorEnforcedCodeIntegrity
    pub fn get_hypervisor_enforced_code_integrity(&self) -> Option<&i32> {
        self.hypervisor_enforced_code_integrity.as_ref()
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

    /// Sets the value of RequireUEFIMemoryAttributesTable
    pub fn set_require_uefimemory_attributes_table(&mut self, value: i32) {
        self.require_uefimemory_attributes_table = Some(value);
    }

    /// Gets the value of RequireUEFIMemoryAttributesTable
    pub fn get_require_uefimemory_attributes_table(&self) -> Option<&i32> {
        self.require_uefimemory_attributes_table.as_ref()
    }
}

