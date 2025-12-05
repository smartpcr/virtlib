// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceStatus_DeviceGuard01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceStatus_DeviceGuard01 {

/// 
    #[serde(rename = "HypervisorEnforcedCodeIntegrityStatus")]
    pub hypervisor_enforced_code_integrity_status: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LsaCfgCredGuardStatus")]
    pub lsa_cfg_cred_guard_status: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SystemGuardStatus")]
    pub system_guard_status: Option<i32>,

/// 
    #[serde(rename = "VirtualizationBasedSecurityHwReq")]
    pub virtualization_based_security_hw_req: Option<i32>,

/// 
    #[serde(rename = "VirtualizationBasedSecurityStatus")]
    pub virtualization_based_security_status: Option<i32>,
}

impl MDM_DeviceStatus_DeviceGuard01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            hypervisor_enforced_code_integrity_status: None,
            instance_id: None,
            lsa_cfg_cred_guard_status: None,
            parent_id: None,
            system_guard_status: None,
            virtualization_based_security_hw_req: None,
            virtualization_based_security_status: None,
        }
    }


    /// Sets the value of HypervisorEnforcedCodeIntegrityStatus
    pub fn set_hypervisor_enforced_code_integrity_status(&mut self, value: i32) {
        self.hypervisor_enforced_code_integrity_status = Some(value);
    }

    /// Gets the value of HypervisorEnforcedCodeIntegrityStatus
    pub fn get_hypervisor_enforced_code_integrity_status(&self) -> Option<&i32> {
        self.hypervisor_enforced_code_integrity_status.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LsaCfgCredGuardStatus
    pub fn set_lsa_cfg_cred_guard_status(&mut self, value: i32) {
        self.lsa_cfg_cred_guard_status = Some(value);
    }

    /// Gets the value of LsaCfgCredGuardStatus
    pub fn get_lsa_cfg_cred_guard_status(&self) -> Option<&i32> {
        self.lsa_cfg_cred_guard_status.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SystemGuardStatus
    pub fn set_system_guard_status(&mut self, value: i32) {
        self.system_guard_status = Some(value);
    }

    /// Gets the value of SystemGuardStatus
    pub fn get_system_guard_status(&self) -> Option<&i32> {
        self.system_guard_status.as_ref()
    }

    /// Sets the value of VirtualizationBasedSecurityHwReq
    pub fn set_virtualization_based_security_hw_req(&mut self, value: i32) {
        self.virtualization_based_security_hw_req = Some(value);
    }

    /// Gets the value of VirtualizationBasedSecurityHwReq
    pub fn get_virtualization_based_security_hw_req(&self) -> Option<&i32> {
        self.virtualization_based_security_hw_req.as_ref()
    }

    /// Sets the value of VirtualizationBasedSecurityStatus
    pub fn set_virtualization_based_security_status(&mut self, value: i32) {
        self.virtualization_based_security_status = Some(value);
    }

    /// Gets the value of VirtualizationBasedSecurityStatus
    pub fn get_virtualization_based_security_status(&self) -> Option<&i32> {
        self.virtualization_based_security_status.as_ref()
    }
}

