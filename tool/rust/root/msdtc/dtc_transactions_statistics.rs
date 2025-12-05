// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcTransactionsStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcTransactionsStatistics {

/// 
    #[serde(rename = "Aborted")]
    pub aborted: Option<u32>,

/// 
    #[serde(rename = "AbortedMax")]
    pub aborted_max: Option<u32>,

/// 
    #[serde(rename = "Committed")]
    pub committed: Option<u32>,

/// 
    #[serde(rename = "CommittedMax")]
    pub committed_max: Option<u32>,

/// 
    #[serde(rename = "ForcedAbort")]
    pub forced_abort: Option<u32>,

/// 
    #[serde(rename = "ForcedCommit")]
    pub forced_commit: Option<u32>,

/// 
    #[serde(rename = "Heuristic")]
    pub heuristic: Option<u32>,

/// 
    #[serde(rename = "HeuristicMax")]
    pub heuristic_max: Option<u32>,

/// 
    #[serde(rename = "InDoubt")]
    pub in_doubt: Option<u32>,

/// 
    #[serde(rename = "InDoubtMax")]
    pub in_doubt_max: Option<u32>,

/// 
    #[serde(rename = "Open")]
    pub open: Option<u32>,

/// 
    #[serde(rename = "OpenMax")]
    pub open_max: Option<u32>,

/// 
    #[serde(rename = "SinglePhaseInDoubt")]
    pub single_phase_in_doubt: Option<u32>,
}

impl DtcTransactionsStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            aborted: None,
            aborted_max: None,
            committed: None,
            committed_max: None,
            forced_abort: None,
            forced_commit: None,
            heuristic: None,
            heuristic_max: None,
            in_doubt: None,
            in_doubt_max: None,
            open: None,
            open_max: None,
            single_phase_in_doubt: None,
        }
    }


    /// Sets the value of Aborted
    pub fn set_aborted(&mut self, value: u32) {
        self.aborted = Some(value);
    }

    /// Gets the value of Aborted
    pub fn get_aborted(&self) -> Option<&u32> {
        self.aborted.as_ref()
    }

    /// Sets the value of AbortedMax
    pub fn set_aborted_max(&mut self, value: u32) {
        self.aborted_max = Some(value);
    }

    /// Gets the value of AbortedMax
    pub fn get_aborted_max(&self) -> Option<&u32> {
        self.aborted_max.as_ref()
    }

    /// Sets the value of Committed
    pub fn set_committed(&mut self, value: u32) {
        self.committed = Some(value);
    }

    /// Gets the value of Committed
    pub fn get_committed(&self) -> Option<&u32> {
        self.committed.as_ref()
    }

    /// Sets the value of CommittedMax
    pub fn set_committed_max(&mut self, value: u32) {
        self.committed_max = Some(value);
    }

    /// Gets the value of CommittedMax
    pub fn get_committed_max(&self) -> Option<&u32> {
        self.committed_max.as_ref()
    }

    /// Sets the value of ForcedAbort
    pub fn set_forced_abort(&mut self, value: u32) {
        self.forced_abort = Some(value);
    }

    /// Gets the value of ForcedAbort
    pub fn get_forced_abort(&self) -> Option<&u32> {
        self.forced_abort.as_ref()
    }

    /// Sets the value of ForcedCommit
    pub fn set_forced_commit(&mut self, value: u32) {
        self.forced_commit = Some(value);
    }

    /// Gets the value of ForcedCommit
    pub fn get_forced_commit(&self) -> Option<&u32> {
        self.forced_commit.as_ref()
    }

    /// Sets the value of Heuristic
    pub fn set_heuristic(&mut self, value: u32) {
        self.heuristic = Some(value);
    }

    /// Gets the value of Heuristic
    pub fn get_heuristic(&self) -> Option<&u32> {
        self.heuristic.as_ref()
    }

    /// Sets the value of HeuristicMax
    pub fn set_heuristic_max(&mut self, value: u32) {
        self.heuristic_max = Some(value);
    }

    /// Gets the value of HeuristicMax
    pub fn get_heuristic_max(&self) -> Option<&u32> {
        self.heuristic_max.as_ref()
    }

    /// Sets the value of InDoubt
    pub fn set_in_doubt(&mut self, value: u32) {
        self.in_doubt = Some(value);
    }

    /// Gets the value of InDoubt
    pub fn get_in_doubt(&self) -> Option<&u32> {
        self.in_doubt.as_ref()
    }

    /// Sets the value of InDoubtMax
    pub fn set_in_doubt_max(&mut self, value: u32) {
        self.in_doubt_max = Some(value);
    }

    /// Gets the value of InDoubtMax
    pub fn get_in_doubt_max(&self) -> Option<&u32> {
        self.in_doubt_max.as_ref()
    }

    /// Sets the value of Open
    pub fn set_open(&mut self, value: u32) {
        self.open = Some(value);
    }

    /// Gets the value of Open
    pub fn get_open(&self) -> Option<&u32> {
        self.open.as_ref()
    }

    /// Sets the value of OpenMax
    pub fn set_open_max(&mut self, value: u32) {
        self.open_max = Some(value);
    }

    /// Gets the value of OpenMax
    pub fn get_open_max(&self) -> Option<&u32> {
        self.open_max.as_ref()
    }

    /// Sets the value of SinglePhaseInDoubt
    pub fn set_single_phase_in_doubt(&mut self, value: u32) {
        self.single_phase_in_doubt = Some(value);
    }

    /// Gets the value of SinglePhaseInDoubt
    pub fn get_single_phase_in_doubt(&self) -> Option<&u32> {
        self.single_phase_in_doubt.as_ref()
    }
}

