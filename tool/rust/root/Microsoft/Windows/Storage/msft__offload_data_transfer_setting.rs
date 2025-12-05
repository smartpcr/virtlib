// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_OffloadDataTransferSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_OffloadDataTransferSetting {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "NumberOfTokensInUse")]
    pub number_of_tokens_in_use: Option<u32>,

/// 
    #[serde(rename = "NumberOfTokensMax")]
    pub number_of_tokens_max: Option<u32>,

/// 
    #[serde(rename = "OptimalDataTokenSize")]
    pub optimal_data_token_size: Option<u32>,

/// 
    #[serde(rename = "SupportInterSubsystem")]
    pub support_inter_subsystem: Option<bool>,
}

impl MSFT_OffloadDataTransferSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            number_of_tokens_in_use: None,
            number_of_tokens_max: None,
            optimal_data_token_size: None,
            support_inter_subsystem: None,
        }
    }


    /// Sets the value of NumberOfTokensInUse
    pub fn set_number_of_tokens_in_use(&mut self, value: u32) {
        self.number_of_tokens_in_use = Some(value);
    }

    /// Gets the value of NumberOfTokensInUse
    pub fn get_number_of_tokens_in_use(&self) -> Option<&u32> {
        self.number_of_tokens_in_use.as_ref()
    }

    /// Sets the value of NumberOfTokensMax
    pub fn set_number_of_tokens_max(&mut self, value: u32) {
        self.number_of_tokens_max = Some(value);
    }

    /// Gets the value of NumberOfTokensMax
    pub fn get_number_of_tokens_max(&self) -> Option<&u32> {
        self.number_of_tokens_max.as_ref()
    }

    /// Sets the value of OptimalDataTokenSize
    pub fn set_optimal_data_token_size(&mut self, value: u32) {
        self.optimal_data_token_size = Some(value);
    }

    /// Gets the value of OptimalDataTokenSize
    pub fn get_optimal_data_token_size(&self) -> Option<&u32> {
        self.optimal_data_token_size.as_ref()
    }

    /// Sets the value of SupportInterSubsystem
    pub fn set_support_inter_subsystem(&mut self, value: bool) {
        self.support_inter_subsystem = Some(value);
    }

    /// Gets the value of SupportInterSubsystem
    pub fn get_support_inter_subsystem(&self) -> Option<&bool> {
        self.support_inter_subsystem.as_ref()
    }
}

