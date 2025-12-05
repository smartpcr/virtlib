// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_GuestFileService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_GuestFileService {
    #[serde(flatten)]
    pub base: Msvm_GuestService,
}

impl Msvm_GuestFileService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_GuestService::new(),
        }
    }


/// 

    /// * `copy_file_to_guest_settings` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn copy_files_to_guest(&self, copy_file_to_guest_settings: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CopyFileToGuestSettings".to_string(), value: copy_file_to_guest_settings.into() });

        let result = self.invoke_method_with_job("CopyFilesToGuest", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

