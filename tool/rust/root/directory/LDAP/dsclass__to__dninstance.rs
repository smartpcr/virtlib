// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DSClass_To_DNInstance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DSClass_To_DNInstance {

/// 
    #[serde(rename = "DSClass")]
    pub dsclass: Option<String>,

/// 
    #[serde(rename = "RootDNForSearchAndQuery")]
    pub root_dnfor_search_and_query: Option<DN_Class>,
}

impl DSClass_To_DNInstance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dsclass: None,
            root_dnfor_search_and_query: None,
        }
    }


    /// Sets the value of DSClass
    pub fn set_dsclass(&mut self, value: String) {
        self.dsclass = Some(value);
    }

    /// Gets the value of DSClass
    pub fn get_dsclass(&self) -> Option<&String> {
        self.dsclass.as_ref()
    }

    /// Sets the value of RootDNForSearchAndQuery
    pub fn set_root_dnfor_search_and_query(&mut self, value: DN_Class) {
        self.root_dnfor_search_and_query = Some(value);
    }

    /// Gets the value of RootDNForSearchAndQuery
    pub fn get_root_dnfor_search_and_query(&self) -> Option<&DN_Class> {
        self.root_dnfor_search_and_query.as_ref()
    }
}

