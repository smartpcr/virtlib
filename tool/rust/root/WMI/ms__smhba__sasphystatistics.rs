// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_SASPHYSTATISTICS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_SASPHYSTATISTICS {

/// 
    #[serde(rename = "InvalidDwordCount")]
    pub invalid_dword_count: Option<i64>,

/// 
    #[serde(rename = "LossofDwordSyncCount")]
    pub lossof_dword_sync_count: Option<i64>,

/// 
    #[serde(rename = "PhyResetProblemCount")]
    pub phy_reset_problem_count: Option<i64>,

/// 
    #[serde(rename = "RunningDisparityErrorCount")]
    pub running_disparity_error_count: Option<i64>,

/// 
    #[serde(rename = "RxFrames")]
    pub rx_frames: Option<i64>,

/// 
    #[serde(rename = "RxWords")]
    pub rx_words: Option<i64>,

/// 
    #[serde(rename = "SecondsSinceLastReset")]
    pub seconds_since_last_reset: Option<i64>,

/// 
    #[serde(rename = "TxFrames")]
    pub tx_frames: Option<i64>,

/// 
    #[serde(rename = "TxWords")]
    pub tx_words: Option<i64>,
}

impl MS_SMHBA_SASPHYSTATISTICS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            invalid_dword_count: None,
            lossof_dword_sync_count: None,
            phy_reset_problem_count: None,
            running_disparity_error_count: None,
            rx_frames: None,
            rx_words: None,
            seconds_since_last_reset: None,
            tx_frames: None,
            tx_words: None,
        }
    }


    /// Sets the value of InvalidDwordCount
    pub fn set_invalid_dword_count(&mut self, value: i64) {
        self.invalid_dword_count = Some(value);
    }

    /// Gets the value of InvalidDwordCount
    pub fn get_invalid_dword_count(&self) -> Option<&i64> {
        self.invalid_dword_count.as_ref()
    }

    /// Sets the value of LossofDwordSyncCount
    pub fn set_lossof_dword_sync_count(&mut self, value: i64) {
        self.lossof_dword_sync_count = Some(value);
    }

    /// Gets the value of LossofDwordSyncCount
    pub fn get_lossof_dword_sync_count(&self) -> Option<&i64> {
        self.lossof_dword_sync_count.as_ref()
    }

    /// Sets the value of PhyResetProblemCount
    pub fn set_phy_reset_problem_count(&mut self, value: i64) {
        self.phy_reset_problem_count = Some(value);
    }

    /// Gets the value of PhyResetProblemCount
    pub fn get_phy_reset_problem_count(&self) -> Option<&i64> {
        self.phy_reset_problem_count.as_ref()
    }

    /// Sets the value of RunningDisparityErrorCount
    pub fn set_running_disparity_error_count(&mut self, value: i64) {
        self.running_disparity_error_count = Some(value);
    }

    /// Gets the value of RunningDisparityErrorCount
    pub fn get_running_disparity_error_count(&self) -> Option<&i64> {
        self.running_disparity_error_count.as_ref()
    }

    /// Sets the value of RxFrames
    pub fn set_rx_frames(&mut self, value: i64) {
        self.rx_frames = Some(value);
    }

    /// Gets the value of RxFrames
    pub fn get_rx_frames(&self) -> Option<&i64> {
        self.rx_frames.as_ref()
    }

    /// Sets the value of RxWords
    pub fn set_rx_words(&mut self, value: i64) {
        self.rx_words = Some(value);
    }

    /// Gets the value of RxWords
    pub fn get_rx_words(&self) -> Option<&i64> {
        self.rx_words.as_ref()
    }

    /// Sets the value of SecondsSinceLastReset
    pub fn set_seconds_since_last_reset(&mut self, value: i64) {
        self.seconds_since_last_reset = Some(value);
    }

    /// Gets the value of SecondsSinceLastReset
    pub fn get_seconds_since_last_reset(&self) -> Option<&i64> {
        self.seconds_since_last_reset.as_ref()
    }

    /// Sets the value of TxFrames
    pub fn set_tx_frames(&mut self, value: i64) {
        self.tx_frames = Some(value);
    }

    /// Gets the value of TxFrames
    pub fn get_tx_frames(&self) -> Option<&i64> {
        self.tx_frames.as_ref()
    }

    /// Sets the value of TxWords
    pub fn set_tx_words(&mut self, value: i64) {
        self.tx_words = Some(value);
    }

    /// Gets the value of TxWords
    pub fn get_tx_words(&self) -> Option<&i64> {
        self.tx_words.as_ref()
    }
}

