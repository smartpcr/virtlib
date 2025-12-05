// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SymmetricSecurityBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymmetricSecurityBindingElement {
    #[serde(flatten)]
    pub base: SecurityBindingElement,

/// The order of message encryption and signing for this binding.
    #[serde(rename = "MessageProtectionOrder")]
    pub message_protection_order: Option<String>,

/// Whether the binding requires signature confirmation.
    #[serde(rename = "RequireSignatureConfirmation")]
    pub require_signature_confirmation: Option<bool>,
}

impl SymmetricSecurityBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SecurityBindingElement::new(),
            message_protection_order: None,
            require_signature_confirmation: None,
        }
    }


    /// Sets the value of MessageProtectionOrder
    pub fn set_message_protection_order(&mut self, value: String) {
        self.message_protection_order = Some(value);
    }

    /// Gets the value of MessageProtectionOrder
    pub fn get_message_protection_order(&self) -> Option<&String> {
        self.message_protection_order.as_ref()
    }

    /// Sets the value of RequireSignatureConfirmation
    pub fn set_require_signature_confirmation(&mut self, value: bool) {
        self.require_signature_confirmation = Some(value);
    }

    /// Gets the value of RequireSignatureConfirmation
    pub fn get_require_signature_confirmation(&self) -> Option<&bool> {
        self.require_signature_confirmation.as_ref()
    }
}

