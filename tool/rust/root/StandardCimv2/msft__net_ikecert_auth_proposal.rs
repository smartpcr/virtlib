// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIKECertAuthProposal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIKECertAuthProposal {
    #[serde(flatten)]
    pub base: MSFT_NetIKEAuthProposal,

/// 
    #[serde(rename = "CertName")]
    pub cert_name: Option<String>,

/// 
    #[serde(rename = "CertNameType")]
    pub cert_name_type: Option<u16>,

/// 
    #[serde(rename = "EKUs")]
    pub ekus: Vec<String>,

/// 
    #[serde(rename = "ExcludeCAName")]
    pub exclude_caname: Option<bool>,

/// 
    #[serde(rename = "FollowRenewal")]
    pub follow_renewal: Option<bool>,

/// 
    #[serde(rename = "MapToAccount")]
    pub map_to_account: Option<bool>,

/// 
    #[serde(rename = "SelectionCriteria")]
    pub selection_criteria: Option<bool>,

/// 
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: Option<u16>,

/// 
    #[serde(rename = "Thumbprint")]
    pub thumbprint: Option<String>,

/// 
    #[serde(rename = "TrustedCA")]
    pub trusted_ca: Option<String>,

/// 
    #[serde(rename = "TrustedCAType")]
    pub trusted_catype: Option<u16>,

/// 
    #[serde(rename = "ValidationCriteria")]
    pub validation_criteria: Option<bool>,
}

impl MSFT_NetIKECertAuthProposal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetIKEAuthProposal::new(),
            cert_name: None,
            cert_name_type: None,
            ekus: Vec::new(),
            exclude_caname: None,
            follow_renewal: None,
            map_to_account: None,
            selection_criteria: None,
            signing_algorithm: None,
            thumbprint: None,
            trusted_ca: None,
            trusted_catype: None,
            validation_criteria: None,
        }
    }


    /// Sets the value of CertName
    pub fn set_cert_name(&mut self, value: String) {
        self.cert_name = Some(value);
    }

    /// Gets the value of CertName
    pub fn get_cert_name(&self) -> Option<&String> {
        self.cert_name.as_ref()
    }

    /// Sets the value of CertNameType
    pub fn set_cert_name_type(&mut self, value: u16) {
        self.cert_name_type = Some(value);
    }

    /// Gets the value of CertNameType
    pub fn get_cert_name_type(&self) -> Option<&u16> {
        self.cert_name_type.as_ref()
    }

    /// Sets the value of EKUs
    pub fn set_ekus(&mut self, value: Vec<String>) {
        self.ekus = value;
    }

    /// Gets the value of EKUs
    pub fn get_ekus(&self) -> &Vec<String> {
        &self.ekus
    }

    /// Sets the value of ExcludeCAName
    pub fn set_exclude_caname(&mut self, value: bool) {
        self.exclude_caname = Some(value);
    }

    /// Gets the value of ExcludeCAName
    pub fn get_exclude_caname(&self) -> Option<&bool> {
        self.exclude_caname.as_ref()
    }

    /// Sets the value of FollowRenewal
    pub fn set_follow_renewal(&mut self, value: bool) {
        self.follow_renewal = Some(value);
    }

    /// Gets the value of FollowRenewal
    pub fn get_follow_renewal(&self) -> Option<&bool> {
        self.follow_renewal.as_ref()
    }

    /// Sets the value of MapToAccount
    pub fn set_map_to_account(&mut self, value: bool) {
        self.map_to_account = Some(value);
    }

    /// Gets the value of MapToAccount
    pub fn get_map_to_account(&self) -> Option<&bool> {
        self.map_to_account.as_ref()
    }

    /// Sets the value of SelectionCriteria
    pub fn set_selection_criteria(&mut self, value: bool) {
        self.selection_criteria = Some(value);
    }

    /// Gets the value of SelectionCriteria
    pub fn get_selection_criteria(&self) -> Option<&bool> {
        self.selection_criteria.as_ref()
    }

    /// Sets the value of SigningAlgorithm
    pub fn set_signing_algorithm(&mut self, value: u16) {
        self.signing_algorithm = Some(value);
    }

    /// Gets the value of SigningAlgorithm
    pub fn get_signing_algorithm(&self) -> Option<&u16> {
        self.signing_algorithm.as_ref()
    }

    /// Sets the value of Thumbprint
    pub fn set_thumbprint(&mut self, value: String) {
        self.thumbprint = Some(value);
    }

    /// Gets the value of Thumbprint
    pub fn get_thumbprint(&self) -> Option<&String> {
        self.thumbprint.as_ref()
    }

    /// Sets the value of TrustedCA
    pub fn set_trusted_ca(&mut self, value: String) {
        self.trusted_ca = Some(value);
    }

    /// Gets the value of TrustedCA
    pub fn get_trusted_ca(&self) -> Option<&String> {
        self.trusted_ca.as_ref()
    }

    /// Sets the value of TrustedCAType
    pub fn set_trusted_catype(&mut self, value: u16) {
        self.trusted_catype = Some(value);
    }

    /// Gets the value of TrustedCAType
    pub fn get_trusted_catype(&self) -> Option<&u16> {
        self.trusted_catype.as_ref()
    }

    /// Sets the value of ValidationCriteria
    pub fn set_validation_criteria(&mut self, value: bool) {
        self.validation_criteria = Some(value);
    }

    /// Gets the value of ValidationCriteria
    pub fn get_validation_criteria(&self) -> Option<&bool> {
        self.validation_criteria.as_ref()
    }
}

