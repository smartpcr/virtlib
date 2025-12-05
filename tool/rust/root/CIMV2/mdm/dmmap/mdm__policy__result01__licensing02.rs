// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Licensing02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Licensing02 {

/// 
    #[serde(rename = "AllowWindowsEntitlementReactivation")]
    pub allow_windows_entitlement_reactivation: Option<i32>,

/// 
    #[serde(rename = "DisallowKMSClientOnlineAVSValidation")]
    pub disallow_kmsclient_online_avsvalidation: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_Licensing02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_windows_entitlement_reactivation: None,
            disallow_kmsclient_online_avsvalidation: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowWindowsEntitlementReactivation
    pub fn set_allow_windows_entitlement_reactivation(&mut self, value: i32) {
        self.allow_windows_entitlement_reactivation = Some(value);
    }

    /// Gets the value of AllowWindowsEntitlementReactivation
    pub fn get_allow_windows_entitlement_reactivation(&self) -> Option<&i32> {
        self.allow_windows_entitlement_reactivation.as_ref()
    }

    /// Sets the value of DisallowKMSClientOnlineAVSValidation
    pub fn set_disallow_kmsclient_online_avsvalidation(&mut self, value: i32) {
        self.disallow_kmsclient_online_avsvalidation = Some(value);
    }

    /// Gets the value of DisallowKMSClientOnlineAVSValidation
    pub fn get_disallow_kmsclient_online_avsvalidation(&self) -> Option<&i32> {
        self.disallow_kmsclient_online_avsvalidation.as_ref()
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
}

