// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SpinLockConfig_V3 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpinLockConfig_V3 {
    #[serde(flatten)]
    pub base: PerfInfo,

/// 
    #[serde(rename = "SpinLockAcquireSampleRate")]
    pub spin_lock_acquire_sample_rate: Option<u32>,

/// 
    #[serde(rename = "SpinLockContentionSampleRate")]
    pub spin_lock_contention_sample_rate: Option<u32>,

/// 
    #[serde(rename = "SpinLockHoldThreshold")]
    pub spin_lock_hold_threshold: Option<u32>,

/// 
    #[serde(rename = "SpinLockSpinThreshold")]
    pub spin_lock_spin_threshold: Option<u32>,
}

impl SpinLockConfig_V3 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo::new(),
            spin_lock_acquire_sample_rate: None,
            spin_lock_contention_sample_rate: None,
            spin_lock_hold_threshold: None,
            spin_lock_spin_threshold: None,
        }
    }


    /// Sets the value of SpinLockAcquireSampleRate
    pub fn set_spin_lock_acquire_sample_rate(&mut self, value: u32) {
        self.spin_lock_acquire_sample_rate = Some(value);
    }

    /// Gets the value of SpinLockAcquireSampleRate
    pub fn get_spin_lock_acquire_sample_rate(&self) -> Option<&u32> {
        self.spin_lock_acquire_sample_rate.as_ref()
    }

    /// Sets the value of SpinLockContentionSampleRate
    pub fn set_spin_lock_contention_sample_rate(&mut self, value: u32) {
        self.spin_lock_contention_sample_rate = Some(value);
    }

    /// Gets the value of SpinLockContentionSampleRate
    pub fn get_spin_lock_contention_sample_rate(&self) -> Option<&u32> {
        self.spin_lock_contention_sample_rate.as_ref()
    }

    /// Sets the value of SpinLockHoldThreshold
    pub fn set_spin_lock_hold_threshold(&mut self, value: u32) {
        self.spin_lock_hold_threshold = Some(value);
    }

    /// Gets the value of SpinLockHoldThreshold
    pub fn get_spin_lock_hold_threshold(&self) -> Option<&u32> {
        self.spin_lock_hold_threshold.as_ref()
    }

    /// Sets the value of SpinLockSpinThreshold
    pub fn set_spin_lock_spin_threshold(&mut self, value: u32) {
        self.spin_lock_spin_threshold = Some(value);
    }

    /// Gets the value of SpinLockSpinThreshold
    pub fn get_spin_lock_spin_threshold(&self) -> Option<&u32> {
        self.spin_lock_spin_threshold.as_ref()
    }
}

