// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_BridgeMgmt_Transaction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_BridgeMgmt_Transaction {
}

impl MDM_BridgeMgmt_Transaction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `request_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn transaction_begin(&self, request_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "requestId".to_string(), value: request_id.into() });
        self.invoke_method("TransactionBegin", &args)

    }


/// 

    /// * `request_id` -  (String)

    /// * `failed_request_details` -  (String)
    /// * `return_value` -  (u32)
    /// * `transaction_mi_result` -  (u32)
    pub fn transaction_end(&self, request_id: &String, transaction_mi_result: &mut u32, failed_request_details: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "requestId".to_string(), value: request_id.into() });

        let result = self.invoke_method("TransactionEnd", &args)?;
        let failed_request_details = result.get_value("failedRequestDetails")?;
        let transaction_mi_result = result.get_value("transactionMiResult")?;
        Ok(result.return_value)

    }

}

