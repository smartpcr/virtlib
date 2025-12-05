// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ObjectProviderRegistration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ObjectProviderRegistration {
    #[serde(flatten)]
    pub base: __ProviderRegistration,

/// 
    #[serde(rename = "InteractionType")]
    pub interaction_type: Option<ObjectProviderRegistration_InteractionType>,

/// 
    #[serde(rename = "QuerySupportLevels")]
    pub query_support_levels: Vec<String>,

/// 
    #[serde(rename = "SupportsBatching")]
    pub supports_batching: Option<bool>,

/// 
    #[serde(rename = "SupportsDelete")]
    pub supports_delete: Option<bool>,

/// 
    #[serde(rename = "SupportsEnumeration")]
    pub supports_enumeration: Option<bool>,

/// 
    #[serde(rename = "SupportsGet")]
    pub supports_get: Option<bool>,

/// 
    #[serde(rename = "SupportsPut")]
    pub supports_put: Option<bool>,

/// 
    #[serde(rename = "SupportsTransactions")]
    pub supports_transactions: Option<bool>,
}

impl __ObjectProviderRegistration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ProviderRegistration::new(),
            interaction_type: None,
            query_support_levels: Vec::new(),
            supports_batching: None,
            supports_delete: None,
            supports_enumeration: None,
            supports_get: None,
            supports_put: None,
            supports_transactions: None,
        }
    }


    /// Sets the value of InteractionType
    pub fn set_interaction_type(&mut self, value: ObjectProviderRegistration_InteractionType) {
        self.interaction_type = Some(value);
    }

    /// Gets the value of InteractionType
    pub fn get_interaction_type(&self) -> Option<&ObjectProviderRegistration_InteractionType> {
        self.interaction_type.as_ref()
    }

    /// Sets the value of QuerySupportLevels
    pub fn set_query_support_levels(&mut self, value: Vec<String>) {
        self.query_support_levels = value;
    }

    /// Gets the value of QuerySupportLevels
    pub fn get_query_support_levels(&self) -> &Vec<String> {
        &self.query_support_levels
    }

    /// Sets the value of SupportsBatching
    pub fn set_supports_batching(&mut self, value: bool) {
        self.supports_batching = Some(value);
    }

    /// Gets the value of SupportsBatching
    pub fn get_supports_batching(&self) -> Option<&bool> {
        self.supports_batching.as_ref()
    }

    /// Sets the value of SupportsDelete
    pub fn set_supports_delete(&mut self, value: bool) {
        self.supports_delete = Some(value);
    }

    /// Gets the value of SupportsDelete
    pub fn get_supports_delete(&self) -> Option<&bool> {
        self.supports_delete.as_ref()
    }

    /// Sets the value of SupportsEnumeration
    pub fn set_supports_enumeration(&mut self, value: bool) {
        self.supports_enumeration = Some(value);
    }

    /// Gets the value of SupportsEnumeration
    pub fn get_supports_enumeration(&self) -> Option<&bool> {
        self.supports_enumeration.as_ref()
    }

    /// Sets the value of SupportsGet
    pub fn set_supports_get(&mut self, value: bool) {
        self.supports_get = Some(value);
    }

    /// Gets the value of SupportsGet
    pub fn get_supports_get(&self) -> Option<&bool> {
        self.supports_get.as_ref()
    }

    /// Sets the value of SupportsPut
    pub fn set_supports_put(&mut self, value: bool) {
        self.supports_put = Some(value);
    }

    /// Gets the value of SupportsPut
    pub fn get_supports_put(&self) -> Option<&bool> {
        self.supports_put.as_ref()
    }

    /// Sets the value of SupportsTransactions
    pub fn set_supports_transactions(&mut self, value: bool) {
        self.supports_transactions = Some(value);
    }

    /// Gets the value of SupportsTransactions
    pub fn get_supports_transactions(&self) -> Option<&bool> {
        self.supports_transactions.as_ref()
    }
}

