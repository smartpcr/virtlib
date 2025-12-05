// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEAuthenticodeCertificate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEAuthenticodeCertificate {

/// 
    #[serde(rename = "certIndex")]
    pub cert_index: Option<i32>,

/// 
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,

/// 
    #[serde(rename = "friendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "intendedPurposes")]
    pub intended_purposes: Option<String>,

/// 
    #[serde(rename = "issuerName")]
    pub issuer_name: Option<String>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<i32>,

/// 
    #[serde(rename = "subjectName")]
    pub subject_name: Option<String>,

/// 
    #[serde(rename = "tabIndex")]
    pub tab_index: Option<i32>,
}

impl RSOP_IEAuthenticodeCertificate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cert_index: None,
            expiration_date: None,
            friendly_name: None,
            intended_purposes: None,
            issuer_name: None,
            rsop_id: None,
            rsop_precedence: None,
            subject_name: None,
            tab_index: None,
        }
    }


    /// Sets the value of certIndex
    pub fn set_cert_index(&mut self, value: i32) {
        self.cert_index = Some(value);
    }

    /// Gets the value of certIndex
    pub fn get_cert_index(&self) -> Option<&i32> {
        self.cert_index.as_ref()
    }

    /// Sets the value of expirationDate
    pub fn set_expiration_date(&mut self, value: String) {
        self.expiration_date = Some(value);
    }

    /// Gets the value of expirationDate
    pub fn get_expiration_date(&self) -> Option<&String> {
        self.expiration_date.as_ref()
    }

    /// Sets the value of friendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of friendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of intendedPurposes
    pub fn set_intended_purposes(&mut self, value: String) {
        self.intended_purposes = Some(value);
    }

    /// Gets the value of intendedPurposes
    pub fn get_intended_purposes(&self) -> Option<&String> {
        self.intended_purposes.as_ref()
    }

    /// Sets the value of issuerName
    pub fn set_issuer_name(&mut self, value: String) {
        self.issuer_name = Some(value);
    }

    /// Gets the value of issuerName
    pub fn get_issuer_name(&self) -> Option<&String> {
        self.issuer_name.as_ref()
    }

    /// Sets the value of rsopID
    pub fn set_rsop_id(&mut self, value: String) {
        self.rsop_id = Some(value);
    }

    /// Gets the value of rsopID
    pub fn get_rsop_id(&self) -> Option<&String> {
        self.rsop_id.as_ref()
    }

    /// Sets the value of rsopPrecedence
    pub fn set_rsop_precedence(&mut self, value: i32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&i32> {
        self.rsop_precedence.as_ref()
    }

    /// Sets the value of subjectName
    pub fn set_subject_name(&mut self, value: String) {
        self.subject_name = Some(value);
    }

    /// Gets the value of subjectName
    pub fn get_subject_name(&self) -> Option<&String> {
        self.subject_name.as_ref()
    }

    /// Sets the value of tabIndex
    pub fn set_tab_index(&mut self, value: i32) {
        self.tab_index = Some(value);
    }

    /// Gets the value of tabIndex
    pub fn get_tab_index(&self) -> Option<&i32> {
        self.tab_index.as_ref()
    }
}

