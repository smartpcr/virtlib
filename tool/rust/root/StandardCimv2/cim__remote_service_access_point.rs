// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RemoteServiceAccessPoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RemoteServiceAccessPoint {
    #[serde(flatten)]
    pub base: CIM_ServiceAccessPoint,

/// 
    #[serde(rename = "AccessContext")]
    pub access_context: Option<u16>,

/// 
    #[serde(rename = "AccessInfo")]
    pub access_info: Option<String>,

/// 
    #[serde(rename = "InfoFormat")]
    pub info_format: Option<u16>,

/// 
    #[serde(rename = "OtherAccessContext")]
    pub other_access_context: Option<String>,

/// 
    #[serde(rename = "OtherInfoFormatDescription")]
    pub other_info_format_description: Option<String>,
}

impl CIM_RemoteServiceAccessPoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ServiceAccessPoint::new(),
            access_context: None,
            access_info: None,
            info_format: None,
            other_access_context: None,
            other_info_format_description: None,
        }
    }


    /// Sets the value of AccessContext
    pub fn set_access_context(&mut self, value: u16) {
        self.access_context = Some(value);
    }

    /// Gets the value of AccessContext
    pub fn get_access_context(&self) -> Option<&u16> {
        self.access_context.as_ref()
    }

    /// Sets the value of AccessInfo
    pub fn set_access_info(&mut self, value: String) {
        self.access_info = Some(value);
    }

    /// Gets the value of AccessInfo
    pub fn get_access_info(&self) -> Option<&String> {
        self.access_info.as_ref()
    }

    /// Sets the value of InfoFormat
    pub fn set_info_format(&mut self, value: u16) {
        self.info_format = Some(value);
    }

    /// Gets the value of InfoFormat
    pub fn get_info_format(&self) -> Option<&u16> {
        self.info_format.as_ref()
    }

    /// Sets the value of OtherAccessContext
    pub fn set_other_access_context(&mut self, value: String) {
        self.other_access_context = Some(value);
    }

    /// Gets the value of OtherAccessContext
    pub fn get_other_access_context(&self) -> Option<&String> {
        self.other_access_context.as_ref()
    }

    /// Sets the value of OtherInfoFormatDescription
    pub fn set_other_info_format_description(&mut self, value: String) {
        self.other_info_format_description = Some(value);
    }

    /// Gets the value of OtherInfoFormatDescription
    pub fn get_other_info_format_description(&self) -> Option<&String> {
        self.other_info_format_description.as_ref()
    }
}

