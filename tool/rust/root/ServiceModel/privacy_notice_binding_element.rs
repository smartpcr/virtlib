// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PrivacyNoticeBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrivacyNoticeBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// The privacy notice version.
    #[serde(rename = "PrivacyNoticeVersion")]
    pub privacy_notice_version: Option<i32>,

/// The URI at which the privacy notice is located.
    #[serde(rename = "Url")]
    pub url: Option<String>,
}

impl PrivacyNoticeBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            privacy_notice_version: None,
            url: None,
        }
    }


    /// Sets the value of PrivacyNoticeVersion
    pub fn set_privacy_notice_version(&mut self, value: i32) {
        self.privacy_notice_version = Some(value);
    }

    /// Gets the value of PrivacyNoticeVersion
    pub fn get_privacy_notice_version(&self) -> Option<&i32> {
        self.privacy_notice_version.as_ref()
    }

    /// Sets the value of Url
    pub fn set_url(&mut self, value: String) {
        self.url = Some(value);
    }

    /// Gets the value of Url
    pub fn get_url(&self) -> Option<&String> {
        self.url.as_ref()
    }
}

