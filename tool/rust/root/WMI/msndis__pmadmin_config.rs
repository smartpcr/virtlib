// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PMAdminConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PMAdminConfig {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "PMAdminConfigParam")]
    pub pmadmin_config_param: Option<MSNdis_PMAdminConfigParam>,
}

impl MSNdis_PMAdminConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
            pmadmin_config_param: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of PMAdminConfigParam
    pub fn set_pmadmin_config_param(&mut self, value: MSNdis_PMAdminConfigParam) {
        self.pmadmin_config_param = Some(value);
    }

    /// Gets the value of PMAdminConfigParam
    pub fn get_pmadmin_config_param(&self) -> Option<&MSNdis_PMAdminConfigParam> {
        self.pmadmin_config_param.as_ref()
    }
}

