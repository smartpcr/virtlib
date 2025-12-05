// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ClusteringService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ClusteringService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl CIM_ClusteringService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `cs` -  (CIM_ComputerSystem)

    /// * `return_value` -  (u32)
    pub fn add_node(&self, cs: CIM_ComputerSystem) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CS".to_string(), value: cs.into() });
        self.invoke_method("AddNode", &args)

    }


/// 

    /// * `cs` -  (CIM_ComputerSystem)

    /// * `return_value` -  (u32)
    pub fn evict_node(&self, cs: CIM_ComputerSystem) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CS".to_string(), value: cs.into() });
        self.invoke_method("EvictNode", &args)

    }

}

