// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServerComponent_NET_Framework_45_ASPNET struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerComponent_NET_Framework_45_ASPNET {
    #[serde(flatten)]
    pub base: MSFT_ServerManagerServerComponentDescriptor,
}

impl ServerComponent_NET_Framework_45_ASPNET {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_ServerManagerServerComponentDescriptor::new(),
        }
    }

}

