// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// IdleStateAccountingEx struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdleStateAccountingEx {

/// 
    #[serde(rename = "FailedTransitions")]
    pub failed_transitions: Option<u32>,

/// 
    #[serde(rename = "IdleTimeBuckets")]
    pub idle_time_buckets: Vec<IdleStateBucketEx>,

/// 
    #[serde(rename = "IdleTransitions")]
    pub idle_transitions: Option<u32>,

/// 
    #[serde(rename = "InvalidBucketIndex")]
    pub invalid_bucket_index: Option<u32>,

/// 
    #[serde(rename = "MaxTimeUs")]
    pub max_time_us: Option<u32>,

/// 
    #[serde(rename = "MinTimeUs")]
    pub min_time_us: Option<u32>,

/// 
    #[serde(rename = "TotalTime")]
    pub total_time: Option<u64>,
}

impl IdleStateAccountingEx {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            failed_transitions: None,
            idle_time_buckets: Vec::new(),
            idle_transitions: None,
            invalid_bucket_index: None,
            max_time_us: None,
            min_time_us: None,
            total_time: None,
        }
    }


    /// Sets the value of FailedTransitions
    pub fn set_failed_transitions(&mut self, value: u32) {
        self.failed_transitions = Some(value);
    }

    /// Gets the value of FailedTransitions
    pub fn get_failed_transitions(&self) -> Option<&u32> {
        self.failed_transitions.as_ref()
    }

    /// Sets the value of IdleTimeBuckets
    pub fn set_idle_time_buckets(&mut self, value: Vec<IdleStateBucketEx>) {
        self.idle_time_buckets = value;
    }

    /// Gets the value of IdleTimeBuckets
    pub fn get_idle_time_buckets(&self) -> &Vec<IdleStateBucketEx> {
        &self.idle_time_buckets
    }

    /// Sets the value of IdleTransitions
    pub fn set_idle_transitions(&mut self, value: u32) {
        self.idle_transitions = Some(value);
    }

    /// Gets the value of IdleTransitions
    pub fn get_idle_transitions(&self) -> Option<&u32> {
        self.idle_transitions.as_ref()
    }

    /// Sets the value of InvalidBucketIndex
    pub fn set_invalid_bucket_index(&mut self, value: u32) {
        self.invalid_bucket_index = Some(value);
    }

    /// Gets the value of InvalidBucketIndex
    pub fn get_invalid_bucket_index(&self) -> Option<&u32> {
        self.invalid_bucket_index.as_ref()
    }

    /// Sets the value of MaxTimeUs
    pub fn set_max_time_us(&mut self, value: u32) {
        self.max_time_us = Some(value);
    }

    /// Gets the value of MaxTimeUs
    pub fn get_max_time_us(&self) -> Option<&u32> {
        self.max_time_us.as_ref()
    }

    /// Sets the value of MinTimeUs
    pub fn set_min_time_us(&mut self, value: u32) {
        self.min_time_us = Some(value);
    }

    /// Gets the value of MinTimeUs
    pub fn get_min_time_us(&self) -> Option<&u32> {
        self.min_time_us.as_ref()
    }

    /// Sets the value of TotalTime
    pub fn set_total_time(&mut self, value: u64) {
        self.total_time = Some(value);
    }

    /// Gets the value of TotalTime
    pub fn get_total_time(&self) -> Option<&u64> {
        self.total_time.as_ref()
    }
}

