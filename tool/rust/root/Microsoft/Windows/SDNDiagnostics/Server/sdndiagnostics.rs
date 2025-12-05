// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SDNDiagnostics.Server
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDNDiagnostics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDNDiagnostics {
}

impl SDNDiagnostics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `diagnostic_information` -  (DiagnosticInfo)

    /// * `change_in_error_code` -  (bool)
    /// * `error_code` -  (u32)
    /// * `return_value` -  (u32)
    pub fn enable(&self, diagnostic_information: DiagnosticInfo, error_code: &mut u32, change_in_error_code: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DiagnosticInformation".to_string(), value: diagnostic_information.into() });

        let result = self.invoke_method("Enable", &args)?;
        let change_in_error_code = result.get_value("ChangeInErrorCode")?;
        let error_code = result.get_value("ErrorCode")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn update_trace_providers_list(&self) -> Result<(), WmiError> {
        self.invoke_method("UpdateTraceProvidersList", &[])

    }

}

