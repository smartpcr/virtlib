// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_EnterpriseCloudPrint02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_EnterpriseCloudPrint02 {

/// 
    #[serde(rename = "CloudPrinterDiscoveryEndPoint")]
    pub cloud_printer_discovery_end_point: Option<String>,

/// 
    #[serde(rename = "CloudPrintOAuthAuthority")]
    pub cloud_print_oauth_authority: Option<String>,

/// 
    #[serde(rename = "CloudPrintOAuthClientId")]
    pub cloud_print_oauth_client_id: Option<String>,

/// 
    #[serde(rename = "CloudPrintResourceId")]
    pub cloud_print_resource_id: Option<String>,

/// 
    #[serde(rename = "DiscoveryMaxPrinterLimit")]
    pub discovery_max_printer_limit: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MopriaDiscoveryResourceId")]
    pub mopria_discovery_resource_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_User_Config01_EnterpriseCloudPrint02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cloud_printer_discovery_end_point: None,
            cloud_print_oauth_authority: None,
            cloud_print_oauth_client_id: None,
            cloud_print_resource_id: None,
            discovery_max_printer_limit: None,
            instance_id: None,
            mopria_discovery_resource_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of CloudPrinterDiscoveryEndPoint
    pub fn set_cloud_printer_discovery_end_point(&mut self, value: String) {
        self.cloud_printer_discovery_end_point = Some(value);
    }

    /// Gets the value of CloudPrinterDiscoveryEndPoint
    pub fn get_cloud_printer_discovery_end_point(&self) -> Option<&String> {
        self.cloud_printer_discovery_end_point.as_ref()
    }

    /// Sets the value of CloudPrintOAuthAuthority
    pub fn set_cloud_print_oauth_authority(&mut self, value: String) {
        self.cloud_print_oauth_authority = Some(value);
    }

    /// Gets the value of CloudPrintOAuthAuthority
    pub fn get_cloud_print_oauth_authority(&self) -> Option<&String> {
        self.cloud_print_oauth_authority.as_ref()
    }

    /// Sets the value of CloudPrintOAuthClientId
    pub fn set_cloud_print_oauth_client_id(&mut self, value: String) {
        self.cloud_print_oauth_client_id = Some(value);
    }

    /// Gets the value of CloudPrintOAuthClientId
    pub fn get_cloud_print_oauth_client_id(&self) -> Option<&String> {
        self.cloud_print_oauth_client_id.as_ref()
    }

    /// Sets the value of CloudPrintResourceId
    pub fn set_cloud_print_resource_id(&mut self, value: String) {
        self.cloud_print_resource_id = Some(value);
    }

    /// Gets the value of CloudPrintResourceId
    pub fn get_cloud_print_resource_id(&self) -> Option<&String> {
        self.cloud_print_resource_id.as_ref()
    }

    /// Sets the value of DiscoveryMaxPrinterLimit
    pub fn set_discovery_max_printer_limit(&mut self, value: i32) {
        self.discovery_max_printer_limit = Some(value);
    }

    /// Gets the value of DiscoveryMaxPrinterLimit
    pub fn get_discovery_max_printer_limit(&self) -> Option<&i32> {
        self.discovery_max_printer_limit.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MopriaDiscoveryResourceId
    pub fn set_mopria_discovery_resource_id(&mut self, value: String) {
        self.mopria_discovery_resource_id = Some(value);
    }

    /// Gets the value of MopriaDiscoveryResourceId
    pub fn get_mopria_discovery_resource_id(&self) -> Option<&String> {
        self.mopria_discovery_resource_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

