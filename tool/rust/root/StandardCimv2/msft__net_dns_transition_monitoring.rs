// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetDnsTransitionMonitoring struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetDnsTransitionMonitoring {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "NumAAAAQueriesFailed")]
    pub num_aaaaqueries_failed: Option<u32>,

/// 
    #[serde(rename = "NumAAAAQueriesIn6ArpaPtr")]
    pub num_aaaaqueries_in6_arpa_ptr: Option<u32>,

/// 
    #[serde(rename = "NumAAAAQueriesSucceeded")]
    pub num_aaaaqueries_succeeded: Option<u32>,

/// 
    #[serde(rename = "NumAAAAQueriesSynthesized")]
    pub num_aaaaqueries_synthesized: Option<u32>,

/// 
    #[serde(rename = "NumOtherQueriesFailed")]
    pub num_other_queries_failed: Option<u32>,

/// 
    #[serde(rename = "NumOtherQueriesSucceeded")]
    pub num_other_queries_succeeded: Option<u32>,
}

impl MSFT_NetDnsTransitionMonitoring {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            num_aaaaqueries_failed: None,
            num_aaaaqueries_in6_arpa_ptr: None,
            num_aaaaqueries_succeeded: None,
            num_aaaaqueries_synthesized: None,
            num_other_queries_failed: None,
            num_other_queries_succeeded: None,
        }
    }


    /// Sets the value of NumAAAAQueriesFailed
    pub fn set_num_aaaaqueries_failed(&mut self, value: u32) {
        self.num_aaaaqueries_failed = Some(value);
    }

    /// Gets the value of NumAAAAQueriesFailed
    pub fn get_num_aaaaqueries_failed(&self) -> Option<&u32> {
        self.num_aaaaqueries_failed.as_ref()
    }

    /// Sets the value of NumAAAAQueriesIn6ArpaPtr
    pub fn set_num_aaaaqueries_in6_arpa_ptr(&mut self, value: u32) {
        self.num_aaaaqueries_in6_arpa_ptr = Some(value);
    }

    /// Gets the value of NumAAAAQueriesIn6ArpaPtr
    pub fn get_num_aaaaqueries_in6_arpa_ptr(&self) -> Option<&u32> {
        self.num_aaaaqueries_in6_arpa_ptr.as_ref()
    }

    /// Sets the value of NumAAAAQueriesSucceeded
    pub fn set_num_aaaaqueries_succeeded(&mut self, value: u32) {
        self.num_aaaaqueries_succeeded = Some(value);
    }

    /// Gets the value of NumAAAAQueriesSucceeded
    pub fn get_num_aaaaqueries_succeeded(&self) -> Option<&u32> {
        self.num_aaaaqueries_succeeded.as_ref()
    }

    /// Sets the value of NumAAAAQueriesSynthesized
    pub fn set_num_aaaaqueries_synthesized(&mut self, value: u32) {
        self.num_aaaaqueries_synthesized = Some(value);
    }

    /// Gets the value of NumAAAAQueriesSynthesized
    pub fn get_num_aaaaqueries_synthesized(&self) -> Option<&u32> {
        self.num_aaaaqueries_synthesized.as_ref()
    }

    /// Sets the value of NumOtherQueriesFailed
    pub fn set_num_other_queries_failed(&mut self, value: u32) {
        self.num_other_queries_failed = Some(value);
    }

    /// Gets the value of NumOtherQueriesFailed
    pub fn get_num_other_queries_failed(&self) -> Option<&u32> {
        self.num_other_queries_failed.as_ref()
    }

    /// Sets the value of NumOtherQueriesSucceeded
    pub fn set_num_other_queries_succeeded(&mut self, value: u32) {
        self.num_other_queries_succeeded = Some(value);
    }

    /// Gets the value of NumOtherQueriesSucceeded
    pub fn get_num_other_queries_succeeded(&self) -> Option<&u32> {
        self.num_other_queries_succeeded.as_ref()
    }
}

