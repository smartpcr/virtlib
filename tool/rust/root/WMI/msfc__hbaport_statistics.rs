// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_HBAPortStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_HBAPortStatistics {

/// 
    #[serde(rename = "DumpedFrames")]
    pub dumped_frames: Option<i64>,

/// 
    #[serde(rename = "ErrorFrames")]
    pub error_frames: Option<i64>,

/// 
    #[serde(rename = "InvalidCRCCount")]
    pub invalid_crccount: Option<i64>,

/// 
    #[serde(rename = "InvalidTxWordCount")]
    pub invalid_tx_word_count: Option<i64>,

/// 
    #[serde(rename = "LinkFailureCount")]
    pub link_failure_count: Option<i64>,

/// 
    #[serde(rename = "LIPCount")]
    pub lipcount: Option<i64>,

/// 
    #[serde(rename = "LossOfSignalCount")]
    pub loss_of_signal_count: Option<i64>,

/// 
    #[serde(rename = "LossOfSyncCount")]
    pub loss_of_sync_count: Option<i64>,

/// 
    #[serde(rename = "NOSCount")]
    pub noscount: Option<i64>,

/// 
    #[serde(rename = "PrimitiveSeqProtocolErrCount")]
    pub primitive_seq_protocol_err_count: Option<i64>,

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

impl MSFC_HBAPortStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dumped_frames: None,
            error_frames: None,
            invalid_crccount: None,
            invalid_tx_word_count: None,
            link_failure_count: None,
            lipcount: None,
            loss_of_signal_count: None,
            loss_of_sync_count: None,
            noscount: None,
            primitive_seq_protocol_err_count: None,
            rx_frames: None,
            rx_words: None,
            seconds_since_last_reset: None,
            tx_frames: None,
            tx_words: None,
        }
    }


    /// Sets the value of DumpedFrames
    pub fn set_dumped_frames(&mut self, value: i64) {
        self.dumped_frames = Some(value);
    }

    /// Gets the value of DumpedFrames
    pub fn get_dumped_frames(&self) -> Option<&i64> {
        self.dumped_frames.as_ref()
    }

    /// Sets the value of ErrorFrames
    pub fn set_error_frames(&mut self, value: i64) {
        self.error_frames = Some(value);
    }

    /// Gets the value of ErrorFrames
    pub fn get_error_frames(&self) -> Option<&i64> {
        self.error_frames.as_ref()
    }

    /// Sets the value of InvalidCRCCount
    pub fn set_invalid_crccount(&mut self, value: i64) {
        self.invalid_crccount = Some(value);
    }

    /// Gets the value of InvalidCRCCount
    pub fn get_invalid_crccount(&self) -> Option<&i64> {
        self.invalid_crccount.as_ref()
    }

    /// Sets the value of InvalidTxWordCount
    pub fn set_invalid_tx_word_count(&mut self, value: i64) {
        self.invalid_tx_word_count = Some(value);
    }

    /// Gets the value of InvalidTxWordCount
    pub fn get_invalid_tx_word_count(&self) -> Option<&i64> {
        self.invalid_tx_word_count.as_ref()
    }

    /// Sets the value of LinkFailureCount
    pub fn set_link_failure_count(&mut self, value: i64) {
        self.link_failure_count = Some(value);
    }

    /// Gets the value of LinkFailureCount
    pub fn get_link_failure_count(&self) -> Option<&i64> {
        self.link_failure_count.as_ref()
    }

    /// Sets the value of LIPCount
    pub fn set_lipcount(&mut self, value: i64) {
        self.lipcount = Some(value);
    }

    /// Gets the value of LIPCount
    pub fn get_lipcount(&self) -> Option<&i64> {
        self.lipcount.as_ref()
    }

    /// Sets the value of LossOfSignalCount
    pub fn set_loss_of_signal_count(&mut self, value: i64) {
        self.loss_of_signal_count = Some(value);
    }

    /// Gets the value of LossOfSignalCount
    pub fn get_loss_of_signal_count(&self) -> Option<&i64> {
        self.loss_of_signal_count.as_ref()
    }

    /// Sets the value of LossOfSyncCount
    pub fn set_loss_of_sync_count(&mut self, value: i64) {
        self.loss_of_sync_count = Some(value);
    }

    /// Gets the value of LossOfSyncCount
    pub fn get_loss_of_sync_count(&self) -> Option<&i64> {
        self.loss_of_sync_count.as_ref()
    }

    /// Sets the value of NOSCount
    pub fn set_noscount(&mut self, value: i64) {
        self.noscount = Some(value);
    }

    /// Gets the value of NOSCount
    pub fn get_noscount(&self) -> Option<&i64> {
        self.noscount.as_ref()
    }

    /// Sets the value of PrimitiveSeqProtocolErrCount
    pub fn set_primitive_seq_protocol_err_count(&mut self, value: i64) {
        self.primitive_seq_protocol_err_count = Some(value);
    }

    /// Gets the value of PrimitiveSeqProtocolErrCount
    pub fn get_primitive_seq_protocol_err_count(&self) -> Option<&i64> {
        self.primitive_seq_protocol_err_count.as_ref()
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

