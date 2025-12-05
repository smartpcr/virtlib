// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PeerResolverBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerResolverBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// Determines how referrals are shared among peers.
    #[serde(rename = "ReferralPolicy")]
    pub referral_policy: Option<String>,
}

impl PeerResolverBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            referral_policy: None,
        }
    }


    /// Sets the value of ReferralPolicy
    pub fn set_referral_policy(&mut self, value: String) {
        self.referral_policy = Some(value);
    }

    /// Gets the value of ReferralPolicy
    pub fn get_referral_policy(&self) -> Option<&String> {
        self.referral_policy.as_ref()
    }
}

