// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceAppDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceAppDomain {

/// Contains properties of the appdomain.
    #[serde(rename = "AppDomainInfo")]
    pub app_domain_info: Option<AppDomainInfo>,

/// The service of this appdomain.
    #[serde(rename = "Service")]
    pub service: Option<Service>,
}

impl ServiceAppDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_domain_info: None,
            service: None,
        }
    }


    /// Sets the value of AppDomainInfo
    pub fn set_app_domain_info(&mut self, value: AppDomainInfo) {
        self.app_domain_info = Some(value);
    }

    /// Gets the value of AppDomainInfo
    pub fn get_app_domain_info(&self) -> Option<&AppDomainInfo> {
        self.app_domain_info.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: Service) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&Service> {
        self.service.as_ref()
    }
}

