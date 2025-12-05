// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NetworkService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NetworkService {
    #[serde(flatten)]
    pub base: CIM_Service,

/// This is a free-form array of strings that provide descriptive words and phrases that can be used in queries. To-date, this property has not been implemented, since it is not standardized. Also, if this was a necessary query construct, then it would be required higher in the inheritance hierarchy. The latter has not proven necessary. Therefore, the property is deprecated.
    #[serde(rename = "Keywords")]
    pub keywords: Vec<String>,

/// This is a URL that provides the protocol, network location, and other service-specific information required in order to access the service. It is deprecated with the recommendation that ServiceAccessURI be instantiated instead. This new class correctly positions the semantics of the service access, and clarifies the format of the information.
    #[serde(rename = "ServiceURL")]
    pub service_url: Option<String>,

/// This is a free-form array of strings that specify any specific pre-conditions that must be met in order for this service to start correctly. It was expected that subclasses would refine the inherited StartService() method to suit their specific needs. To-date, this refinement has not been necessary. Also, the property is not very useful, since it is not standardized. If this was a necessary construct, then it would be required higher in the inheritance hierarchy (on Service). The latter has not proven true. Therefore, the property is deprecated.
    #[serde(rename = "StartupConditions")]
    pub startup_conditions: Vec<String>,

/// This is a free-form array of strings that specify any specific parameters that must be supplied to the StartService() method in order for this service to start correctly. It was expected that subclasses would refine the inherited StartService() methods to suit their specific needs. To-date, this refinement has not been necessary. If indeed the method were refined, then its parameters would more formally convey this information. Therefore, the property is deprecated.
    #[serde(rename = "StartupParameters")]
    pub startup_parameters: Vec<String>,
}

impl CIM_NetworkService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
            keywords: Vec::new(),
            service_url: None,
            startup_conditions: Vec::new(),
            startup_parameters: Vec::new(),
        }
    }


    /// Sets the value of Keywords
    pub fn set_keywords(&mut self, value: Vec<String>) {
        self.keywords = value;
    }

    /// Gets the value of Keywords
    pub fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    /// Sets the value of ServiceURL
    pub fn set_service_url(&mut self, value: String) {
        self.service_url = Some(value);
    }

    /// Gets the value of ServiceURL
    pub fn get_service_url(&self) -> Option<&String> {
        self.service_url.as_ref()
    }

    /// Sets the value of StartupConditions
    pub fn set_startup_conditions(&mut self, value: Vec<String>) {
        self.startup_conditions = value;
    }

    /// Gets the value of StartupConditions
    pub fn get_startup_conditions(&self) -> &Vec<String> {
        &self.startup_conditions
    }

    /// Sets the value of StartupParameters
    pub fn set_startup_parameters(&mut self, value: Vec<String>) {
        self.startup_parameters = value;
    }

    /// Gets the value of StartupParameters
    pub fn get_startup_parameters(&self) -> &Vec<String> {
        &self.startup_parameters
    }
}

