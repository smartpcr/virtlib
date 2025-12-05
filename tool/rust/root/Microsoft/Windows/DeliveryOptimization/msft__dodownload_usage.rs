// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DODownloadUsage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DODownloadUsage {
    #[serde(flatten)]
    pub base: MSFT_DOUsage,

/// 
    #[serde(rename = "BackgroundRatePct")]
    pub background_rate_pct: Option<u8>,

/// 
    #[serde(rename = "ForegroundRatePct")]
    pub foreground_rate_pct: Option<u8>,

/// 
    #[serde(rename = "MonthlyBkRateBps")]
    pub monthly_bk_rate_bps: Option<u64>,

/// 
    #[serde(rename = "MonthlyCacheHostBytes")]
    pub monthly_cache_host_bytes: Option<u64>,

/// 
    #[serde(rename = "MonthlyCdnBytes")]
    pub monthly_cdn_bytes: Option<u64>,

/// 
    #[serde(rename = "MonthlyFrRateBps")]
    pub monthly_fr_rate_bps: Option<u64>,

/// 
    #[serde(rename = "NormalDownloads")]
    pub normal_downloads: Option<u32>,

/// 
    #[serde(rename = "NormalDownloadsPending")]
    pub normal_downloads_pending: Option<u32>,

/// 
    #[serde(rename = "PriorityDownloads")]
    pub priority_downloads: Option<u32>,

/// 
    #[serde(rename = "PriorityDownloadsPending")]
    pub priority_downloads_pending: Option<u32>,
}

impl MSFT_DODownloadUsage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DOUsage::new(),
            background_rate_pct: None,
            foreground_rate_pct: None,
            monthly_bk_rate_bps: None,
            monthly_cache_host_bytes: None,
            monthly_cdn_bytes: None,
            monthly_fr_rate_bps: None,
            normal_downloads: None,
            normal_downloads_pending: None,
            priority_downloads: None,
            priority_downloads_pending: None,
        }
    }


    /// Sets the value of BackgroundRatePct
    pub fn set_background_rate_pct(&mut self, value: u8) {
        self.background_rate_pct = Some(value);
    }

    /// Gets the value of BackgroundRatePct
    pub fn get_background_rate_pct(&self) -> Option<&u8> {
        self.background_rate_pct.as_ref()
    }

    /// Sets the value of ForegroundRatePct
    pub fn set_foreground_rate_pct(&mut self, value: u8) {
        self.foreground_rate_pct = Some(value);
    }

    /// Gets the value of ForegroundRatePct
    pub fn get_foreground_rate_pct(&self) -> Option<&u8> {
        self.foreground_rate_pct.as_ref()
    }

    /// Sets the value of MonthlyBkRateBps
    pub fn set_monthly_bk_rate_bps(&mut self, value: u64) {
        self.monthly_bk_rate_bps = Some(value);
    }

    /// Gets the value of MonthlyBkRateBps
    pub fn get_monthly_bk_rate_bps(&self) -> Option<&u64> {
        self.monthly_bk_rate_bps.as_ref()
    }

    /// Sets the value of MonthlyCacheHostBytes
    pub fn set_monthly_cache_host_bytes(&mut self, value: u64) {
        self.monthly_cache_host_bytes = Some(value);
    }

    /// Gets the value of MonthlyCacheHostBytes
    pub fn get_monthly_cache_host_bytes(&self) -> Option<&u64> {
        self.monthly_cache_host_bytes.as_ref()
    }

    /// Sets the value of MonthlyCdnBytes
    pub fn set_monthly_cdn_bytes(&mut self, value: u64) {
        self.monthly_cdn_bytes = Some(value);
    }

    /// Gets the value of MonthlyCdnBytes
    pub fn get_monthly_cdn_bytes(&self) -> Option<&u64> {
        self.monthly_cdn_bytes.as_ref()
    }

    /// Sets the value of MonthlyFrRateBps
    pub fn set_monthly_fr_rate_bps(&mut self, value: u64) {
        self.monthly_fr_rate_bps = Some(value);
    }

    /// Gets the value of MonthlyFrRateBps
    pub fn get_monthly_fr_rate_bps(&self) -> Option<&u64> {
        self.monthly_fr_rate_bps.as_ref()
    }

    /// Sets the value of NormalDownloads
    pub fn set_normal_downloads(&mut self, value: u32) {
        self.normal_downloads = Some(value);
    }

    /// Gets the value of NormalDownloads
    pub fn get_normal_downloads(&self) -> Option<&u32> {
        self.normal_downloads.as_ref()
    }

    /// Sets the value of NormalDownloadsPending
    pub fn set_normal_downloads_pending(&mut self, value: u32) {
        self.normal_downloads_pending = Some(value);
    }

    /// Gets the value of NormalDownloadsPending
    pub fn get_normal_downloads_pending(&self) -> Option<&u32> {
        self.normal_downloads_pending.as_ref()
    }

    /// Sets the value of PriorityDownloads
    pub fn set_priority_downloads(&mut self, value: u32) {
        self.priority_downloads = Some(value);
    }

    /// Gets the value of PriorityDownloads
    pub fn get_priority_downloads(&self) -> Option<&u32> {
        self.priority_downloads.as_ref()
    }

    /// Sets the value of PriorityDownloadsPending
    pub fn set_priority_downloads_pending(&mut self, value: u32) {
        self.priority_downloads_pending = Some(value);
    }

    /// Gets the value of PriorityDownloadsPending
    pub fn get_priority_downloads_pending(&self) -> Option<&u32> {
        self.priority_downloads_pending.as_ref()
    }
}

