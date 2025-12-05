// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_GuestCommunicationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_GuestCommunicationService {
    #[serde(flatten)]
    pub base: Msvm_GuestService,
}

impl Msvm_GuestCommunicationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_GuestService::new(),
        }
    }

}

