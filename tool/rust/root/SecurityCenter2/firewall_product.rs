// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SecurityCenter2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FirewallProduct struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FirewallProduct {

/// 
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "instanceGuid")]
    pub instance_guid: Option<String>,

/// 
    #[serde(rename = "pathToSignedProductExe")]
    pub path_to_signed_product_exe: Option<String>,

/// 
    #[serde(rename = "pathToSignedReportingExe")]
    pub path_to_signed_reporting_exe: Option<String>,

/// 
    #[serde(rename = "productState")]
    pub product_state: Option<u32>,

/// 
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
}

impl FirewallProduct {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            display_name: None,
            instance_guid: None,
            path_to_signed_product_exe: None,
            path_to_signed_reporting_exe: None,
            product_state: None,
            timestamp: None,
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

    /// Sets the value of instanceGuid
    pub fn set_instance_guid(&mut self, value: String) {
        self.instance_guid = Some(value);
    }

    /// Gets the value of instanceGuid
    pub fn get_instance_guid(&self) -> Option<&String> {
        self.instance_guid.as_ref()
    }

    /// Sets the value of pathToSignedProductExe
    pub fn set_path_to_signed_product_exe(&mut self, value: String) {
        self.path_to_signed_product_exe = Some(value);
    }

    /// Gets the value of pathToSignedProductExe
    pub fn get_path_to_signed_product_exe(&self) -> Option<&String> {
        self.path_to_signed_product_exe.as_ref()
    }

    /// Sets the value of pathToSignedReportingExe
    pub fn set_path_to_signed_reporting_exe(&mut self, value: String) {
        self.path_to_signed_reporting_exe = Some(value);
    }

    /// Gets the value of pathToSignedReportingExe
    pub fn get_path_to_signed_reporting_exe(&self) -> Option<&String> {
        self.path_to_signed_reporting_exe.as_ref()
    }

    /// Sets the value of productState
    pub fn set_product_state(&mut self, value: u32) {
        self.product_state = Some(value);
    }

    /// Gets the value of productState
    pub fn get_product_state(&self) -> Option<&u32> {
        self.product_state.as_ref()
    }

    /// Sets the value of timestamp
    pub fn set_timestamp(&mut self, value: String) {
        self.timestamp = Some(value);
    }

    /// Gets the value of timestamp
    pub fn get_timestamp(&self) -> Option<&String> {
        self.timestamp.as_ref()
    }
}

