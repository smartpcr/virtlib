// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WindowsLicensing struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WindowsLicensing {

/// 
    #[serde(rename = "Edition")]
    pub edition: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LicenseKeyType")]
    pub license_key_type: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,
}

impl MDM_WindowsLicensing {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            edition: None,
            instance_id: None,
            license_key_type: None,
            parent_id: None,
            status: None,
        }
    }


    /// Sets the value of Edition
    pub fn set_edition(&mut self, value: i32) {
        self.edition = Some(value);
    }

    /// Gets the value of Edition
    pub fn get_edition(&self) -> Option<&i32> {
        self.edition.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LicenseKeyType
    pub fn set_license_key_type(&mut self, value: String) {
        self.license_key_type = Some(value);
    }

    /// Gets the value of LicenseKeyType
    pub fn get_license_key_type(&self) -> Option<&String> {
        self.license_key_type.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
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
    pub fn upgrade_edition_with_product_key_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("UpgradeEditionWithProductKeyMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn change_product_key_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("ChangeProductKeyMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn upgrade_edition_with_license_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("UpgradeEditionWithLicenseMethod", &args)

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn check_applicability_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("CheckApplicabilityMethod", &args)

    }

}

