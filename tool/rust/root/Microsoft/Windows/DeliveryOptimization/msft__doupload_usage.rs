// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DOUploadUsage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DOUploadUsage {
    #[serde(flatten)]
    pub base: MSFT_DOUsage,

/// 
    #[serde(rename = "MonthlyUploadRestriction")]
    pub monthly_upload_restriction: Option<DOUploadUsage_MonthlyUploadRestriction>,

/// 
    #[serde(rename = "UploadRatePct")]
    pub upload_rate_pct: Option<u8>,

/// 
    #[serde(rename = "Uploads")]
    pub uploads: Option<u32>,
}

impl MSFT_DOUploadUsage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DOUsage::new(),
            monthly_upload_restriction: None,
            upload_rate_pct: None,
            uploads: None,
        }
    }


    /// Sets the value of MonthlyUploadRestriction
    pub fn set_monthly_upload_restriction(&mut self, value: DOUploadUsage_MonthlyUploadRestriction) {
        self.monthly_upload_restriction = Some(value);
    }

    /// Gets the value of MonthlyUploadRestriction
    pub fn get_monthly_upload_restriction(&self) -> Option<&DOUploadUsage_MonthlyUploadRestriction> {
        self.monthly_upload_restriction.as_ref()
    }

    /// Sets the value of UploadRatePct
    pub fn set_upload_rate_pct(&mut self, value: u8) {
        self.upload_rate_pct = Some(value);
    }

    /// Gets the value of UploadRatePct
    pub fn get_upload_rate_pct(&self) -> Option<&u8> {
        self.upload_rate_pct.as_ref()
    }

    /// Sets the value of Uploads
    pub fn set_uploads(&mut self, value: u32) {
        self.uploads = Some(value);
    }

    /// Gets the value of Uploads
    pub fn get_uploads(&self) -> Option<&u32> {
        self.uploads.as_ref()
    }
}

