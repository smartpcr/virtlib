// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ProtocolEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ProtocolEndpoint {
    #[serde(flatten)]
    pub base: CIM_ServiceAccessPoint,

/// 
    #[serde(rename = "NameFormat")]
    pub name_format: Option<String>,

/// 
    #[serde(rename = "OtherTypeDescription")]
    pub other_type_description: Option<String>,

/// 
    #[serde(rename = "ProtocolIFType")]
    pub protocol_iftype: Option<u16>,

/// 
    #[serde(rename = "ProtocolType")]
    pub protocol_type: Option<u16>,
}

impl CIM_ProtocolEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ServiceAccessPoint::new(),
            name_format: None,
            other_type_description: None,
            protocol_iftype: None,
            protocol_type: None,
        }
    }


    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: String) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&String> {
        self.name_format.as_ref()
    }

    /// Sets the value of OtherTypeDescription
    pub fn set_other_type_description(&mut self, value: String) {
        self.other_type_description = Some(value);
    }

    /// Gets the value of OtherTypeDescription
    pub fn get_other_type_description(&self) -> Option<&String> {
        self.other_type_description.as_ref()
    }

    /// Sets the value of ProtocolIFType
    pub fn set_protocol_iftype(&mut self, value: u16) {
        self.protocol_iftype = Some(value);
    }

    /// Gets the value of ProtocolIFType
    pub fn get_protocol_iftype(&self) -> Option<&u16> {
        self.protocol_iftype.as_ref()
    }

    /// Sets the value of ProtocolType
    pub fn set_protocol_type(&mut self, value: u16) {
        self.protocol_type = Some(value);
    }

    /// Gets the value of ProtocolType
    pub fn get_protocol_type(&self) -> Option<&u16> {
        self.protocol_type.as_ref()
    }
}

