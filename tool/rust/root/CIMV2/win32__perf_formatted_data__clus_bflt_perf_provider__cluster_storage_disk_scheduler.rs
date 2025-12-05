// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageDiskScheduler struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageDiskScheduler {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DspPerSysAvgQueueLength")]
    pub dsp_per_sys_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerSysHighAvgQueueLength")]
    pub dsp_per_sys_high_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerSysHighAvgsecPerDataRequest")]
    pub dsp_per_sys_high_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "DspPerSysHighCurrentQueueLength")]
    pub dsp_per_sys_high_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerSysIdlePerLowAvgQueueLength")]
    pub dsp_per_sys_idle_per_low_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerSysIdlePerLowAvgsecPerDataRequest")]
    pub dsp_per_sys_idle_per_low_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "DspPerSysIdlePerLowCurrentQueueLength")]
    pub dsp_per_sys_idle_per_low_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerSysNormalAvgQueueLength")]
    pub dsp_per_sys_normal_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerSysNormalAvgsecPerDataRequest")]
    pub dsp_per_sys_normal_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "DspPerSysNormalCurrentQueueLength")]
    pub dsp_per_sys_normal_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerUsrAvgQueueLength")]
    pub dsp_per_usr_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerUsrHighAvgQueueLength")]
    pub dsp_per_usr_high_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerUsrHighAvgsecPerDataRequest")]
    pub dsp_per_usr_high_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "DspPerUsrHighCurrentQueueLength")]
    pub dsp_per_usr_high_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerUsrIdlePerLowAvgQueueLength")]
    pub dsp_per_usr_idle_per_low_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerUsrIdlePerLowAvgsecPerDataRequest")]
    pub dsp_per_usr_idle_per_low_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "DspPerUsrIdlePerLowCurrentQueueLength")]
    pub dsp_per_usr_idle_per_low_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerUsrNormalAvgQueueLength")]
    pub dsp_per_usr_normal_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "DspPerUsrNormalAvgsecPerDataRequest")]
    pub dsp_per_usr_normal_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "DspPerUsrNormalCurrentQueueLength")]
    pub dsp_per_usr_normal_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysAvgQueueLength")]
    pub que_per_sys_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysHighAvgQueueLength")]
    pub que_per_sys_high_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysHighAvgsecPerDataRequest")]
    pub que_per_sys_high_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "QuePerSysHighBytesPersec")]
    pub que_per_sys_high_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerSysHighCurrentQueueLength")]
    pub que_per_sys_high_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysHighDataRequestsPersec")]
    pub que_per_sys_high_data_requests_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerSysIdlePerLowAvgQueueLength")]
    pub que_per_sys_idle_per_low_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysIdlePerLowAvgsecPerDataRequest")]
    pub que_per_sys_idle_per_low_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "QuePerSysIdlePerLowBytesPersec")]
    pub que_per_sys_idle_per_low_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerSysIdlePerLowCurrentQueueLength")]
    pub que_per_sys_idle_per_low_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysIdlePerLowDataRequestsPersec")]
    pub que_per_sys_idle_per_low_data_requests_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerSysNormalAvgQueueLength")]
    pub que_per_sys_normal_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysNormalAvgsecPerDataRequest")]
    pub que_per_sys_normal_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "QuePerSysNormalBytesPersec")]
    pub que_per_sys_normal_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerSysNormalCurrentQueueLength")]
    pub que_per_sys_normal_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerSysNormalDataRequestsPersec")]
    pub que_per_sys_normal_data_requests_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrAvgQueueLength")]
    pub que_per_usr_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrHighAvgQueueLength")]
    pub que_per_usr_high_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrHighAvgsecPerDataRequest")]
    pub que_per_usr_high_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "QuePerUsrHighBytesPersec")]
    pub que_per_usr_high_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrHighCurrentQueueLength")]
    pub que_per_usr_high_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrHighDataRequestsPersec")]
    pub que_per_usr_high_data_requests_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrIdlePerLowAvgQueueLength")]
    pub que_per_usr_idle_per_low_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrIdlePerLowAvgsecPerDataRequest")]
    pub que_per_usr_idle_per_low_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "QuePerUsrIdlePerLowBytesPersec")]
    pub que_per_usr_idle_per_low_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrIdlePerLowCurrentQueueLength")]
    pub que_per_usr_idle_per_low_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrIdlePerLowDataRequestsPersec")]
    pub que_per_usr_idle_per_low_data_requests_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrNormalAvgQueueLength")]
    pub que_per_usr_normal_avg_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrNormalAvgsecPerDataRequest")]
    pub que_per_usr_normal_avgsec_per_data_request: Option<u32>,

/// 
    #[serde(rename = "QuePerUsrNormalBytesPersec")]
    pub que_per_usr_normal_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrNormalCurrentQueueLength")]
    pub que_per_usr_normal_current_queue_length: Option<u64>,

/// 
    #[serde(rename = "QuePerUsrNormalDataRequestsPersec")]
    pub que_per_usr_normal_data_requests_persec: Option<u64>,
}

impl Win32_PerfFormattedData_ClusBfltPerfProvider_ClusterStorageDiskScheduler {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            dsp_per_sys_avg_queue_length: None,
            dsp_per_sys_high_avg_queue_length: None,
            dsp_per_sys_high_avgsec_per_data_request: None,
            dsp_per_sys_high_current_queue_length: None,
            dsp_per_sys_idle_per_low_avg_queue_length: None,
            dsp_per_sys_idle_per_low_avgsec_per_data_request: None,
            dsp_per_sys_idle_per_low_current_queue_length: None,
            dsp_per_sys_normal_avg_queue_length: None,
            dsp_per_sys_normal_avgsec_per_data_request: None,
            dsp_per_sys_normal_current_queue_length: None,
            dsp_per_usr_avg_queue_length: None,
            dsp_per_usr_high_avg_queue_length: None,
            dsp_per_usr_high_avgsec_per_data_request: None,
            dsp_per_usr_high_current_queue_length: None,
            dsp_per_usr_idle_per_low_avg_queue_length: None,
            dsp_per_usr_idle_per_low_avgsec_per_data_request: None,
            dsp_per_usr_idle_per_low_current_queue_length: None,
            dsp_per_usr_normal_avg_queue_length: None,
            dsp_per_usr_normal_avgsec_per_data_request: None,
            dsp_per_usr_normal_current_queue_length: None,
            que_per_sys_avg_queue_length: None,
            que_per_sys_high_avg_queue_length: None,
            que_per_sys_high_avgsec_per_data_request: None,
            que_per_sys_high_bytes_persec: None,
            que_per_sys_high_current_queue_length: None,
            que_per_sys_high_data_requests_persec: None,
            que_per_sys_idle_per_low_avg_queue_length: None,
            que_per_sys_idle_per_low_avgsec_per_data_request: None,
            que_per_sys_idle_per_low_bytes_persec: None,
            que_per_sys_idle_per_low_current_queue_length: None,
            que_per_sys_idle_per_low_data_requests_persec: None,
            que_per_sys_normal_avg_queue_length: None,
            que_per_sys_normal_avgsec_per_data_request: None,
            que_per_sys_normal_bytes_persec: None,
            que_per_sys_normal_current_queue_length: None,
            que_per_sys_normal_data_requests_persec: None,
            que_per_usr_avg_queue_length: None,
            que_per_usr_high_avg_queue_length: None,
            que_per_usr_high_avgsec_per_data_request: None,
            que_per_usr_high_bytes_persec: None,
            que_per_usr_high_current_queue_length: None,
            que_per_usr_high_data_requests_persec: None,
            que_per_usr_idle_per_low_avg_queue_length: None,
            que_per_usr_idle_per_low_avgsec_per_data_request: None,
            que_per_usr_idle_per_low_bytes_persec: None,
            que_per_usr_idle_per_low_current_queue_length: None,
            que_per_usr_idle_per_low_data_requests_persec: None,
            que_per_usr_normal_avg_queue_length: None,
            que_per_usr_normal_avgsec_per_data_request: None,
            que_per_usr_normal_bytes_persec: None,
            que_per_usr_normal_current_queue_length: None,
            que_per_usr_normal_data_requests_persec: None,
        }
    }


    /// Sets the value of DspPerSysAvgQueueLength
    pub fn set_dsp_per_sys_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_sys_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerSysAvgQueueLength
    pub fn get_dsp_per_sys_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_sys_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerSysHighAvgQueueLength
    pub fn set_dsp_per_sys_high_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_sys_high_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerSysHighAvgQueueLength
    pub fn get_dsp_per_sys_high_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_sys_high_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerSysHighAvgsecPerDataRequest
    pub fn set_dsp_per_sys_high_avgsec_per_data_request(&mut self, value: u32) {
        self.dsp_per_sys_high_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of DspPerSysHighAvgsecPerDataRequest
    pub fn get_dsp_per_sys_high_avgsec_per_data_request(&self) -> Option<&u32> {
        self.dsp_per_sys_high_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of DspPerSysHighCurrentQueueLength
    pub fn set_dsp_per_sys_high_current_queue_length(&mut self, value: u64) {
        self.dsp_per_sys_high_current_queue_length = Some(value);
    }

    /// Gets the value of DspPerSysHighCurrentQueueLength
    pub fn get_dsp_per_sys_high_current_queue_length(&self) -> Option<&u64> {
        self.dsp_per_sys_high_current_queue_length.as_ref()
    }

    /// Sets the value of DspPerSysIdlePerLowAvgQueueLength
    pub fn set_dsp_per_sys_idle_per_low_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_sys_idle_per_low_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerSysIdlePerLowAvgQueueLength
    pub fn get_dsp_per_sys_idle_per_low_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_sys_idle_per_low_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerSysIdlePerLowAvgsecPerDataRequest
    pub fn set_dsp_per_sys_idle_per_low_avgsec_per_data_request(&mut self, value: u32) {
        self.dsp_per_sys_idle_per_low_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of DspPerSysIdlePerLowAvgsecPerDataRequest
    pub fn get_dsp_per_sys_idle_per_low_avgsec_per_data_request(&self) -> Option<&u32> {
        self.dsp_per_sys_idle_per_low_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of DspPerSysIdlePerLowCurrentQueueLength
    pub fn set_dsp_per_sys_idle_per_low_current_queue_length(&mut self, value: u64) {
        self.dsp_per_sys_idle_per_low_current_queue_length = Some(value);
    }

    /// Gets the value of DspPerSysIdlePerLowCurrentQueueLength
    pub fn get_dsp_per_sys_idle_per_low_current_queue_length(&self) -> Option<&u64> {
        self.dsp_per_sys_idle_per_low_current_queue_length.as_ref()
    }

    /// Sets the value of DspPerSysNormalAvgQueueLength
    pub fn set_dsp_per_sys_normal_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_sys_normal_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerSysNormalAvgQueueLength
    pub fn get_dsp_per_sys_normal_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_sys_normal_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerSysNormalAvgsecPerDataRequest
    pub fn set_dsp_per_sys_normal_avgsec_per_data_request(&mut self, value: u32) {
        self.dsp_per_sys_normal_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of DspPerSysNormalAvgsecPerDataRequest
    pub fn get_dsp_per_sys_normal_avgsec_per_data_request(&self) -> Option<&u32> {
        self.dsp_per_sys_normal_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of DspPerSysNormalCurrentQueueLength
    pub fn set_dsp_per_sys_normal_current_queue_length(&mut self, value: u64) {
        self.dsp_per_sys_normal_current_queue_length = Some(value);
    }

    /// Gets the value of DspPerSysNormalCurrentQueueLength
    pub fn get_dsp_per_sys_normal_current_queue_length(&self) -> Option<&u64> {
        self.dsp_per_sys_normal_current_queue_length.as_ref()
    }

    /// Sets the value of DspPerUsrAvgQueueLength
    pub fn set_dsp_per_usr_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_usr_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerUsrAvgQueueLength
    pub fn get_dsp_per_usr_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_usr_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerUsrHighAvgQueueLength
    pub fn set_dsp_per_usr_high_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_usr_high_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerUsrHighAvgQueueLength
    pub fn get_dsp_per_usr_high_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_usr_high_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerUsrHighAvgsecPerDataRequest
    pub fn set_dsp_per_usr_high_avgsec_per_data_request(&mut self, value: u32) {
        self.dsp_per_usr_high_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of DspPerUsrHighAvgsecPerDataRequest
    pub fn get_dsp_per_usr_high_avgsec_per_data_request(&self) -> Option<&u32> {
        self.dsp_per_usr_high_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of DspPerUsrHighCurrentQueueLength
    pub fn set_dsp_per_usr_high_current_queue_length(&mut self, value: u64) {
        self.dsp_per_usr_high_current_queue_length = Some(value);
    }

    /// Gets the value of DspPerUsrHighCurrentQueueLength
    pub fn get_dsp_per_usr_high_current_queue_length(&self) -> Option<&u64> {
        self.dsp_per_usr_high_current_queue_length.as_ref()
    }

    /// Sets the value of DspPerUsrIdlePerLowAvgQueueLength
    pub fn set_dsp_per_usr_idle_per_low_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_usr_idle_per_low_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerUsrIdlePerLowAvgQueueLength
    pub fn get_dsp_per_usr_idle_per_low_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_usr_idle_per_low_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerUsrIdlePerLowAvgsecPerDataRequest
    pub fn set_dsp_per_usr_idle_per_low_avgsec_per_data_request(&mut self, value: u32) {
        self.dsp_per_usr_idle_per_low_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of DspPerUsrIdlePerLowAvgsecPerDataRequest
    pub fn get_dsp_per_usr_idle_per_low_avgsec_per_data_request(&self) -> Option<&u32> {
        self.dsp_per_usr_idle_per_low_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of DspPerUsrIdlePerLowCurrentQueueLength
    pub fn set_dsp_per_usr_idle_per_low_current_queue_length(&mut self, value: u64) {
        self.dsp_per_usr_idle_per_low_current_queue_length = Some(value);
    }

    /// Gets the value of DspPerUsrIdlePerLowCurrentQueueLength
    pub fn get_dsp_per_usr_idle_per_low_current_queue_length(&self) -> Option<&u64> {
        self.dsp_per_usr_idle_per_low_current_queue_length.as_ref()
    }

    /// Sets the value of DspPerUsrNormalAvgQueueLength
    pub fn set_dsp_per_usr_normal_avg_queue_length(&mut self, value: u64) {
        self.dsp_per_usr_normal_avg_queue_length = Some(value);
    }

    /// Gets the value of DspPerUsrNormalAvgQueueLength
    pub fn get_dsp_per_usr_normal_avg_queue_length(&self) -> Option<&u64> {
        self.dsp_per_usr_normal_avg_queue_length.as_ref()
    }

    /// Sets the value of DspPerUsrNormalAvgsecPerDataRequest
    pub fn set_dsp_per_usr_normal_avgsec_per_data_request(&mut self, value: u32) {
        self.dsp_per_usr_normal_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of DspPerUsrNormalAvgsecPerDataRequest
    pub fn get_dsp_per_usr_normal_avgsec_per_data_request(&self) -> Option<&u32> {
        self.dsp_per_usr_normal_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of DspPerUsrNormalCurrentQueueLength
    pub fn set_dsp_per_usr_normal_current_queue_length(&mut self, value: u64) {
        self.dsp_per_usr_normal_current_queue_length = Some(value);
    }

    /// Gets the value of DspPerUsrNormalCurrentQueueLength
    pub fn get_dsp_per_usr_normal_current_queue_length(&self) -> Option<&u64> {
        self.dsp_per_usr_normal_current_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysAvgQueueLength
    pub fn set_que_per_sys_avg_queue_length(&mut self, value: u64) {
        self.que_per_sys_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerSysAvgQueueLength
    pub fn get_que_per_sys_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_sys_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysHighAvgQueueLength
    pub fn set_que_per_sys_high_avg_queue_length(&mut self, value: u64) {
        self.que_per_sys_high_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerSysHighAvgQueueLength
    pub fn get_que_per_sys_high_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_sys_high_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysHighAvgsecPerDataRequest
    pub fn set_que_per_sys_high_avgsec_per_data_request(&mut self, value: u32) {
        self.que_per_sys_high_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of QuePerSysHighAvgsecPerDataRequest
    pub fn get_que_per_sys_high_avgsec_per_data_request(&self) -> Option<&u32> {
        self.que_per_sys_high_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of QuePerSysHighBytesPersec
    pub fn set_que_per_sys_high_bytes_persec(&mut self, value: u64) {
        self.que_per_sys_high_bytes_persec = Some(value);
    }

    /// Gets the value of QuePerSysHighBytesPersec
    pub fn get_que_per_sys_high_bytes_persec(&self) -> Option<&u64> {
        self.que_per_sys_high_bytes_persec.as_ref()
    }

    /// Sets the value of QuePerSysHighCurrentQueueLength
    pub fn set_que_per_sys_high_current_queue_length(&mut self, value: u64) {
        self.que_per_sys_high_current_queue_length = Some(value);
    }

    /// Gets the value of QuePerSysHighCurrentQueueLength
    pub fn get_que_per_sys_high_current_queue_length(&self) -> Option<&u64> {
        self.que_per_sys_high_current_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysHighDataRequestsPersec
    pub fn set_que_per_sys_high_data_requests_persec(&mut self, value: u64) {
        self.que_per_sys_high_data_requests_persec = Some(value);
    }

    /// Gets the value of QuePerSysHighDataRequestsPersec
    pub fn get_que_per_sys_high_data_requests_persec(&self) -> Option<&u64> {
        self.que_per_sys_high_data_requests_persec.as_ref()
    }

    /// Sets the value of QuePerSysIdlePerLowAvgQueueLength
    pub fn set_que_per_sys_idle_per_low_avg_queue_length(&mut self, value: u64) {
        self.que_per_sys_idle_per_low_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerSysIdlePerLowAvgQueueLength
    pub fn get_que_per_sys_idle_per_low_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_sys_idle_per_low_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysIdlePerLowAvgsecPerDataRequest
    pub fn set_que_per_sys_idle_per_low_avgsec_per_data_request(&mut self, value: u32) {
        self.que_per_sys_idle_per_low_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of QuePerSysIdlePerLowAvgsecPerDataRequest
    pub fn get_que_per_sys_idle_per_low_avgsec_per_data_request(&self) -> Option<&u32> {
        self.que_per_sys_idle_per_low_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of QuePerSysIdlePerLowBytesPersec
    pub fn set_que_per_sys_idle_per_low_bytes_persec(&mut self, value: u64) {
        self.que_per_sys_idle_per_low_bytes_persec = Some(value);
    }

    /// Gets the value of QuePerSysIdlePerLowBytesPersec
    pub fn get_que_per_sys_idle_per_low_bytes_persec(&self) -> Option<&u64> {
        self.que_per_sys_idle_per_low_bytes_persec.as_ref()
    }

    /// Sets the value of QuePerSysIdlePerLowCurrentQueueLength
    pub fn set_que_per_sys_idle_per_low_current_queue_length(&mut self, value: u64) {
        self.que_per_sys_idle_per_low_current_queue_length = Some(value);
    }

    /// Gets the value of QuePerSysIdlePerLowCurrentQueueLength
    pub fn get_que_per_sys_idle_per_low_current_queue_length(&self) -> Option<&u64> {
        self.que_per_sys_idle_per_low_current_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysIdlePerLowDataRequestsPersec
    pub fn set_que_per_sys_idle_per_low_data_requests_persec(&mut self, value: u64) {
        self.que_per_sys_idle_per_low_data_requests_persec = Some(value);
    }

    /// Gets the value of QuePerSysIdlePerLowDataRequestsPersec
    pub fn get_que_per_sys_idle_per_low_data_requests_persec(&self) -> Option<&u64> {
        self.que_per_sys_idle_per_low_data_requests_persec.as_ref()
    }

    /// Sets the value of QuePerSysNormalAvgQueueLength
    pub fn set_que_per_sys_normal_avg_queue_length(&mut self, value: u64) {
        self.que_per_sys_normal_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerSysNormalAvgQueueLength
    pub fn get_que_per_sys_normal_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_sys_normal_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysNormalAvgsecPerDataRequest
    pub fn set_que_per_sys_normal_avgsec_per_data_request(&mut self, value: u32) {
        self.que_per_sys_normal_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of QuePerSysNormalAvgsecPerDataRequest
    pub fn get_que_per_sys_normal_avgsec_per_data_request(&self) -> Option<&u32> {
        self.que_per_sys_normal_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of QuePerSysNormalBytesPersec
    pub fn set_que_per_sys_normal_bytes_persec(&mut self, value: u64) {
        self.que_per_sys_normal_bytes_persec = Some(value);
    }

    /// Gets the value of QuePerSysNormalBytesPersec
    pub fn get_que_per_sys_normal_bytes_persec(&self) -> Option<&u64> {
        self.que_per_sys_normal_bytes_persec.as_ref()
    }

    /// Sets the value of QuePerSysNormalCurrentQueueLength
    pub fn set_que_per_sys_normal_current_queue_length(&mut self, value: u64) {
        self.que_per_sys_normal_current_queue_length = Some(value);
    }

    /// Gets the value of QuePerSysNormalCurrentQueueLength
    pub fn get_que_per_sys_normal_current_queue_length(&self) -> Option<&u64> {
        self.que_per_sys_normal_current_queue_length.as_ref()
    }

    /// Sets the value of QuePerSysNormalDataRequestsPersec
    pub fn set_que_per_sys_normal_data_requests_persec(&mut self, value: u64) {
        self.que_per_sys_normal_data_requests_persec = Some(value);
    }

    /// Gets the value of QuePerSysNormalDataRequestsPersec
    pub fn get_que_per_sys_normal_data_requests_persec(&self) -> Option<&u64> {
        self.que_per_sys_normal_data_requests_persec.as_ref()
    }

    /// Sets the value of QuePerUsrAvgQueueLength
    pub fn set_que_per_usr_avg_queue_length(&mut self, value: u64) {
        self.que_per_usr_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerUsrAvgQueueLength
    pub fn get_que_per_usr_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_usr_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerUsrHighAvgQueueLength
    pub fn set_que_per_usr_high_avg_queue_length(&mut self, value: u64) {
        self.que_per_usr_high_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerUsrHighAvgQueueLength
    pub fn get_que_per_usr_high_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_usr_high_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerUsrHighAvgsecPerDataRequest
    pub fn set_que_per_usr_high_avgsec_per_data_request(&mut self, value: u32) {
        self.que_per_usr_high_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of QuePerUsrHighAvgsecPerDataRequest
    pub fn get_que_per_usr_high_avgsec_per_data_request(&self) -> Option<&u32> {
        self.que_per_usr_high_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of QuePerUsrHighBytesPersec
    pub fn set_que_per_usr_high_bytes_persec(&mut self, value: u64) {
        self.que_per_usr_high_bytes_persec = Some(value);
    }

    /// Gets the value of QuePerUsrHighBytesPersec
    pub fn get_que_per_usr_high_bytes_persec(&self) -> Option<&u64> {
        self.que_per_usr_high_bytes_persec.as_ref()
    }

    /// Sets the value of QuePerUsrHighCurrentQueueLength
    pub fn set_que_per_usr_high_current_queue_length(&mut self, value: u64) {
        self.que_per_usr_high_current_queue_length = Some(value);
    }

    /// Gets the value of QuePerUsrHighCurrentQueueLength
    pub fn get_que_per_usr_high_current_queue_length(&self) -> Option<&u64> {
        self.que_per_usr_high_current_queue_length.as_ref()
    }

    /// Sets the value of QuePerUsrHighDataRequestsPersec
    pub fn set_que_per_usr_high_data_requests_persec(&mut self, value: u64) {
        self.que_per_usr_high_data_requests_persec = Some(value);
    }

    /// Gets the value of QuePerUsrHighDataRequestsPersec
    pub fn get_que_per_usr_high_data_requests_persec(&self) -> Option<&u64> {
        self.que_per_usr_high_data_requests_persec.as_ref()
    }

    /// Sets the value of QuePerUsrIdlePerLowAvgQueueLength
    pub fn set_que_per_usr_idle_per_low_avg_queue_length(&mut self, value: u64) {
        self.que_per_usr_idle_per_low_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerUsrIdlePerLowAvgQueueLength
    pub fn get_que_per_usr_idle_per_low_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_usr_idle_per_low_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerUsrIdlePerLowAvgsecPerDataRequest
    pub fn set_que_per_usr_idle_per_low_avgsec_per_data_request(&mut self, value: u32) {
        self.que_per_usr_idle_per_low_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of QuePerUsrIdlePerLowAvgsecPerDataRequest
    pub fn get_que_per_usr_idle_per_low_avgsec_per_data_request(&self) -> Option<&u32> {
        self.que_per_usr_idle_per_low_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of QuePerUsrIdlePerLowBytesPersec
    pub fn set_que_per_usr_idle_per_low_bytes_persec(&mut self, value: u64) {
        self.que_per_usr_idle_per_low_bytes_persec = Some(value);
    }

    /// Gets the value of QuePerUsrIdlePerLowBytesPersec
    pub fn get_que_per_usr_idle_per_low_bytes_persec(&self) -> Option<&u64> {
        self.que_per_usr_idle_per_low_bytes_persec.as_ref()
    }

    /// Sets the value of QuePerUsrIdlePerLowCurrentQueueLength
    pub fn set_que_per_usr_idle_per_low_current_queue_length(&mut self, value: u64) {
        self.que_per_usr_idle_per_low_current_queue_length = Some(value);
    }

    /// Gets the value of QuePerUsrIdlePerLowCurrentQueueLength
    pub fn get_que_per_usr_idle_per_low_current_queue_length(&self) -> Option<&u64> {
        self.que_per_usr_idle_per_low_current_queue_length.as_ref()
    }

    /// Sets the value of QuePerUsrIdlePerLowDataRequestsPersec
    pub fn set_que_per_usr_idle_per_low_data_requests_persec(&mut self, value: u64) {
        self.que_per_usr_idle_per_low_data_requests_persec = Some(value);
    }

    /// Gets the value of QuePerUsrIdlePerLowDataRequestsPersec
    pub fn get_que_per_usr_idle_per_low_data_requests_persec(&self) -> Option<&u64> {
        self.que_per_usr_idle_per_low_data_requests_persec.as_ref()
    }

    /// Sets the value of QuePerUsrNormalAvgQueueLength
    pub fn set_que_per_usr_normal_avg_queue_length(&mut self, value: u64) {
        self.que_per_usr_normal_avg_queue_length = Some(value);
    }

    /// Gets the value of QuePerUsrNormalAvgQueueLength
    pub fn get_que_per_usr_normal_avg_queue_length(&self) -> Option<&u64> {
        self.que_per_usr_normal_avg_queue_length.as_ref()
    }

    /// Sets the value of QuePerUsrNormalAvgsecPerDataRequest
    pub fn set_que_per_usr_normal_avgsec_per_data_request(&mut self, value: u32) {
        self.que_per_usr_normal_avgsec_per_data_request = Some(value);
    }

    /// Gets the value of QuePerUsrNormalAvgsecPerDataRequest
    pub fn get_que_per_usr_normal_avgsec_per_data_request(&self) -> Option<&u32> {
        self.que_per_usr_normal_avgsec_per_data_request.as_ref()
    }

    /// Sets the value of QuePerUsrNormalBytesPersec
    pub fn set_que_per_usr_normal_bytes_persec(&mut self, value: u64) {
        self.que_per_usr_normal_bytes_persec = Some(value);
    }

    /// Gets the value of QuePerUsrNormalBytesPersec
    pub fn get_que_per_usr_normal_bytes_persec(&self) -> Option<&u64> {
        self.que_per_usr_normal_bytes_persec.as_ref()
    }

    /// Sets the value of QuePerUsrNormalCurrentQueueLength
    pub fn set_que_per_usr_normal_current_queue_length(&mut self, value: u64) {
        self.que_per_usr_normal_current_queue_length = Some(value);
    }

    /// Gets the value of QuePerUsrNormalCurrentQueueLength
    pub fn get_que_per_usr_normal_current_queue_length(&self) -> Option<&u64> {
        self.que_per_usr_normal_current_queue_length.as_ref()
    }

    /// Sets the value of QuePerUsrNormalDataRequestsPersec
    pub fn set_que_per_usr_normal_data_requests_persec(&mut self, value: u64) {
        self.que_per_usr_normal_data_requests_persec = Some(value);
    }

    /// Gets the value of QuePerUsrNormalDataRequestsPersec
    pub fn get_que_per_usr_normal_data_requests_persec(&self) -> Option<&u64> {
        self.que_per_usr_normal_data_requests_persec.as_ref()
    }
}

