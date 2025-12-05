// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Registry_Counters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Registry_Counters {
    #[serde(flatten)]
    pub base: Registry,

/// 
    #[serde(rename = "Counter1")]
    pub counter1: Option<u64>,

/// 
    #[serde(rename = "Counter10")]
    pub counter10: Option<u64>,

/// 
    #[serde(rename = "Counter11")]
    pub counter11: Option<u64>,

/// 
    #[serde(rename = "Counter2")]
    pub counter2: Option<u64>,

/// 
    #[serde(rename = "Counter3")]
    pub counter3: Option<u64>,

/// 
    #[serde(rename = "Counter4")]
    pub counter4: Option<u64>,

/// 
    #[serde(rename = "Counter5")]
    pub counter5: Option<u64>,

/// 
    #[serde(rename = "Counter6")]
    pub counter6: Option<u64>,

/// 
    #[serde(rename = "Counter7")]
    pub counter7: Option<u64>,

/// 
    #[serde(rename = "Counter8")]
    pub counter8: Option<u64>,

/// 
    #[serde(rename = "Counter9")]
    pub counter9: Option<u64>,
}

impl Registry_Counters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Registry::new(),
            counter1: None,
            counter10: None,
            counter11: None,
            counter2: None,
            counter3: None,
            counter4: None,
            counter5: None,
            counter6: None,
            counter7: None,
            counter8: None,
            counter9: None,
        }
    }


    /// Sets the value of Counter1
    pub fn set_counter1(&mut self, value: u64) {
        self.counter1 = Some(value);
    }

    /// Gets the value of Counter1
    pub fn get_counter1(&self) -> Option<&u64> {
        self.counter1.as_ref()
    }

    /// Sets the value of Counter10
    pub fn set_counter10(&mut self, value: u64) {
        self.counter10 = Some(value);
    }

    /// Gets the value of Counter10
    pub fn get_counter10(&self) -> Option<&u64> {
        self.counter10.as_ref()
    }

    /// Sets the value of Counter11
    pub fn set_counter11(&mut self, value: u64) {
        self.counter11 = Some(value);
    }

    /// Gets the value of Counter11
    pub fn get_counter11(&self) -> Option<&u64> {
        self.counter11.as_ref()
    }

    /// Sets the value of Counter2
    pub fn set_counter2(&mut self, value: u64) {
        self.counter2 = Some(value);
    }

    /// Gets the value of Counter2
    pub fn get_counter2(&self) -> Option<&u64> {
        self.counter2.as_ref()
    }

    /// Sets the value of Counter3
    pub fn set_counter3(&mut self, value: u64) {
        self.counter3 = Some(value);
    }

    /// Gets the value of Counter3
    pub fn get_counter3(&self) -> Option<&u64> {
        self.counter3.as_ref()
    }

    /// Sets the value of Counter4
    pub fn set_counter4(&mut self, value: u64) {
        self.counter4 = Some(value);
    }

    /// Gets the value of Counter4
    pub fn get_counter4(&self) -> Option<&u64> {
        self.counter4.as_ref()
    }

    /// Sets the value of Counter5
    pub fn set_counter5(&mut self, value: u64) {
        self.counter5 = Some(value);
    }

    /// Gets the value of Counter5
    pub fn get_counter5(&self) -> Option<&u64> {
        self.counter5.as_ref()
    }

    /// Sets the value of Counter6
    pub fn set_counter6(&mut self, value: u64) {
        self.counter6 = Some(value);
    }

    /// Gets the value of Counter6
    pub fn get_counter6(&self) -> Option<&u64> {
        self.counter6.as_ref()
    }

    /// Sets the value of Counter7
    pub fn set_counter7(&mut self, value: u64) {
        self.counter7 = Some(value);
    }

    /// Gets the value of Counter7
    pub fn get_counter7(&self) -> Option<&u64> {
        self.counter7.as_ref()
    }

    /// Sets the value of Counter8
    pub fn set_counter8(&mut self, value: u64) {
        self.counter8 = Some(value);
    }

    /// Gets the value of Counter8
    pub fn get_counter8(&self) -> Option<&u64> {
        self.counter8.as_ref()
    }

    /// Sets the value of Counter9
    pub fn set_counter9(&mut self, value: u64) {
        self.counter9 = Some(value);
    }

    /// Gets the value of Counter9
    pub fn get_counter9(&self) -> Option<&u64> {
        self.counter9.as_ref()
    }
}

