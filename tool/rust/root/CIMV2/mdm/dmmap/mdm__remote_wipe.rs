// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_RemoteWipe struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_RemoteWipe {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_RemoteWipe {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
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

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_wipe_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("doWipeMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_wipe_persist_provisioned_data_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("doWipePersistProvisionedDataMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_wipe_protected_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("doWipeProtectedMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_wipe_persist_user_data_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("doWipePersistUserDataMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_wipe_cloud_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("doWipeCloudMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_wipe_cloud_persist_user_data_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("doWipeCloudPersistUserDataMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn do_wipe_cloud_persist_provisioned_data_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("doWipeCloudPersistProvisionedDataMethod", &args)

    }

}

