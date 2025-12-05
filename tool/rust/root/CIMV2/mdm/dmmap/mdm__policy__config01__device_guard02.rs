// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_DeviceGuard02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_DeviceGuard02 {

/// 
    #[serde(rename = "ConfigureSystemGuardLaunch")]
    pub configure_system_guard_launch: Option<i32>,

/// 
    #[serde(rename = "EnableVirtualizationBasedSecurity")]
    pub enable_virtualization_based_security: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LsaCfgFlags")]
    pub lsa_cfg_flags: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequirePlatformSecurityFeatures")]
    pub require_platform_security_features: Option<i32>,
}

impl MDM_Policy_Config01_DeviceGuard02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configure_system_guard_launch: None,
            enable_virtualization_based_security: None,
            instance_id: None,
            lsa_cfg_flags: None,
            parent_id: None,
            require_platform_security_features: None,
        }
    }


    /// Sets the value of ConfigureSystemGuardLaunch
    pub fn set_configure_system_guard_launch(&mut self, value: i32) {
        self.configure_system_guard_launch = Some(value);
    }

    /// Gets the value of ConfigureSystemGuardLaunch
    pub fn get_configure_system_guard_launch(&self) -> Option<&i32> {
        self.configure_system_guard_launch.as_ref()
    }

    /// Sets the value of EnableVirtualizationBasedSecurity
    pub fn set_enable_virtualization_based_security(&mut self, value: i32) {
        self.enable_virtualization_based_security = Some(value);
    }

    /// Gets the value of EnableVirtualizationBasedSecurity
    pub fn get_enable_virtualization_based_security(&self) -> Option<&i32> {
        self.enable_virtualization_based_security.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LsaCfgFlags
    pub fn set_lsa_cfg_flags(&mut self, value: i32) {
        self.lsa_cfg_flags = Some(value);
    }

    /// Gets the value of LsaCfgFlags
    pub fn get_lsa_cfg_flags(&self) -> Option<&i32> {
        self.lsa_cfg_flags.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RequirePlatformSecurityFeatures
    pub fn set_require_platform_security_features(&mut self, value: i32) {
        self.require_platform_security_features = Some(value);
    }

    /// Gets the value of RequirePlatformSecurityFeatures
    pub fn get_require_platform_security_features(&self) -> Option<&i32> {
        self.require_platform_security_features.as_ref()
    }
}

