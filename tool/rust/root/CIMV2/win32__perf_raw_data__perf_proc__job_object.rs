// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PerfProc_JobObject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PerfProc_JobObject {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "CurrentPercentKernelModeTime")]
    pub current_percent_kernel_mode_time: Option<u64>,

/// 
    #[serde(rename = "CurrentPercentProcessorTime")]
    pub current_percent_processor_time: Option<u64>,

/// 
    #[serde(rename = "CurrentPercentUserModeTime")]
    pub current_percent_user_mode_time: Option<u64>,

/// 
    #[serde(rename = "PagesPerSec")]
    pub pages_per_sec: Option<u32>,

/// 
    #[serde(rename = "ProcessCountActive")]
    pub process_count_active: Option<u32>,

/// 
    #[serde(rename = "ProcessCountTerminated")]
    pub process_count_terminated: Option<u32>,

/// 
    #[serde(rename = "ProcessCountTotal")]
    pub process_count_total: Option<u32>,

/// 
    #[serde(rename = "ThisPeriodmSecKernelMode")]
    pub this_periodm_sec_kernel_mode: Option<u64>,

/// 
    #[serde(rename = "ThisPeriodmSecProcessor")]
    pub this_periodm_sec_processor: Option<u64>,

/// 
    #[serde(rename = "ThisPeriodmSecUserMode")]
    pub this_periodm_sec_user_mode: Option<u64>,

/// 
    #[serde(rename = "TotalmSecKernelMode")]
    pub totalm_sec_kernel_mode: Option<u64>,

/// 
    #[serde(rename = "TotalmSecProcessor")]
    pub totalm_sec_processor: Option<u64>,

/// 
    #[serde(rename = "TotalmSecUserMode")]
    pub totalm_sec_user_mode: Option<u64>,
}

impl Win32_PerfRawData_PerfProc_JobObject {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            current_percent_kernel_mode_time: None,
            current_percent_processor_time: None,
            current_percent_user_mode_time: None,
            pages_per_sec: None,
            process_count_active: None,
            process_count_terminated: None,
            process_count_total: None,
            this_periodm_sec_kernel_mode: None,
            this_periodm_sec_processor: None,
            this_periodm_sec_user_mode: None,
            totalm_sec_kernel_mode: None,
            totalm_sec_processor: None,
            totalm_sec_user_mode: None,
        }
    }


    /// Sets the value of CurrentPercentKernelModeTime
    pub fn set_current_percent_kernel_mode_time(&mut self, value: u64) {
        self.current_percent_kernel_mode_time = Some(value);
    }

    /// Gets the value of CurrentPercentKernelModeTime
    pub fn get_current_percent_kernel_mode_time(&self) -> Option<&u64> {
        self.current_percent_kernel_mode_time.as_ref()
    }

    /// Sets the value of CurrentPercentProcessorTime
    pub fn set_current_percent_processor_time(&mut self, value: u64) {
        self.current_percent_processor_time = Some(value);
    }

    /// Gets the value of CurrentPercentProcessorTime
    pub fn get_current_percent_processor_time(&self) -> Option<&u64> {
        self.current_percent_processor_time.as_ref()
    }

    /// Sets the value of CurrentPercentUserModeTime
    pub fn set_current_percent_user_mode_time(&mut self, value: u64) {
        self.current_percent_user_mode_time = Some(value);
    }

    /// Gets the value of CurrentPercentUserModeTime
    pub fn get_current_percent_user_mode_time(&self) -> Option<&u64> {
        self.current_percent_user_mode_time.as_ref()
    }

    /// Sets the value of PagesPerSec
    pub fn set_pages_per_sec(&mut self, value: u32) {
        self.pages_per_sec = Some(value);
    }

    /// Gets the value of PagesPerSec
    pub fn get_pages_per_sec(&self) -> Option<&u32> {
        self.pages_per_sec.as_ref()
    }

    /// Sets the value of ProcessCountActive
    pub fn set_process_count_active(&mut self, value: u32) {
        self.process_count_active = Some(value);
    }

    /// Gets the value of ProcessCountActive
    pub fn get_process_count_active(&self) -> Option<&u32> {
        self.process_count_active.as_ref()
    }

    /// Sets the value of ProcessCountTerminated
    pub fn set_process_count_terminated(&mut self, value: u32) {
        self.process_count_terminated = Some(value);
    }

    /// Gets the value of ProcessCountTerminated
    pub fn get_process_count_terminated(&self) -> Option<&u32> {
        self.process_count_terminated.as_ref()
    }

    /// Sets the value of ProcessCountTotal
    pub fn set_process_count_total(&mut self, value: u32) {
        self.process_count_total = Some(value);
    }

    /// Gets the value of ProcessCountTotal
    pub fn get_process_count_total(&self) -> Option<&u32> {
        self.process_count_total.as_ref()
    }

    /// Sets the value of ThisPeriodmSecKernelMode
    pub fn set_this_periodm_sec_kernel_mode(&mut self, value: u64) {
        self.this_periodm_sec_kernel_mode = Some(value);
    }

    /// Gets the value of ThisPeriodmSecKernelMode
    pub fn get_this_periodm_sec_kernel_mode(&self) -> Option<&u64> {
        self.this_periodm_sec_kernel_mode.as_ref()
    }

    /// Sets the value of ThisPeriodmSecProcessor
    pub fn set_this_periodm_sec_processor(&mut self, value: u64) {
        self.this_periodm_sec_processor = Some(value);
    }

    /// Gets the value of ThisPeriodmSecProcessor
    pub fn get_this_periodm_sec_processor(&self) -> Option<&u64> {
        self.this_periodm_sec_processor.as_ref()
    }

    /// Sets the value of ThisPeriodmSecUserMode
    pub fn set_this_periodm_sec_user_mode(&mut self, value: u64) {
        self.this_periodm_sec_user_mode = Some(value);
    }

    /// Gets the value of ThisPeriodmSecUserMode
    pub fn get_this_periodm_sec_user_mode(&self) -> Option<&u64> {
        self.this_periodm_sec_user_mode.as_ref()
    }

    /// Sets the value of TotalmSecKernelMode
    pub fn set_totalm_sec_kernel_mode(&mut self, value: u64) {
        self.totalm_sec_kernel_mode = Some(value);
    }

    /// Gets the value of TotalmSecKernelMode
    pub fn get_totalm_sec_kernel_mode(&self) -> Option<&u64> {
        self.totalm_sec_kernel_mode.as_ref()
    }

    /// Sets the value of TotalmSecProcessor
    pub fn set_totalm_sec_processor(&mut self, value: u64) {
        self.totalm_sec_processor = Some(value);
    }

    /// Gets the value of TotalmSecProcessor
    pub fn get_totalm_sec_processor(&self) -> Option<&u64> {
        self.totalm_sec_processor.as_ref()
    }

    /// Sets the value of TotalmSecUserMode
    pub fn set_totalm_sec_user_mode(&mut self, value: u64) {
        self.totalm_sec_user_mode = Some(value);
    }

    /// Gets the value of TotalmSecUserMode
    pub fn get_totalm_sec_user_mode(&self) -> Option<&u64> {
        self.totalm_sec_user_mode.as_ref()
    }
}

