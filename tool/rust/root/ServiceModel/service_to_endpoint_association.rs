// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceToEndpointAssociation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceToEndpointAssociation {

/// The endpoint associated with the service.
    #[serde(rename = "Endpoint")]
    pub endpoint: Option<Endpoint>,

/// The service associated with the endpoint.
    #[serde(rename = "Service")]
    pub service: Option<Service>,
}

impl ServiceToEndpointAssociation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            endpoint: None,
            service: None,
        }
    }


    /// Sets the value of Endpoint
    pub fn set_endpoint(&mut self, value: Endpoint) {
        self.endpoint = Some(value);
    }

    /// Gets the value of Endpoint
    pub fn get_endpoint(&self) -> Option<&Endpoint> {
        self.endpoint.as_ref()
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

