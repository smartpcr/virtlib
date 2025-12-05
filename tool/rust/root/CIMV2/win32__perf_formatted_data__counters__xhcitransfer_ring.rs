// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_XHCITransferRing struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_XHCITransferRing {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BytesPerSec")]
    pub bytes_per_sec: Option<u32>,

/// 
    #[serde(rename = "FailedTransferCount")]
    pub failed_transfer_count: Option<u32>,

/// 
    #[serde(rename = "IsochTDFailuresPersec")]
    pub isoch_tdfailures_persec: Option<u32>,

/// 
    #[serde(rename = "IsochTDPersec")]
    pub isoch_tdpersec: Option<u32>,

/// 
    #[serde(rename = "MissedServiceErrorCount")]
    pub missed_service_error_count: Option<u32>,

/// 
    #[serde(rename = "TransfersPersec")]
    pub transfers_persec: Option<u32>,

/// 
    #[serde(rename = "UnderrunOverruncount")]
    pub underrun_overruncount: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_XHCITransferRing {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytes_per_sec: None,
            failed_transfer_count: None,
            isoch_tdfailures_persec: None,
            isoch_tdpersec: None,
            missed_service_error_count: None,
            transfers_persec: None,
            underrun_overruncount: None,
        }
    }


    /// Sets the value of BytesPerSec
    pub fn set_bytes_per_sec(&mut self, value: u32) {
        self.bytes_per_sec = Some(value);
    }

    /// Gets the value of BytesPerSec
    pub fn get_bytes_per_sec(&self) -> Option<&u32> {
        self.bytes_per_sec.as_ref()
    }

    /// Sets the value of FailedTransferCount
    pub fn set_failed_transfer_count(&mut self, value: u32) {
        self.failed_transfer_count = Some(value);
    }

    /// Gets the value of FailedTransferCount
    pub fn get_failed_transfer_count(&self) -> Option<&u32> {
        self.failed_transfer_count.as_ref()
    }

    /// Sets the value of IsochTDFailuresPersec
    pub fn set_isoch_tdfailures_persec(&mut self, value: u32) {
        self.isoch_tdfailures_persec = Some(value);
    }

    /// Gets the value of IsochTDFailuresPersec
    pub fn get_isoch_tdfailures_persec(&self) -> Option<&u32> {
        self.isoch_tdfailures_persec.as_ref()
    }

    /// Sets the value of IsochTDPersec
    pub fn set_isoch_tdpersec(&mut self, value: u32) {
        self.isoch_tdpersec = Some(value);
    }

    /// Gets the value of IsochTDPersec
    pub fn get_isoch_tdpersec(&self) -> Option<&u32> {
        self.isoch_tdpersec.as_ref()
    }

    /// Sets the value of MissedServiceErrorCount
    pub fn set_missed_service_error_count(&mut self, value: u32) {
        self.missed_service_error_count = Some(value);
    }

    /// Gets the value of MissedServiceErrorCount
    pub fn get_missed_service_error_count(&self) -> Option<&u32> {
        self.missed_service_error_count.as_ref()
    }

    /// Sets the value of TransfersPersec
    pub fn set_transfers_persec(&mut self, value: u32) {
        self.transfers_persec = Some(value);
    }

    /// Gets the value of TransfersPersec
    pub fn get_transfers_persec(&self) -> Option<&u32> {
        self.transfers_persec.as_ref()
    }

    /// Sets the value of UnderrunOverruncount
    pub fn set_underrun_overruncount(&mut self, value: u32) {
        self.underrun_overruncount = Some(value);
    }

    /// Gets the value of UnderrunOverruncount
    pub fn get_underrun_overruncount(&self) -> Option<&u32> {
        self.underrun_overruncount.as_ref()
    }
}

