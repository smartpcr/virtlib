// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SupportAccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SupportAccess {

/// 
    #[serde(rename = "CommunicationInfo")]
    pub communication_info: Option<String>,

/// 
    #[serde(rename = "CommunicationMode")]
    pub communication_mode: Option<u16>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Locale")]
    pub locale: Option<String>,

/// 
    #[serde(rename = "SupportAccessId")]
    pub support_access_id: Option<String>,
}

impl CIM_SupportAccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            communication_info: None,
            communication_mode: None,
            description: None,
            locale: None,
            support_access_id: None,
        }
    }


    /// Sets the value of CommunicationInfo
    pub fn set_communication_info(&mut self, value: String) {
        self.communication_info = Some(value);
    }

    /// Gets the value of CommunicationInfo
    pub fn get_communication_info(&self) -> Option<&String> {
        self.communication_info.as_ref()
    }

    /// Sets the value of CommunicationMode
    pub fn set_communication_mode(&mut self, value: u16) {
        self.communication_mode = Some(value);
    }

    /// Gets the value of CommunicationMode
    pub fn get_communication_mode(&self) -> Option<&u16> {
        self.communication_mode.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Locale
    pub fn set_locale(&mut self, value: String) {
        self.locale = Some(value);
    }

    /// Gets the value of Locale
    pub fn get_locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }

    /// Sets the value of SupportAccessId
    pub fn set_support_access_id(&mut self, value: String) {
        self.support_access_id = Some(value);
    }

    /// Gets the value of SupportAccessId
    pub fn get_support_access_id(&self) -> Option<&String> {
        self.support_access_id.as_ref()
    }
}

