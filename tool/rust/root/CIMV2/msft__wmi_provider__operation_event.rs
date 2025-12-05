// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_WmiProvider_OperationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_WmiProvider_OperationEvent {
    #[serde(flatten)]
    pub base: MSFT_WmiSelfEvent,

/// 
    #[serde(rename = "HostingGroup")]
    pub hosting_group: Option<String>,

/// 
    #[serde(rename = "HostingSpecification")]
    pub hosting_specification: Option<u32>,

/// 
    #[serde(rename = "Locale")]
    pub locale: Option<String>,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// 
    #[serde(rename = "provider")]
    pub provider: Option<String>,

/// 
    #[serde(rename = "TransactionIdentifer")]
    pub transaction_identifer: Option<String>,

/// 
    #[serde(rename = "User")]
    pub user: Option<String>,
}

impl Msft_WmiProvider_OperationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_WmiSelfEvent::new(),
            hosting_group: None,
            hosting_specification: None,
            locale: None,
            namespace: None,
            provider: None,
            transaction_identifer: None,
            user: None,
        }
    }


    /// Sets the value of HostingGroup
    pub fn set_hosting_group(&mut self, value: String) {
        self.hosting_group = Some(value);
    }

    /// Gets the value of HostingGroup
    pub fn get_hosting_group(&self) -> Option<&String> {
        self.hosting_group.as_ref()
    }

    /// Sets the value of HostingSpecification
    pub fn set_hosting_specification(&mut self, value: u32) {
        self.hosting_specification = Some(value);
    }

    /// Gets the value of HostingSpecification
    pub fn get_hosting_specification(&self) -> Option<&u32> {
        self.hosting_specification.as_ref()
    }

    /// Sets the value of Locale
    pub fn set_locale(&mut self, value: String) {
        self.locale = Some(value);
    }

    /// Gets the value of Locale
    pub fn get_locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }

    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    /// Sets the value of provider
    pub fn set_provider(&mut self, value: String) {
        self.provider = Some(value);
    }

    /// Gets the value of provider
    pub fn get_provider(&self) -> Option<&String> {
        self.provider.as_ref()
    }

    /// Sets the value of TransactionIdentifer
    pub fn set_transaction_identifer(&mut self, value: String) {
        self.transaction_identifer = Some(value);
    }

    /// Gets the value of TransactionIdentifer
    pub fn get_transaction_identifer(&self) -> Option<&String> {
        self.transaction_identifer.as_ref()
    }

    /// Sets the value of User
    pub fn set_user(&mut self, value: String) {
        self.user = Some(value);
    }

    /// Gets the value of User
    pub fn get_user(&self) -> Option<&String> {
        self.user.as_ref()
    }
}

