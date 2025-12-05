// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PrinterNfcTagTasks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PrinterNfcTagTasks {
}

impl MSFT_PrinterNfcTagTasks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `lock` -  (bool)
    /// * `share_path` -  (String[])
    /// * `wsd_address` -  (String[])

    /// * `return_value` -  (u32)
    pub fn write_by_manual_specification(&self, share_path: &Vec<String>, wsd_address: &Vec<String>, lock: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SharePath".to_string(), value: share_path.into() });
        args.push(MethodParameter { name: "WsdAddress".to_string(), value: wsd_address.into() });
        args.push(MethodParameter { name: "Lock".to_string(), value: lock.into() });
        self.invoke_method("WriteByManualSpecification", &args)

    }


/// 

    /// * `input_object` -  (MSFT_PrinterNfcTag)

    /// * `return_value` -  (u32)
    pub fn write_by_printer_nfc_tag(&self, input_object: MSFT_PrinterNfcTag) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputObject".to_string(), value: input_object.into() });
        self.invoke_method("WriteByPrinterNfcTag", &args)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_PrinterNfcTag)
    /// * `return_value` -  (u32)
    pub fn read(&self, cmdlet_output: &mut MSFT_PrinterNfcTag) -> Result<(), WmiError> {

        let result = self.invoke_method("Read", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

