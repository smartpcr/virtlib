// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DeliveryOptimizationConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DeliveryOptimizationConfig {
    #[serde(flatten)]
    pub base: MSFT_DOBaseStatus,

/// 
    #[serde(rename = "DownBackLimitBps")]
    pub down_back_limit_bps: Option<u32>,

/// 
    #[serde(rename = "DownBackLimitBpsProvider")]
    pub down_back_limit_bps_provider: Option<DeliveryOptimizationConfig_DownBackLimitBpsProvider>,

/// 
    #[serde(rename = "DownBackLimitPct")]
    pub down_back_limit_pct: Option<u8>,

/// 
    #[serde(rename = "DownBackLimitPctProvider")]
    pub down_back_limit_pct_provider: Option<DeliveryOptimizationConfig_DownBackLimitPctProvider>,

/// 
    #[serde(rename = "DownForeLimitBps")]
    pub down_fore_limit_bps: Option<u32>,

/// 
    #[serde(rename = "DownForeLimitBpsProvider")]
    pub down_fore_limit_bps_provider: Option<DeliveryOptimizationConfig_DownForeLimitBpsProvider>,

/// 
    #[serde(rename = "DownForeLimitPct")]
    pub down_fore_limit_pct: Option<u8>,

/// 
    #[serde(rename = "DownForeLimitPctProvider")]
    pub down_fore_limit_pct_provider: Option<DeliveryOptimizationConfig_DownForeLimitPctProvider>,

/// 16
    #[serde(rename = "DownloadMode")]
    pub download_mode: Option<DeliveryOptimizationConfig_DownloadMode>,

/// 
    #[serde(rename = "DownloadModeProvider")]
    pub download_mode_provider: Option<DeliveryOptimizationConfig_DownloadModeProvider>,

/// 
    #[serde(rename = "MaxUploadRatePct")]
    pub max_upload_rate_pct: Option<u8>,

/// 
    #[serde(rename = "MaxUploadRateProvider")]
    pub max_upload_rate_provider: Option<DeliveryOptimizationConfig_MaxUploadRateProvider>,

/// 
    #[serde(rename = "UpLimitMonthlyGB")]
    pub up_limit_monthly_gb: Option<f64>,

/// 
    #[serde(rename = "UpLimitMonthlyGBProvider")]
    pub up_limit_monthly_gbprovider: Option<DeliveryOptimizationConfig_UpLimitMonthlyGBProvider>,
}

impl MSFT_DeliveryOptimizationConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DOBaseStatus::new(),
            down_back_limit_bps: None,
            down_back_limit_bps_provider: None,
            down_back_limit_pct: None,
            down_back_limit_pct_provider: None,
            down_fore_limit_bps: None,
            down_fore_limit_bps_provider: None,
            down_fore_limit_pct: None,
            down_fore_limit_pct_provider: None,
            download_mode: None,
            download_mode_provider: None,
            max_upload_rate_pct: None,
            max_upload_rate_provider: None,
            up_limit_monthly_gb: None,
            up_limit_monthly_gbprovider: None,
        }
    }


    /// Sets the value of DownBackLimitBps
    pub fn set_down_back_limit_bps(&mut self, value: u32) {
        self.down_back_limit_bps = Some(value);
    }

    /// Gets the value of DownBackLimitBps
    pub fn get_down_back_limit_bps(&self) -> Option<&u32> {
        self.down_back_limit_bps.as_ref()
    }

    /// Sets the value of DownBackLimitBpsProvider
    pub fn set_down_back_limit_bps_provider(&mut self, value: DeliveryOptimizationConfig_DownBackLimitBpsProvider) {
        self.down_back_limit_bps_provider = Some(value);
    }

    /// Gets the value of DownBackLimitBpsProvider
    pub fn get_down_back_limit_bps_provider(&self) -> Option<&DeliveryOptimizationConfig_DownBackLimitBpsProvider> {
        self.down_back_limit_bps_provider.as_ref()
    }

    /// Sets the value of DownBackLimitPct
    pub fn set_down_back_limit_pct(&mut self, value: u8) {
        self.down_back_limit_pct = Some(value);
    }

    /// Gets the value of DownBackLimitPct
    pub fn get_down_back_limit_pct(&self) -> Option<&u8> {
        self.down_back_limit_pct.as_ref()
    }

    /// Sets the value of DownBackLimitPctProvider
    pub fn set_down_back_limit_pct_provider(&mut self, value: DeliveryOptimizationConfig_DownBackLimitPctProvider) {
        self.down_back_limit_pct_provider = Some(value);
    }

    /// Gets the value of DownBackLimitPctProvider
    pub fn get_down_back_limit_pct_provider(&self) -> Option<&DeliveryOptimizationConfig_DownBackLimitPctProvider> {
        self.down_back_limit_pct_provider.as_ref()
    }

    /// Sets the value of DownForeLimitBps
    pub fn set_down_fore_limit_bps(&mut self, value: u32) {
        self.down_fore_limit_bps = Some(value);
    }

    /// Gets the value of DownForeLimitBps
    pub fn get_down_fore_limit_bps(&self) -> Option<&u32> {
        self.down_fore_limit_bps.as_ref()
    }

    /// Sets the value of DownForeLimitBpsProvider
    pub fn set_down_fore_limit_bps_provider(&mut self, value: DeliveryOptimizationConfig_DownForeLimitBpsProvider) {
        self.down_fore_limit_bps_provider = Some(value);
    }

    /// Gets the value of DownForeLimitBpsProvider
    pub fn get_down_fore_limit_bps_provider(&self) -> Option<&DeliveryOptimizationConfig_DownForeLimitBpsProvider> {
        self.down_fore_limit_bps_provider.as_ref()
    }

    /// Sets the value of DownForeLimitPct
    pub fn set_down_fore_limit_pct(&mut self, value: u8) {
        self.down_fore_limit_pct = Some(value);
    }

    /// Gets the value of DownForeLimitPct
    pub fn get_down_fore_limit_pct(&self) -> Option<&u8> {
        self.down_fore_limit_pct.as_ref()
    }

    /// Sets the value of DownForeLimitPctProvider
    pub fn set_down_fore_limit_pct_provider(&mut self, value: DeliveryOptimizationConfig_DownForeLimitPctProvider) {
        self.down_fore_limit_pct_provider = Some(value);
    }

    /// Gets the value of DownForeLimitPctProvider
    pub fn get_down_fore_limit_pct_provider(&self) -> Option<&DeliveryOptimizationConfig_DownForeLimitPctProvider> {
        self.down_fore_limit_pct_provider.as_ref()
    }

    /// Sets the value of DownloadMode
    pub fn set_download_mode(&mut self, value: DeliveryOptimizationConfig_DownloadMode) {
        self.download_mode = Some(value);
    }

    /// Gets the value of DownloadMode
    pub fn get_download_mode(&self) -> Option<&DeliveryOptimizationConfig_DownloadMode> {
        self.download_mode.as_ref()
    }

    /// Sets the value of DownloadModeProvider
    pub fn set_download_mode_provider(&mut self, value: DeliveryOptimizationConfig_DownloadModeProvider) {
        self.download_mode_provider = Some(value);
    }

    /// Gets the value of DownloadModeProvider
    pub fn get_download_mode_provider(&self) -> Option<&DeliveryOptimizationConfig_DownloadModeProvider> {
        self.download_mode_provider.as_ref()
    }

    /// Sets the value of MaxUploadRatePct
    pub fn set_max_upload_rate_pct(&mut self, value: u8) {
        self.max_upload_rate_pct = Some(value);
    }

    /// Gets the value of MaxUploadRatePct
    pub fn get_max_upload_rate_pct(&self) -> Option<&u8> {
        self.max_upload_rate_pct.as_ref()
    }

    /// Sets the value of MaxUploadRateProvider
    pub fn set_max_upload_rate_provider(&mut self, value: DeliveryOptimizationConfig_MaxUploadRateProvider) {
        self.max_upload_rate_provider = Some(value);
    }

    /// Gets the value of MaxUploadRateProvider
    pub fn get_max_upload_rate_provider(&self) -> Option<&DeliveryOptimizationConfig_MaxUploadRateProvider> {
        self.max_upload_rate_provider.as_ref()
    }

    /// Sets the value of UpLimitMonthlyGB
    pub fn set_up_limit_monthly_gb(&mut self, value: f64) {
        self.up_limit_monthly_gb = Some(value);
    }

    /// Gets the value of UpLimitMonthlyGB
    pub fn get_up_limit_monthly_gb(&self) -> Option<&f64> {
        self.up_limit_monthly_gb.as_ref()
    }

    /// Sets the value of UpLimitMonthlyGBProvider
    pub fn set_up_limit_monthly_gbprovider(&mut self, value: DeliveryOptimizationConfig_UpLimitMonthlyGBProvider) {
        self.up_limit_monthly_gbprovider = Some(value);
    }

    /// Gets the value of UpLimitMonthlyGBProvider
    pub fn get_up_limit_monthly_gbprovider(&self) -> Option<&DeliveryOptimizationConfig_UpLimitMonthlyGBProvider> {
        self.up_limit_monthly_gbprovider.as_ref()
    }

/// 22

    /// * `background` -  (bool)
    /// * `limit_bps` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_download_rate_limit_bps(&self, background: bool, limit_bps: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "background".to_string(), value: background.into() });
        args.push(MethodParameter { name: "limitBps".to_string(), value: limit_bps.into() });
        self.invoke_method("SetDownloadRateLimitBps", &args)

    }


/// 23

    /// * `background` -  (bool)
    /// * `limit_pct` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_download_rate_limit_pct(&self, background: bool, limit_pct: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "background".to_string(), value: background.into() });
        args.push(MethodParameter { name: "limitPct".to_string(), value: limit_pct.into() });
        self.invoke_method("SetDownloadRateLimitPct", &args)

    }


/// 

    /// * `download_mode` -  (DeliveryOptimizationConfig_downloadMode)

    /// * `return_value` -  (u32)
    pub fn set_download_mode(&self, download_mode: DeliveryOptimizationConfig_downloadMode) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "downloadMode".to_string(), value: download_mode.into() });
        self.invoke_method("SetDownloadMode", &args)

    }


/// 24

    /// * `limit_pct` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set_upload_rate_limit_pct(&self, limit_pct: u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "limitPct".to_string(), value: limit_pct.into() });
        self.invoke_method("SetUploadRateLimitPct", &args)

    }


/// 25

    /// * `limit_gb` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_monthly_upload_limit(&self, limit_gb: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "limitGB".to_string(), value: limit_gb.into() });
        self.invoke_method("SetMonthlyUploadLimit", &args)

    }

}

