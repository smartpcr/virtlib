// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VssService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VssService {
    #[serde(flatten)]
    pub base: Msvm_GuestService,
}

impl Msvm_VssService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_GuestService::new(),
        }
    }


/// 

    /// * `guest_cluster_information` - Guest cluster information returned (Msvm_GuestClusterInformation)
    /// * `return_value` -  (u32)
    pub fn query_guest_cluster_information(&self, guest_cluster_information: &mut Msvm_GuestClusterInformation) -> Result<(), WmiError> {

        let result = self.invoke_method("QueryGuestClusterInformation", &[])?;
        let guest_cluster_information = result.get_value("GuestClusterInformation")?;
        Ok(result.return_value)

    }

}

