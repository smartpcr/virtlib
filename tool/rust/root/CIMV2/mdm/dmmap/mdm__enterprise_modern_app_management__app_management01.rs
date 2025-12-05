// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseModernAppManagement_AppManagement01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseModernAppManagement_AppManagement01 {

/// 
    #[serde(rename = "AppInventoryQuery")]
    pub app_inventory_query: Option<String>,

/// 
    #[serde(rename = "AppInventoryResults")]
    pub app_inventory_results: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LastScanError")]
    pub last_scan_error: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RemovePackage")]
    pub remove_package: Option<String>,
}

impl MDM_EnterpriseModernAppManagement_AppManagement01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_inventory_query: None,
            app_inventory_results: None,
            instance_id: None,
            last_scan_error: None,
            parent_id: None,
            remove_package: None,
        }
    }


    /// Sets the value of AppInventoryQuery
    pub fn set_app_inventory_query(&mut self, value: String) {
        self.app_inventory_query = Some(value);
    }

    /// Gets the value of AppInventoryQuery
    pub fn get_app_inventory_query(&self) -> Option<&String> {
        self.app_inventory_query.as_ref()
    }

    /// Sets the value of AppInventoryResults
    pub fn set_app_inventory_results(&mut self, value: String) {
        self.app_inventory_results = Some(value);
    }

    /// Gets the value of AppInventoryResults
    pub fn get_app_inventory_results(&self) -> Option<&String> {
        self.app_inventory_results.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LastScanError
    pub fn set_last_scan_error(&mut self, value: i32) {
        self.last_scan_error = Some(value);
    }

    /// Gets the value of LastScanError
    pub fn get_last_scan_error(&self) -> Option<&i32> {
        self.last_scan_error.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RemovePackage
    pub fn set_remove_package(&mut self, value: String) {
        self.remove_package = Some(value);
    }

    /// Gets the value of RemovePackage
    pub fn get_remove_package(&self) -> Option<&String> {
        self.remove_package.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn update_scan_method(&self) -> Result<(), WmiError> {
        self.invoke_method("UpdateScanMethod", &[])

    }


/// 

    /// * `param` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_package_method(&self, param: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "param".to_string(), value: param.into() });
        self.invoke_method("RemovePackageMethod", &args)

    }

}

