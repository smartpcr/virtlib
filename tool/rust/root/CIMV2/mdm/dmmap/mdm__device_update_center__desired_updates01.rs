// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceUpdateCenter_DesiredUpdates01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceUpdateCenter_DesiredUpdates01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "OcpVersion")]
    pub ocp_version: Option<String>,

/// 
    #[serde(rename = "OsVersion")]
    pub os_version: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SystemManifestVersion")]
    pub system_manifest_version: Option<String>,
}

impl MDM_DeviceUpdateCenter_DesiredUpdates01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            ocp_version: None,
            os_version: None,
            parent_id: None,
            system_manifest_version: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of OcpVersion
    pub fn set_ocp_version(&mut self, value: String) {
        self.ocp_version = Some(value);
    }

    /// Gets the value of OcpVersion
    pub fn get_ocp_version(&self) -> Option<&String> {
        self.ocp_version.as_ref()
    }

    /// Sets the value of OsVersion
    pub fn set_os_version(&mut self, value: String) {
        self.os_version = Some(value);
    }

    /// Gets the value of OsVersion
    pub fn get_os_version(&self) -> Option<&String> {
        self.os_version.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SystemManifestVersion
    pub fn set_system_manifest_version(&mut self, value: String) {
        self.system_manifest_version = Some(value);
    }

    /// Gets the value of SystemManifestVersion
    pub fn get_system_manifest_version(&self) -> Option<&String> {
        self.system_manifest_version.as_ref()
    }
}

