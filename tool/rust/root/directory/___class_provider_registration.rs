// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ClassProviderRegistration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ClassProviderRegistration {
    #[serde(flatten)]
    pub base: __ObjectProviderRegistration,

/// 
    #[serde(rename = "CacheRefreshInterval")]
    pub cache_refresh_interval: Option<String>,

/// 
    #[serde(rename = "PerUserSchema")]
    pub per_user_schema: Option<bool>,

/// 
    #[serde(rename = "ReferencedSetQueries")]
    pub referenced_set_queries: Vec<String>,

/// 
    #[serde(rename = "ResultSetQueries")]
    pub result_set_queries: Vec<String>,

/// 
    #[serde(rename = "ReSynchroniseOnNamespaceOpen")]
    pub re_synchronise_on_namespace_open: Option<bool>,

/// 
    #[serde(rename = "UnsupportedQueries")]
    pub unsupported_queries: Vec<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<u32>,
}

impl __ClassProviderRegistration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ObjectProviderRegistration::new(),
            cache_refresh_interval: None,
            per_user_schema: None,
            referenced_set_queries: Vec::new(),
            result_set_queries: Vec::new(),
            re_synchronise_on_namespace_open: None,
            unsupported_queries: Vec::new(),
            version: None,
        }
    }


    /// Sets the value of CacheRefreshInterval
    pub fn set_cache_refresh_interval(&mut self, value: String) {
        self.cache_refresh_interval = Some(value);
    }

    /// Gets the value of CacheRefreshInterval
    pub fn get_cache_refresh_interval(&self) -> Option<&String> {
        self.cache_refresh_interval.as_ref()
    }

    /// Sets the value of PerUserSchema
    pub fn set_per_user_schema(&mut self, value: bool) {
        self.per_user_schema = Some(value);
    }

    /// Gets the value of PerUserSchema
    pub fn get_per_user_schema(&self) -> Option<&bool> {
        self.per_user_schema.as_ref()
    }

    /// Sets the value of ReferencedSetQueries
    pub fn set_referenced_set_queries(&mut self, value: Vec<String>) {
        self.referenced_set_queries = value;
    }

    /// Gets the value of ReferencedSetQueries
    pub fn get_referenced_set_queries(&self) -> &Vec<String> {
        &self.referenced_set_queries
    }

    /// Sets the value of ResultSetQueries
    pub fn set_result_set_queries(&mut self, value: Vec<String>) {
        self.result_set_queries = value;
    }

    /// Gets the value of ResultSetQueries
    pub fn get_result_set_queries(&self) -> &Vec<String> {
        &self.result_set_queries
    }

    /// Sets the value of ReSynchroniseOnNamespaceOpen
    pub fn set_re_synchronise_on_namespace_open(&mut self, value: bool) {
        self.re_synchronise_on_namespace_open = Some(value);
    }

    /// Gets the value of ReSynchroniseOnNamespaceOpen
    pub fn get_re_synchronise_on_namespace_open(&self) -> Option<&bool> {
        self.re_synchronise_on_namespace_open.as_ref()
    }

    /// Sets the value of UnsupportedQueries
    pub fn set_unsupported_queries(&mut self, value: Vec<String>) {
        self.unsupported_queries = value;
    }

    /// Gets the value of UnsupportedQueries
    pub fn get_unsupported_queries(&self) -> &Vec<String> {
        &self.unsupported_queries
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: u32) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&u32> {
        self.version.as_ref()
    }
}

