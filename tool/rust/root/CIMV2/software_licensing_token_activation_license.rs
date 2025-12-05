// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SoftwareLicensingTokenActivationLicense struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareLicensingTokenActivationLicense {

/// 
    #[serde(rename = "AdditionalInfo")]
    pub additional_info: Option<String>,

/// 
    #[serde(rename = "AuthorizationStatus")]
    pub authorization_status: Option<u32>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "ExpirationDate")]
    pub expiration_date: Option<String>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "ILID")]
    pub ilid: Option<String>,

/// 
    #[serde(rename = "ILVID")]
    pub ilvid: Option<u32>,
}

impl SoftwareLicensingTokenActivationLicense {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            additional_info: None,
            authorization_status: None,
            description: None,
            expiration_date: None,
            id: None,
            ilid: None,
            ilvid: None,
        }
    }


    /// Sets the value of AdditionalInfo
    pub fn set_additional_info(&mut self, value: String) {
        self.additional_info = Some(value);
    }

    /// Gets the value of AdditionalInfo
    pub fn get_additional_info(&self) -> Option<&String> {
        self.additional_info.as_ref()
    }

    /// Sets the value of AuthorizationStatus
    pub fn set_authorization_status(&mut self, value: u32) {
        self.authorization_status = Some(value);
    }

    /// Gets the value of AuthorizationStatus
    pub fn get_authorization_status(&self) -> Option<&u32> {
        self.authorization_status.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of ExpirationDate
    pub fn set_expiration_date(&mut self, value: String) {
        self.expiration_date = Some(value);
    }

    /// Gets the value of ExpirationDate
    pub fn get_expiration_date(&self) -> Option<&String> {
        self.expiration_date.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of ILID
    pub fn set_ilid(&mut self, value: String) {
        self.ilid = Some(value);
    }

    /// Gets the value of ILID
    pub fn get_ilid(&self) -> Option<&String> {
        self.ilid.as_ref()
    }

    /// Sets the value of ILVID
    pub fn set_ilvid(&mut self, value: u32) {
        self.ilvid = Some(value);
    }

    /// Gets the value of ILVID
    pub fn get_ilvid(&self) -> Option<&u32> {
        self.ilvid.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn uninstall(&self) -> Result<(), WmiError> {
        self.invoke_method("Uninstall", &[])

    }

}

