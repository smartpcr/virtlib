// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_RemoteProcedureCall02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_RemoteProcedureCall02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RestrictUnauthenticatedRPCClients")]
    pub restrict_unauthenticated_rpcclients: Option<String>,

/// 
    #[serde(rename = "RPCEndpointMapperClientAuthentication")]
    pub rpcendpoint_mapper_client_authentication: Option<String>,
}

impl MDM_Policy_Result01_RemoteProcedureCall02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            restrict_unauthenticated_rpcclients: None,
            rpcendpoint_mapper_client_authentication: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RestrictUnauthenticatedRPCClients
    pub fn set_restrict_unauthenticated_rpcclients(&mut self, value: String) {
        self.restrict_unauthenticated_rpcclients = Some(value);
    }

    /// Gets the value of RestrictUnauthenticatedRPCClients
    pub fn get_restrict_unauthenticated_rpcclients(&self) -> Option<&String> {
        self.restrict_unauthenticated_rpcclients.as_ref()
    }

    /// Sets the value of RPCEndpointMapperClientAuthentication
    pub fn set_rpcendpoint_mapper_client_authentication(&mut self, value: String) {
        self.rpcendpoint_mapper_client_authentication = Some(value);
    }

    /// Gets the value of RPCEndpointMapperClientAuthentication
    pub fn get_rpcendpoint_mapper_client_authentication(&self) -> Option<&String> {
        self.rpcendpoint_mapper_client_authentication.as_ref()
    }
}

