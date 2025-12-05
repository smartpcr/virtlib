// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WindowsDefenderApplicationGuard struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WindowsDefenderApplicationGuard {

/// 
    #[serde(rename = "InstallWindowsDefenderApplicationGuard")]
    pub install_windows_defender_application_guard: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PlatformStatus")]
    pub platform_status: Option<i32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,
}

impl MDM_WindowsDefenderApplicationGuard {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            install_windows_defender_application_guard: None,
            instance_id: None,
            parent_id: None,
            platform_status: None,
            status: None,
        }
    }


    /// Sets the value of InstallWindowsDefenderApplicationGuard
    pub fn set_install_windows_defender_application_guard(&mut self, value: String) {
        self.install_windows_defender_application_guard = Some(value);
    }

    /// Gets the value of InstallWindowsDefenderApplicationGuard
    pub fn get_install_windows_defender_application_guard(&self) -> Option<&String> {
        self.install_windows_defender_application_guard.as_ref()
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

    /// Sets the value of PlatformStatus
    pub fn set_platform_status(&mut self, value: i32) {
        self.platform_status = Some(value);
    }

    /// Gets the value of PlatformStatus
    pub fn get_platform_status(&self) -> Option<&i32> {
        self.platform_status.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn install_windows_defender_application_guard_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("InstallWindowsDefenderApplicationGuardMethod", &args)

    }

}

