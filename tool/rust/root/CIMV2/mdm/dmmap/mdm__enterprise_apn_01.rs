// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseAPN_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseAPN_01 {

/// 
    #[serde(rename = "AlwaysOn")]
    pub always_on: Option<bool>,

/// 
    #[serde(rename = "APNName")]
    pub apnname: Option<String>,

/// 
    #[serde(rename = "AuthType")]
    pub auth_type: Option<String>,

/// 
    #[serde(rename = "ClassId")]
    pub class_id: Option<String>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "IccId")]
    pub icc_id: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IPType")]
    pub iptype: Option<String>,

/// 
    #[serde(rename = "IsAttachAPN")]
    pub is_attach_apn: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "Roaming")]
    pub roaming: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl MDM_EnterpriseAPN_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            always_on: None,
            apnname: None,
            auth_type: None,
            class_id: None,
            enabled: None,
            icc_id: None,
            instance_id: None,
            iptype: None,
            is_attach_apn: None,
            parent_id: None,
            password: None,
            roaming: None,
            user_name: None,
        }
    }


    /// Sets the value of AlwaysOn
    pub fn set_always_on(&mut self, value: bool) {
        self.always_on = Some(value);
    }

    /// Gets the value of AlwaysOn
    pub fn get_always_on(&self) -> Option<&bool> {
        self.always_on.as_ref()
    }

    /// Sets the value of APNName
    pub fn set_apnname(&mut self, value: String) {
        self.apnname = Some(value);
    }

    /// Gets the value of APNName
    pub fn get_apnname(&self) -> Option<&String> {
        self.apnname.as_ref()
    }

    /// Sets the value of AuthType
    pub fn set_auth_type(&mut self, value: String) {
        self.auth_type = Some(value);
    }

    /// Gets the value of AuthType
    pub fn get_auth_type(&self) -> Option<&String> {
        self.auth_type.as_ref()
    }

    /// Sets the value of ClassId
    pub fn set_class_id(&mut self, value: String) {
        self.class_id = Some(value);
    }

    /// Gets the value of ClassId
    pub fn get_class_id(&self) -> Option<&String> {
        self.class_id.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of IccId
    pub fn set_icc_id(&mut self, value: String) {
        self.icc_id = Some(value);
    }

    /// Gets the value of IccId
    pub fn get_icc_id(&self) -> Option<&String> {
        self.icc_id.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IPType
    pub fn set_iptype(&mut self, value: String) {
        self.iptype = Some(value);
    }

    /// Gets the value of IPType
    pub fn get_iptype(&self) -> Option<&String> {
        self.iptype.as_ref()
    }

    /// Sets the value of IsAttachAPN
    pub fn set_is_attach_apn(&mut self, value: bool) {
        self.is_attach_apn = Some(value);
    }

    /// Gets the value of IsAttachAPN
    pub fn get_is_attach_apn(&self) -> Option<&bool> {
        self.is_attach_apn.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Sets the value of Roaming
    pub fn set_roaming(&mut self, value: String) {
        self.roaming = Some(value);
    }

    /// Gets the value of Roaming
    pub fn get_roaming(&self) -> Option<&String> {
        self.roaming.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

