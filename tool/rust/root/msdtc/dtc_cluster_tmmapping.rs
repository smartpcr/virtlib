// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcClusterTMMapping struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcClusterTMMapping {

/// 
    #[serde(rename = "Application")]
    pub application: Option<String>,

/// 
    #[serde(rename = "ApplicationType")]
    pub application_type: Option<String>,

/// 
    #[serde(rename = "ClusterResourceName")]
    pub cluster_resource_name: Option<String>,

/// 
    #[serde(rename = "Local")]
    pub local: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl DtcClusterTMMapping {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            application: None,
            application_type: None,
            cluster_resource_name: None,
            local: None,
            name: None,
        }
    }


    /// Sets the value of Application
    pub fn set_application(&mut self, value: String) {
        self.application = Some(value);
    }

    /// Gets the value of Application
    pub fn get_application(&self) -> Option<&String> {
        self.application.as_ref()
    }

    /// Sets the value of ApplicationType
    pub fn set_application_type(&mut self, value: String) {
        self.application_type = Some(value);
    }

    /// Gets the value of ApplicationType
    pub fn get_application_type(&self) -> Option<&String> {
        self.application_type.as_ref()
    }

    /// Sets the value of ClusterResourceName
    pub fn set_cluster_resource_name(&mut self, value: String) {
        self.cluster_resource_name = Some(value);
    }

    /// Gets the value of ClusterResourceName
    pub fn get_cluster_resource_name(&self) -> Option<&String> {
        self.cluster_resource_name.as_ref()
    }

    /// Sets the value of Local
    pub fn set_local(&mut self, value: bool) {
        self.local = Some(value);
    }

    /// Gets the value of Local
    pub fn get_local(&self) -> Option<&bool> {
        self.local.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

