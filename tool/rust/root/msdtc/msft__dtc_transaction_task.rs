// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcTransactionTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcTransactionTask {
}

impl MSFT_DtcTransactionTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `dtc_name` -  (String)

    /// * `cmdlet_output` -  (DtcTransactionInfo[])
    /// * `return_value` -  (u32)
    pub fn get(&self, dtc_name: &String, cmdlet_output: &mut Vec<DtcTransactionInfo>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `abort` -  (bool)
    /// * `dtc_name` -  (String)
    /// * `transaction_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_abort_set(&self, dtc_name: &String, transaction_id: &String, abort: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "TransactionId".to_string(), value: transaction_id.into() });
        args.push(MethodParameter { name: "Abort".to_string(), value: abort.into() });
        self.invoke_method("SetByAbortSet", &args)

    }


/// 

    /// * `commit` -  (bool)
    /// * `dtc_name` -  (String)
    /// * `transaction_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_commit_set(&self, dtc_name: &String, transaction_id: &String, commit: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "TransactionId".to_string(), value: transaction_id.into() });
        args.push(MethodParameter { name: "Commit".to_string(), value: commit.into() });
        self.invoke_method("SetByCommitSet", &args)

    }


/// 

    /// * `dtc_name` -  (String)
    /// * `forget` -  (bool)
    /// * `transaction_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_forget_set(&self, dtc_name: &String, transaction_id: &String, forget: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "TransactionId".to_string(), value: transaction_id.into() });
        args.push(MethodParameter { name: "Forget".to_string(), value: forget.into() });
        self.invoke_method("SetByForgetSet", &args)

    }


/// 

    /// * `dtc_name` -  (String)
    /// * `trace` -  (bool)
    /// * `transaction_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_trace_set(&self, dtc_name: &String, transaction_id: &String, trace: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "TransactionId".to_string(), value: transaction_id.into() });
        args.push(MethodParameter { name: "Trace".to_string(), value: trace.into() });
        self.invoke_method("SetByTraceSet", &args)

    }

}

