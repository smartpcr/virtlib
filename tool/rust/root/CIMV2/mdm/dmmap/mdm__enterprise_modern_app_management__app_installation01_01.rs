// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseModernAppManagement_AppInstallation01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseModernAppManagement_AppInstallation01_01 {

/// 
    #[serde(rename = "HostedInstall")]
    pub hosted_install: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LastError")]
    pub last_error: Option<i32>,

/// 
    #[serde(rename = "LastErrorDesc")]
    pub last_error_desc: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ProgressStatus")]
    pub progress_status: Option<i32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,

/// 
    #[serde(rename = "StoreInstall")]
    pub store_install: Option<String>,
}

impl MDM_EnterpriseModernAppManagement_AppInstallation01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            hosted_install: None,
            instance_id: None,
            last_error: None,
            last_error_desc: None,
            parent_id: None,
            progress_status: None,
            status: None,
            store_install: None,
        }
    }


    /// Sets the value of HostedInstall
    pub fn set_hosted_install(&mut self, value: String) {
        self.hosted_install = Some(value);
    }

    /// Gets the value of HostedInstall
    pub fn get_hosted_install(&self) -> Option<&String> {
        self.hosted_install.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LastError
    pub fn set_last_error(&mut self, value: i32) {
        self.last_error = Some(value);
    }

    /// Gets the value of LastError
    pub fn get_last_error(&self) -> Option<&i32> {
        self.last_error.as_ref()
    }

    /// Sets the value of LastErrorDesc
    pub fn set_last_error_desc(&mut self, value: String) {
        self.last_error_desc = Some(value);
    }

    /// Gets the value of LastErrorDesc
    pub fn get_last_error_desc(&self) -> Option<&String> {
        self.last_error_desc.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ProgressStatus
    pub fn set_progress_status(&mut self, value: i32) {
        self.progress_status = Some(value);
    }

    /// Gets the value of ProgressStatus
    pub fn get_progress_status(&self) -> Option<&i32> {
        self.progress_status.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

    /// Sets the value of StoreInstall
    pub fn set_store_install(&mut self, value: String) {
        self.store_install = Some(value);
    }

    /// Gets the value of StoreInstall
    pub fn get_store_install(&self) -> Option<&String> {
        self.store_install.as_ref()
    }

/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn store_install_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("StoreInstallMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn hosted_install_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("HostedInstallMethod", &args)

    }

}

