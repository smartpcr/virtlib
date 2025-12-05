// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcTransactionsTraceSettingTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcTransactionsTraceSettingTask {
}

impl MSFT_DtcTransactionsTraceSettingTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `cmdlet_output` -  (DtcTransactionsTraceSettings)
    /// * `return_value` -  (u32)
    pub fn get(&self, cmdlet_output: &mut DtcTransactionsTraceSettings) -> Result<(), WmiError> {

        let result = self.invoke_method("Get", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `all_transactions_tracing_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_by_trace_all_parameter_set(&self, all_transactions_tracing_enabled: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AllTransactionsTracingEnabled".to_string(), value: all_transactions_tracing_enabled.into() });
        self.invoke_method("SetByTraceAllParameterSet", &args)

    }


/// 

    /// * `aborted_transactions_tracing_enabled` -  (bool)
    /// * `long_lived_transactions_tracing_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_by_trace_selected_parameter_set(&self, aborted_transactions_tracing_enabled: bool, long_lived_transactions_tracing_enabled: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AbortedTransactionsTracingEnabled".to_string(), value: aborted_transactions_tracing_enabled.into() });
        args.push(MethodParameter { name: "LongLivedTransactionsTracingEnabled".to_string(), value: long_lived_transactions_tracing_enabled.into() });
        self.invoke_method("SetByTraceSelectedParameterSet", &args)

    }

}

