// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ContextBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// The ContextExchangeMechanism for ContextBindingElement
    #[serde(rename = "ContextExchangeMechanism")]
    pub context_exchange_mechanism: Option<String>,

/// Whether automatic context management is enabled for ContextBindingElement
    #[serde(rename = "ContextManagementEnabled")]
    pub context_management_enabled: Option<bool>,

/// The ProtectionLevel forContextBindingElement
    #[serde(rename = "ProtectionLevel")]
    pub protection_level: Option<String>,
}

impl ContextBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            context_exchange_mechanism: None,
            context_management_enabled: None,
            protection_level: None,
        }
    }


    /// Sets the value of ContextExchangeMechanism
    pub fn set_context_exchange_mechanism(&mut self, value: String) {
        self.context_exchange_mechanism = Some(value);
    }

    /// Gets the value of ContextExchangeMechanism
    pub fn get_context_exchange_mechanism(&self) -> Option<&String> {
        self.context_exchange_mechanism.as_ref()
    }

    /// Sets the value of ContextManagementEnabled
    pub fn set_context_management_enabled(&mut self, value: bool) {
        self.context_management_enabled = Some(value);
    }

    /// Gets the value of ContextManagementEnabled
    pub fn get_context_management_enabled(&self) -> Option<&bool> {
        self.context_management_enabled.as_ref()
    }

    /// Sets the value of ProtectionLevel
    pub fn set_protection_level(&mut self, value: String) {
        self.protection_level = Some(value);
    }

    /// Gets the value of ProtectionLevel
    pub fn get_protection_level(&self) -> Option<&String> {
        self.protection_level.as_ref()
    }
}

