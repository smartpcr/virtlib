// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_PassportForWork_Device_Policies02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_PassportForWork_Device_Policies02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "UseCertificateForOnPremAuth")]
    pub use_certificate_for_on_prem_auth: Option<bool>,
}

impl MDM_PassportForWork_Device_Policies02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            use_certificate_for_on_prem_auth: None,
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

    /// Sets the value of UseCertificateForOnPremAuth
    pub fn set_use_certificate_for_on_prem_auth(&mut self, value: bool) {
        self.use_certificate_for_on_prem_auth = Some(value);
    }

    /// Gets the value of UseCertificateForOnPremAuth
    pub fn get_use_certificate_for_on_prem_auth(&self) -> Option<&bool> {
        self.use_certificate_for_on_prem_auth.as_ref()
    }
}

