// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcTransactionsTraceSessionTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcTransactionsTraceSessionTask {
}

impl MSFT_DtcTransactionsTraceSessionTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `return_value` -  (u32)
    pub fn stop(&self) -> Result<(), WmiError> {
        self.invoke_method("Stop", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn write(&self) -> Result<(), WmiError> {
        self.invoke_method("Write", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn start(&self) -> Result<(), WmiError> {
        self.invoke_method("Start", &[])

    }


/// 

    /// * `cmdlet_output` -  (DtcTransactionsTraceSession)
    /// * `return_value` -  (u32)
    pub fn get(&self, cmdlet_output: &mut DtcTransactionsTraceSession) -> Result<(), WmiError> {

        let result = self.invoke_method("Get", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `buffer_count` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set(&self, buffer_count: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BufferCount".to_string(), value: buffer_count.into() });
        self.invoke_method("Set", &args)

    }

}

