// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseModernAppManagement_StoreLicenses02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseModernAppManagement_StoreLicenses02_01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LicenseCategory")]
    pub license_category: Option<String>,

/// 
    #[serde(rename = "LicenseUsage")]
    pub license_usage: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequesterID")]
    pub requester_id: Option<String>,
}

impl MDM_EnterpriseModernAppManagement_StoreLicenses02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            license_category: None,
            license_usage: None,
            parent_id: None,
            requester_id: None,
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

    /// Sets the value of LicenseCategory
    pub fn set_license_category(&mut self, value: String) {
        self.license_category = Some(value);
    }

    /// Gets the value of LicenseCategory
    pub fn get_license_category(&self) -> Option<&String> {
        self.license_category.as_ref()
    }

    /// Sets the value of LicenseUsage
    pub fn set_license_usage(&mut self, value: String) {
        self.license_usage = Some(value);
    }

    /// Gets the value of LicenseUsage
    pub fn get_license_usage(&self) -> Option<&String> {
        self.license_usage.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RequesterID
    pub fn set_requester_id(&mut self, value: String) {
        self.requester_id = Some(value);
    }

    /// Gets the value of RequesterID
    pub fn get_requester_id(&self) -> Option<&String> {
        self.requester_id.as_ref()
    }

/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_license_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("AddLicenseMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn get_license_from_store_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("GetLicenseFromStoreMethod", &args)

    }

}

