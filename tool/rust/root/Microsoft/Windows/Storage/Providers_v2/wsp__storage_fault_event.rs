// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSP_StorageFaultEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSP_StorageFaultEvent {
    #[serde(flatten)]
    pub base: MSFT_StorageFaultEvent,
}

impl WSP_StorageFaultEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageFaultEvent::new(),
        }
    }


/// This method manually fires alerts

    /// * `fault_alert` - Copy of the alert payload to be fired (WSP_StorageFaultEvent)

    /// * `return_value` -  (u32)
    pub fn fire_alert(&self, fault_alert: WSP_StorageFaultEvent) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FaultAlert".to_string(), value: fault_alert.into() });
        self.invoke_method("FireAlert", &args)

    }

}

