// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcTransactionsTraceSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcTransactionsTraceSettings {

/// 
    #[serde(rename = "AbortedTransactionsTracingEnabled")]
    pub aborted_transactions_tracing_enabled: Option<bool>,

/// 
    #[serde(rename = "AllTransactionsTracingEnabled")]
    pub all_transactions_tracing_enabled: Option<bool>,

/// 
    #[serde(rename = "LongLivedTransactionsTracingEnabled")]
    pub long_lived_transactions_tracing_enabled: Option<bool>,
}

impl DtcTransactionsTraceSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            aborted_transactions_tracing_enabled: None,
            all_transactions_tracing_enabled: None,
            long_lived_transactions_tracing_enabled: None,
        }
    }


    /// Sets the value of AbortedTransactionsTracingEnabled
    pub fn set_aborted_transactions_tracing_enabled(&mut self, value: bool) {
        self.aborted_transactions_tracing_enabled = Some(value);
    }

    /// Gets the value of AbortedTransactionsTracingEnabled
    pub fn get_aborted_transactions_tracing_enabled(&self) -> Option<&bool> {
        self.aborted_transactions_tracing_enabled.as_ref()
    }

    /// Sets the value of AllTransactionsTracingEnabled
    pub fn set_all_transactions_tracing_enabled(&mut self, value: bool) {
        self.all_transactions_tracing_enabled = Some(value);
    }

    /// Gets the value of AllTransactionsTracingEnabled
    pub fn get_all_transactions_tracing_enabled(&self) -> Option<&bool> {
        self.all_transactions_tracing_enabled.as_ref()
    }

    /// Sets the value of LongLivedTransactionsTracingEnabled
    pub fn set_long_lived_transactions_tracing_enabled(&mut self, value: bool) {
        self.long_lived_transactions_tracing_enabled = Some(value);
    }

    /// Gets the value of LongLivedTransactionsTracingEnabled
    pub fn get_long_lived_transactions_tracing_enabled(&self) -> Option<&bool> {
        self.long_lived_transactions_tracing_enabled.as_ref()
    }
}

