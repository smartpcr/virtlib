// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_Certificate04 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_Certificate04 {

/// 
    #[serde(rename = "Eku")]
    pub eku: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "Issuer")]
    pub issuer: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_VPNv2_Certificate04 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            eku: None,
            instance_id: None,
            issuer: None,
            parent_id: None,
        }
    }


    /// Sets the value of Eku
    pub fn set_eku(&mut self, value: String) {
        self.eku = Some(value);
    }

    /// Gets the value of Eku
    pub fn get_eku(&self) -> Option<&String> {
        self.eku.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of Issuer
    pub fn set_issuer(&mut self, value: String) {
        self.issuer = Some(value);
    }

    /// Gets the value of Issuer
    pub fn get_issuer(&self) -> Option<&String> {
        self.issuer.as_ref()
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

