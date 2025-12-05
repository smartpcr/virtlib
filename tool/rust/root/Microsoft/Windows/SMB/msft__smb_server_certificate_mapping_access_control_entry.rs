// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbServerCertificateMappingAccessControlEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbServerCertificateMappingAccessControlEntry {

/// 
    #[serde(rename = "AccessControlType")]
    pub access_control_type: Option<SmbServerCertificateMappingAccessControlEntry_AccessControlType>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Identifier")]
    pub identifier: Option<String>,

/// 
    #[serde(rename = "IdentifierType")]
    pub identifier_type: Option<SmbServerCertificateMappingAccessControlEntry_IdentifierType>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_SmbServerCertificateMappingAccessControlEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_control_type: None,
            description: None,
            identifier: None,
            identifier_type: None,
            name: None,
        }
    }


    /// Sets the value of AccessControlType
    pub fn set_access_control_type(&mut self, value: SmbServerCertificateMappingAccessControlEntry_AccessControlType) {
        self.access_control_type = Some(value);
    }

    /// Gets the value of AccessControlType
    pub fn get_access_control_type(&self) -> Option<&SmbServerCertificateMappingAccessControlEntry_AccessControlType> {
        self.access_control_type.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Identifier
    pub fn set_identifier(&mut self, value: String) {
        self.identifier = Some(value);
    }

    /// Gets the value of Identifier
    pub fn get_identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    /// Sets the value of IdentifierType
    pub fn set_identifier_type(&mut self, value: SmbServerCertificateMappingAccessControlEntry_IdentifierType) {
        self.identifier_type = Some(value);
    }

    /// Gets the value of IdentifierType
    pub fn get_identifier_type(&self) -> Option<&SmbServerCertificateMappingAccessControlEntry_IdentifierType> {
        self.identifier_type.as_ref()
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

