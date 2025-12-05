// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_BITS_BITSNetUtilization struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_BITS_BITSNetUtilization {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BITSDownloadBlockSizeBytes")]
    pub bitsdownload_block_size_bytes: Option<u32>,

/// 
    #[serde(rename = "BITSDownloadResponseIntervalmsec")]
    pub bitsdownload_response_intervalmsec: Option<u32>,

/// 
    #[serde(rename = "EstimatedbandwidthavailabletotheremotesystemBitsPersec")]
    pub estimatedbandwidthavailabletotheremotesystem_bits_persec: Option<u32>,

/// 
    #[serde(rename = "IGDSpeedBitsPerSec")]
    pub igdspeed_bits_per_sec: Option<u32>,

/// 
    #[serde(rename = "NetcardSpeedBitsPerSec")]
    pub netcard_speed_bits_per_sec: Option<u32>,

/// 
    #[serde(rename = "PercentIGDFree")]
    pub percent_igdfree: Option<u32>,

/// 
    #[serde(rename = "PercentNetcardFree")]
    pub percent_netcard_free: Option<u32>,

/// 
    #[serde(rename = "RemoteServerSpeedBitsPerSec")]
    pub remote_server_speed_bits_per_sec: Option<u32>,
}

impl Win32_PerfFormattedData_BITS_BITSNetUtilization {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bitsdownload_block_size_bytes: None,
            bitsdownload_response_intervalmsec: None,
            estimatedbandwidthavailabletotheremotesystem_bits_persec: None,
            igdspeed_bits_per_sec: None,
            netcard_speed_bits_per_sec: None,
            percent_igdfree: None,
            percent_netcard_free: None,
            remote_server_speed_bits_per_sec: None,
        }
    }


    /// Sets the value of BITSDownloadBlockSizeBytes
    pub fn set_bitsdownload_block_size_bytes(&mut self, value: u32) {
        self.bitsdownload_block_size_bytes = Some(value);
    }

    /// Gets the value of BITSDownloadBlockSizeBytes
    pub fn get_bitsdownload_block_size_bytes(&self) -> Option<&u32> {
        self.bitsdownload_block_size_bytes.as_ref()
    }

    /// Sets the value of BITSDownloadResponseIntervalmsec
    pub fn set_bitsdownload_response_intervalmsec(&mut self, value: u32) {
        self.bitsdownload_response_intervalmsec = Some(value);
    }

    /// Gets the value of BITSDownloadResponseIntervalmsec
    pub fn get_bitsdownload_response_intervalmsec(&self) -> Option<&u32> {
        self.bitsdownload_response_intervalmsec.as_ref()
    }

    /// Sets the value of EstimatedbandwidthavailabletotheremotesystemBitsPersec
    pub fn set_estimatedbandwidthavailabletotheremotesystem_bits_persec(&mut self, value: u32) {
        self.estimatedbandwidthavailabletotheremotesystem_bits_persec = Some(value);
    }

    /// Gets the value of EstimatedbandwidthavailabletotheremotesystemBitsPersec
    pub fn get_estimatedbandwidthavailabletotheremotesystem_bits_persec(&self) -> Option<&u32> {
        self.estimatedbandwidthavailabletotheremotesystem_bits_persec.as_ref()
    }

    /// Sets the value of IGDSpeedBitsPerSec
    pub fn set_igdspeed_bits_per_sec(&mut self, value: u32) {
        self.igdspeed_bits_per_sec = Some(value);
    }

    /// Gets the value of IGDSpeedBitsPerSec
    pub fn get_igdspeed_bits_per_sec(&self) -> Option<&u32> {
        self.igdspeed_bits_per_sec.as_ref()
    }

    /// Sets the value of NetcardSpeedBitsPerSec
    pub fn set_netcard_speed_bits_per_sec(&mut self, value: u32) {
        self.netcard_speed_bits_per_sec = Some(value);
    }

    /// Gets the value of NetcardSpeedBitsPerSec
    pub fn get_netcard_speed_bits_per_sec(&self) -> Option<&u32> {
        self.netcard_speed_bits_per_sec.as_ref()
    }

    /// Sets the value of PercentIGDFree
    pub fn set_percent_igdfree(&mut self, value: u32) {
        self.percent_igdfree = Some(value);
    }

    /// Gets the value of PercentIGDFree
    pub fn get_percent_igdfree(&self) -> Option<&u32> {
        self.percent_igdfree.as_ref()
    }

    /// Sets the value of PercentNetcardFree
    pub fn set_percent_netcard_free(&mut self, value: u32) {
        self.percent_netcard_free = Some(value);
    }

    /// Gets the value of PercentNetcardFree
    pub fn get_percent_netcard_free(&self) -> Option<&u32> {
        self.percent_netcard_free.as_ref()
    }

    /// Sets the value of RemoteServerSpeedBitsPerSec
    pub fn set_remote_server_speed_bits_per_sec(&mut self, value: u32) {
        self.remote_server_speed_bits_per_sec = Some(value);
    }

    /// Gets the value of RemoteServerSpeedBitsPerSec
    pub fn get_remote_server_speed_bits_per_sec(&self) -> Option<&u32> {
        self.remote_server_speed_bits_per_sec.as_ref()
    }
}

