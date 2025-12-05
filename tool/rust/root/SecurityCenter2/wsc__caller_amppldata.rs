// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SecurityCenter2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSC_CallerAMPPLData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSC_CallerAMPPLData {

/// 
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "hresultCode")]
    pub hresult_code: Option<i32>,

/// 
    #[serde(rename = "iniString")]
    pub ini_string: Option<String>,

/// 
    #[serde(rename = "instanceGuid")]
    pub instance_guid: Option<String>,

/// 
    #[serde(rename = "pathToCaller")]
    pub path_to_caller: Option<String>,

/// 
    #[serde(rename = "pathToSignedProductExe")]
    pub path_to_signed_product_exe: Option<String>,

/// 
    #[serde(rename = "statusCode")]
    pub status_code: Option<i32>,
}

impl WSC_CallerAMPPLData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            display_name: None,
            hresult_code: None,
            ini_string: None,
            instance_guid: None,
            path_to_caller: None,
            path_to_signed_product_exe: None,
            status_code: None,
        }
    }


    /// Sets the value of displayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of displayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of hresultCode
    pub fn set_hresult_code(&mut self, value: i32) {
        self.hresult_code = Some(value);
    }

    /// Gets the value of hresultCode
    pub fn get_hresult_code(&self) -> Option<&i32> {
        self.hresult_code.as_ref()
    }

    /// Sets the value of iniString
    pub fn set_ini_string(&mut self, value: String) {
        self.ini_string = Some(value);
    }

    /// Gets the value of iniString
    pub fn get_ini_string(&self) -> Option<&String> {
        self.ini_string.as_ref()
    }

    /// Sets the value of instanceGuid
    pub fn set_instance_guid(&mut self, value: String) {
        self.instance_guid = Some(value);
    }

    /// Gets the value of instanceGuid
    pub fn get_instance_guid(&self) -> Option<&String> {
        self.instance_guid.as_ref()
    }

    /// Sets the value of pathToCaller
    pub fn set_path_to_caller(&mut self, value: String) {
        self.path_to_caller = Some(value);
    }

    /// Gets the value of pathToCaller
    pub fn get_path_to_caller(&self) -> Option<&String> {
        self.path_to_caller.as_ref()
    }

    /// Sets the value of pathToSignedProductExe
    pub fn set_path_to_signed_product_exe(&mut self, value: String) {
        self.path_to_signed_product_exe = Some(value);
    }

    /// Gets the value of pathToSignedProductExe
    pub fn get_path_to_signed_product_exe(&self) -> Option<&String> {
        self.path_to_signed_product_exe.as_ref()
    }

    /// Sets the value of statusCode
    pub fn set_status_code(&mut self, value: i32) {
        self.status_code = Some(value);
    }

    /// Gets the value of statusCode
    pub fn get_status_code(&self) -> Option<&i32> {
        self.status_code.as_ref()
    }
}

