// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_MSDTC_DistributedTransactionCoordinator struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_MSDTC_DistributedTransactionCoordinator {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AbortedTransactions")]
    pub aborted_transactions: Option<u32>,

/// 
    #[serde(rename = "AbortedTransactionsPersec")]
    pub aborted_transactions_persec: Option<u32>,

/// 
    #[serde(rename = "ActiveTransactions")]
    pub active_transactions: Option<u32>,

/// 
    #[serde(rename = "ActiveTransactionsMaximum")]
    pub active_transactions_maximum: Option<u32>,

/// 
    #[serde(rename = "CommittedTransactions")]
    pub committed_transactions: Option<u32>,

/// 
    #[serde(rename = "CommittedTransactionsPersec")]
    pub committed_transactions_persec: Option<u32>,

/// 
    #[serde(rename = "ForceAbortedTransactions")]
    pub force_aborted_transactions: Option<u32>,

/// 
    #[serde(rename = "ForceCommittedTransactions")]
    pub force_committed_transactions: Option<u32>,

/// 
    #[serde(rename = "InDoubtTransactions")]
    pub in_doubt_transactions: Option<u32>,

/// 
    #[serde(rename = "ResponseTimeAverage")]
    pub response_time_average: Option<u32>,

/// 
    #[serde(rename = "ResponseTimeMaximum")]
    pub response_time_maximum: Option<u32>,

/// 
    #[serde(rename = "ResponseTimeMinimum")]
    pub response_time_minimum: Option<u32>,

/// 
    #[serde(rename = "TransactionsPersec")]
    pub transactions_persec: Option<u32>,
}

impl Win32_PerfRawData_MSDTC_DistributedTransactionCoordinator {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            aborted_transactions: None,
            aborted_transactions_persec: None,
            active_transactions: None,
            active_transactions_maximum: None,
            committed_transactions: None,
            committed_transactions_persec: None,
            force_aborted_transactions: None,
            force_committed_transactions: None,
            in_doubt_transactions: None,
            response_time_average: None,
            response_time_maximum: None,
            response_time_minimum: None,
            transactions_persec: None,
        }
    }


    /// Sets the value of AbortedTransactions
    pub fn set_aborted_transactions(&mut self, value: u32) {
        self.aborted_transactions = Some(value);
    }

    /// Gets the value of AbortedTransactions
    pub fn get_aborted_transactions(&self) -> Option<&u32> {
        self.aborted_transactions.as_ref()
    }

    /// Sets the value of AbortedTransactionsPersec
    pub fn set_aborted_transactions_persec(&mut self, value: u32) {
        self.aborted_transactions_persec = Some(value);
    }

    /// Gets the value of AbortedTransactionsPersec
    pub fn get_aborted_transactions_persec(&self) -> Option<&u32> {
        self.aborted_transactions_persec.as_ref()
    }

    /// Sets the value of ActiveTransactions
    pub fn set_active_transactions(&mut self, value: u32) {
        self.active_transactions = Some(value);
    }

    /// Gets the value of ActiveTransactions
    pub fn get_active_transactions(&self) -> Option<&u32> {
        self.active_transactions.as_ref()
    }

    /// Sets the value of ActiveTransactionsMaximum
    pub fn set_active_transactions_maximum(&mut self, value: u32) {
        self.active_transactions_maximum = Some(value);
    }

    /// Gets the value of ActiveTransactionsMaximum
    pub fn get_active_transactions_maximum(&self) -> Option<&u32> {
        self.active_transactions_maximum.as_ref()
    }

    /// Sets the value of CommittedTransactions
    pub fn set_committed_transactions(&mut self, value: u32) {
        self.committed_transactions = Some(value);
    }

    /// Gets the value of CommittedTransactions
    pub fn get_committed_transactions(&self) -> Option<&u32> {
        self.committed_transactions.as_ref()
    }

    /// Sets the value of CommittedTransactionsPersec
    pub fn set_committed_transactions_persec(&mut self, value: u32) {
        self.committed_transactions_persec = Some(value);
    }

    /// Gets the value of CommittedTransactionsPersec
    pub fn get_committed_transactions_persec(&self) -> Option<&u32> {
        self.committed_transactions_persec.as_ref()
    }

    /// Sets the value of ForceAbortedTransactions
    pub fn set_force_aborted_transactions(&mut self, value: u32) {
        self.force_aborted_transactions = Some(value);
    }

    /// Gets the value of ForceAbortedTransactions
    pub fn get_force_aborted_transactions(&self) -> Option<&u32> {
        self.force_aborted_transactions.as_ref()
    }

    /// Sets the value of ForceCommittedTransactions
    pub fn set_force_committed_transactions(&mut self, value: u32) {
        self.force_committed_transactions = Some(value);
    }

    /// Gets the value of ForceCommittedTransactions
    pub fn get_force_committed_transactions(&self) -> Option<&u32> {
        self.force_committed_transactions.as_ref()
    }

    /// Sets the value of InDoubtTransactions
    pub fn set_in_doubt_transactions(&mut self, value: u32) {
        self.in_doubt_transactions = Some(value);
    }

    /// Gets the value of InDoubtTransactions
    pub fn get_in_doubt_transactions(&self) -> Option<&u32> {
        self.in_doubt_transactions.as_ref()
    }

    /// Sets the value of ResponseTimeAverage
    pub fn set_response_time_average(&mut self, value: u32) {
        self.response_time_average = Some(value);
    }

    /// Gets the value of ResponseTimeAverage
    pub fn get_response_time_average(&self) -> Option<&u32> {
        self.response_time_average.as_ref()
    }

    /// Sets the value of ResponseTimeMaximum
    pub fn set_response_time_maximum(&mut self, value: u32) {
        self.response_time_maximum = Some(value);
    }

    /// Gets the value of ResponseTimeMaximum
    pub fn get_response_time_maximum(&self) -> Option<&u32> {
        self.response_time_maximum.as_ref()
    }

    /// Sets the value of ResponseTimeMinimum
    pub fn set_response_time_minimum(&mut self, value: u32) {
        self.response_time_minimum = Some(value);
    }

    /// Gets the value of ResponseTimeMinimum
    pub fn get_response_time_minimum(&self) -> Option<&u32> {
        self.response_time_minimum.as_ref()
    }

    /// Sets the value of TransactionsPersec
    pub fn set_transactions_persec(&mut self, value: u32) {
        self.transactions_persec = Some(value);
    }

    /// Gets the value of TransactionsPersec
    pub fn get_transactions_persec(&self) -> Option<&u32> {
        self.transactions_persec.as_ref()
    }
}

