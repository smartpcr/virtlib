// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ClusterUtilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ClusterUtilities {

/// 
    #[serde(rename = "Fqdn")]
    pub fqdn: Option<String>,

/// 
    #[serde(rename = "HasSystemAccess")]
    pub has_system_access: Option<bool>,
}

impl MSCluster_ClusterUtilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fqdn: None,
            has_system_access: None,
        }
    }


    /// Sets the value of Fqdn
    pub fn set_fqdn(&mut self, value: String) {
        self.fqdn = Some(value);
    }

    /// Gets the value of Fqdn
    pub fn get_fqdn(&self) -> Option<&String> {
        self.fqdn.as_ref()
    }

    /// Sets the value of HasSystemAccess
    pub fn set_has_system_access(&mut self, value: bool) {
        self.has_system_access = Some(value);
    }

    /// Gets the value of HasSystemAccess
    pub fn get_has_system_access(&self) -> Option<&bool> {
        self.has_system_access.as_ref()
    }

/// 

    /// * `return_value` -  (bool)
    pub fn is_cluster_supported(&self) -> Result<(), WmiError> {
        self.invoke_method("IsClusterSupported", &[])

    }


/// 

    /// * `return_value` -  (bool)
    pub fn is_storage_spaces_direct_supported(&self) -> Result<(), WmiError> {
        self.invoke_method("IsStorageSpacesDirectSupported", &[])

    }


/// 

    /// * `return_value` -  (bool)
    pub fn is_storage_spaces_direct_cache_supported(&self) -> Result<(), WmiError> {
        self.invoke_method("IsStorageSpacesDirectCacheSupported", &[])

    }

}

