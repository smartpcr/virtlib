// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// StackWalk_Event struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StackWalk_Event {
    #[serde(flatten)]
    pub base: StackWalk,

/// 
    #[serde(rename = "EventTimeStamp")]
    pub event_time_stamp: Option<u64>,

/// 
    #[serde(rename = "Stack1")]
    pub stack1: Option<u32>,

/// 
    #[serde(rename = "Stack10")]
    pub stack10: Option<u32>,

/// 
    #[serde(rename = "Stack11")]
    pub stack11: Option<u32>,

/// 
    #[serde(rename = "Stack12")]
    pub stack12: Option<u32>,

/// 
    #[serde(rename = "Stack13")]
    pub stack13: Option<u32>,

/// 
    #[serde(rename = "Stack14")]
    pub stack14: Option<u32>,

/// 
    #[serde(rename = "Stack15")]
    pub stack15: Option<u32>,

/// 
    #[serde(rename = "Stack16")]
    pub stack16: Option<u32>,

/// 
    #[serde(rename = "Stack17")]
    pub stack17: Option<u32>,

/// 
    #[serde(rename = "Stack18")]
    pub stack18: Option<u32>,

/// 
    #[serde(rename = "Stack19")]
    pub stack19: Option<u32>,

/// 
    #[serde(rename = "Stack2")]
    pub stack2: Option<u32>,

/// 
    #[serde(rename = "Stack20")]
    pub stack20: Option<u32>,

/// 
    #[serde(rename = "Stack21")]
    pub stack21: Option<u32>,

/// 
    #[serde(rename = "Stack22")]
    pub stack22: Option<u32>,

/// 
    #[serde(rename = "Stack23")]
    pub stack23: Option<u32>,

/// 
    #[serde(rename = "Stack24")]
    pub stack24: Option<u32>,

/// 
    #[serde(rename = "Stack25")]
    pub stack25: Option<u32>,

/// 
    #[serde(rename = "Stack26")]
    pub stack26: Option<u32>,

/// 
    #[serde(rename = "Stack27")]
    pub stack27: Option<u32>,

/// 
    #[serde(rename = "Stack28")]
    pub stack28: Option<u32>,

/// 
    #[serde(rename = "Stack29")]
    pub stack29: Option<u32>,

/// 
    #[serde(rename = "Stack3")]
    pub stack3: Option<u32>,

/// 
    #[serde(rename = "Stack30")]
    pub stack30: Option<u32>,

/// 
    #[serde(rename = "Stack31")]
    pub stack31: Option<u32>,

/// 
    #[serde(rename = "Stack32")]
    pub stack32: Option<u32>,

/// 
    #[serde(rename = "Stack4")]
    pub stack4: Option<u32>,

/// 
    #[serde(rename = "Stack5")]
    pub stack5: Option<u32>,

/// 
    #[serde(rename = "Stack6")]
    pub stack6: Option<u32>,

/// 
    #[serde(rename = "Stack7")]
    pub stack7: Option<u32>,

/// 
    #[serde(rename = "Stack8")]
    pub stack8: Option<u32>,

/// 
    #[serde(rename = "Stack9")]
    pub stack9: Option<u32>,

/// 
    #[serde(rename = "StackProcess")]
    pub stack_process: Option<u32>,

/// 
    #[serde(rename = "StackThread")]
    pub stack_thread: Option<u32>,
}

impl StackWalk_Event {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: StackWalk::new(),
            event_time_stamp: None,
            stack1: None,
            stack10: None,
            stack11: None,
            stack12: None,
            stack13: None,
            stack14: None,
            stack15: None,
            stack16: None,
            stack17: None,
            stack18: None,
            stack19: None,
            stack2: None,
            stack20: None,
            stack21: None,
            stack22: None,
            stack23: None,
            stack24: None,
            stack25: None,
            stack26: None,
            stack27: None,
            stack28: None,
            stack29: None,
            stack3: None,
            stack30: None,
            stack31: None,
            stack32: None,
            stack4: None,
            stack5: None,
            stack6: None,
            stack7: None,
            stack8: None,
            stack9: None,
            stack_process: None,
            stack_thread: None,
        }
    }


    /// Sets the value of EventTimeStamp
    pub fn set_event_time_stamp(&mut self, value: u64) {
        self.event_time_stamp = Some(value);
    }

    /// Gets the value of EventTimeStamp
    pub fn get_event_time_stamp(&self) -> Option<&u64> {
        self.event_time_stamp.as_ref()
    }

    /// Sets the value of Stack1
    pub fn set_stack1(&mut self, value: u32) {
        self.stack1 = Some(value);
    }

    /// Gets the value of Stack1
    pub fn get_stack1(&self) -> Option<&u32> {
        self.stack1.as_ref()
    }

    /// Sets the value of Stack10
    pub fn set_stack10(&mut self, value: u32) {
        self.stack10 = Some(value);
    }

    /// Gets the value of Stack10
    pub fn get_stack10(&self) -> Option<&u32> {
        self.stack10.as_ref()
    }

    /// Sets the value of Stack11
    pub fn set_stack11(&mut self, value: u32) {
        self.stack11 = Some(value);
    }

    /// Gets the value of Stack11
    pub fn get_stack11(&self) -> Option<&u32> {
        self.stack11.as_ref()
    }

    /// Sets the value of Stack12
    pub fn set_stack12(&mut self, value: u32) {
        self.stack12 = Some(value);
    }

    /// Gets the value of Stack12
    pub fn get_stack12(&self) -> Option<&u32> {
        self.stack12.as_ref()
    }

    /// Sets the value of Stack13
    pub fn set_stack13(&mut self, value: u32) {
        self.stack13 = Some(value);
    }

    /// Gets the value of Stack13
    pub fn get_stack13(&self) -> Option<&u32> {
        self.stack13.as_ref()
    }

    /// Sets the value of Stack14
    pub fn set_stack14(&mut self, value: u32) {
        self.stack14 = Some(value);
    }

    /// Gets the value of Stack14
    pub fn get_stack14(&self) -> Option<&u32> {
        self.stack14.as_ref()
    }

    /// Sets the value of Stack15
    pub fn set_stack15(&mut self, value: u32) {
        self.stack15 = Some(value);
    }

    /// Gets the value of Stack15
    pub fn get_stack15(&self) -> Option<&u32> {
        self.stack15.as_ref()
    }

    /// Sets the value of Stack16
    pub fn set_stack16(&mut self, value: u32) {
        self.stack16 = Some(value);
    }

    /// Gets the value of Stack16
    pub fn get_stack16(&self) -> Option<&u32> {
        self.stack16.as_ref()
    }

    /// Sets the value of Stack17
    pub fn set_stack17(&mut self, value: u32) {
        self.stack17 = Some(value);
    }

    /// Gets the value of Stack17
    pub fn get_stack17(&self) -> Option<&u32> {
        self.stack17.as_ref()
    }

    /// Sets the value of Stack18
    pub fn set_stack18(&mut self, value: u32) {
        self.stack18 = Some(value);
    }

    /// Gets the value of Stack18
    pub fn get_stack18(&self) -> Option<&u32> {
        self.stack18.as_ref()
    }

    /// Sets the value of Stack19
    pub fn set_stack19(&mut self, value: u32) {
        self.stack19 = Some(value);
    }

    /// Gets the value of Stack19
    pub fn get_stack19(&self) -> Option<&u32> {
        self.stack19.as_ref()
    }

    /// Sets the value of Stack2
    pub fn set_stack2(&mut self, value: u32) {
        self.stack2 = Some(value);
    }

    /// Gets the value of Stack2
    pub fn get_stack2(&self) -> Option<&u32> {
        self.stack2.as_ref()
    }

    /// Sets the value of Stack20
    pub fn set_stack20(&mut self, value: u32) {
        self.stack20 = Some(value);
    }

    /// Gets the value of Stack20
    pub fn get_stack20(&self) -> Option<&u32> {
        self.stack20.as_ref()
    }

    /// Sets the value of Stack21
    pub fn set_stack21(&mut self, value: u32) {
        self.stack21 = Some(value);
    }

    /// Gets the value of Stack21
    pub fn get_stack21(&self) -> Option<&u32> {
        self.stack21.as_ref()
    }

    /// Sets the value of Stack22
    pub fn set_stack22(&mut self, value: u32) {
        self.stack22 = Some(value);
    }

    /// Gets the value of Stack22
    pub fn get_stack22(&self) -> Option<&u32> {
        self.stack22.as_ref()
    }

    /// Sets the value of Stack23
    pub fn set_stack23(&mut self, value: u32) {
        self.stack23 = Some(value);
    }

    /// Gets the value of Stack23
    pub fn get_stack23(&self) -> Option<&u32> {
        self.stack23.as_ref()
    }

    /// Sets the value of Stack24
    pub fn set_stack24(&mut self, value: u32) {
        self.stack24 = Some(value);
    }

    /// Gets the value of Stack24
    pub fn get_stack24(&self) -> Option<&u32> {
        self.stack24.as_ref()
    }

    /// Sets the value of Stack25
    pub fn set_stack25(&mut self, value: u32) {
        self.stack25 = Some(value);
    }

    /// Gets the value of Stack25
    pub fn get_stack25(&self) -> Option<&u32> {
        self.stack25.as_ref()
    }

    /// Sets the value of Stack26
    pub fn set_stack26(&mut self, value: u32) {
        self.stack26 = Some(value);
    }

    /// Gets the value of Stack26
    pub fn get_stack26(&self) -> Option<&u32> {
        self.stack26.as_ref()
    }

    /// Sets the value of Stack27
    pub fn set_stack27(&mut self, value: u32) {
        self.stack27 = Some(value);
    }

    /// Gets the value of Stack27
    pub fn get_stack27(&self) -> Option<&u32> {
        self.stack27.as_ref()
    }

    /// Sets the value of Stack28
    pub fn set_stack28(&mut self, value: u32) {
        self.stack28 = Some(value);
    }

    /// Gets the value of Stack28
    pub fn get_stack28(&self) -> Option<&u32> {
        self.stack28.as_ref()
    }

    /// Sets the value of Stack29
    pub fn set_stack29(&mut self, value: u32) {
        self.stack29 = Some(value);
    }

    /// Gets the value of Stack29
    pub fn get_stack29(&self) -> Option<&u32> {
        self.stack29.as_ref()
    }

    /// Sets the value of Stack3
    pub fn set_stack3(&mut self, value: u32) {
        self.stack3 = Some(value);
    }

    /// Gets the value of Stack3
    pub fn get_stack3(&self) -> Option<&u32> {
        self.stack3.as_ref()
    }

    /// Sets the value of Stack30
    pub fn set_stack30(&mut self, value: u32) {
        self.stack30 = Some(value);
    }

    /// Gets the value of Stack30
    pub fn get_stack30(&self) -> Option<&u32> {
        self.stack30.as_ref()
    }

    /// Sets the value of Stack31
    pub fn set_stack31(&mut self, value: u32) {
        self.stack31 = Some(value);
    }

    /// Gets the value of Stack31
    pub fn get_stack31(&self) -> Option<&u32> {
        self.stack31.as_ref()
    }

    /// Sets the value of Stack32
    pub fn set_stack32(&mut self, value: u32) {
        self.stack32 = Some(value);
    }

    /// Gets the value of Stack32
    pub fn get_stack32(&self) -> Option<&u32> {
        self.stack32.as_ref()
    }

    /// Sets the value of Stack4
    pub fn set_stack4(&mut self, value: u32) {
        self.stack4 = Some(value);
    }

    /// Gets the value of Stack4
    pub fn get_stack4(&self) -> Option<&u32> {
        self.stack4.as_ref()
    }

    /// Sets the value of Stack5
    pub fn set_stack5(&mut self, value: u32) {
        self.stack5 = Some(value);
    }

    /// Gets the value of Stack5
    pub fn get_stack5(&self) -> Option<&u32> {
        self.stack5.as_ref()
    }

    /// Sets the value of Stack6
    pub fn set_stack6(&mut self, value: u32) {
        self.stack6 = Some(value);
    }

    /// Gets the value of Stack6
    pub fn get_stack6(&self) -> Option<&u32> {
        self.stack6.as_ref()
    }

    /// Sets the value of Stack7
    pub fn set_stack7(&mut self, value: u32) {
        self.stack7 = Some(value);
    }

    /// Gets the value of Stack7
    pub fn get_stack7(&self) -> Option<&u32> {
        self.stack7.as_ref()
    }

    /// Sets the value of Stack8
    pub fn set_stack8(&mut self, value: u32) {
        self.stack8 = Some(value);
    }

    /// Gets the value of Stack8
    pub fn get_stack8(&self) -> Option<&u32> {
        self.stack8.as_ref()
    }

    /// Sets the value of Stack9
    pub fn set_stack9(&mut self, value: u32) {
        self.stack9 = Some(value);
    }

    /// Gets the value of Stack9
    pub fn get_stack9(&self) -> Option<&u32> {
        self.stack9.as_ref()
    }

    /// Sets the value of StackProcess
    pub fn set_stack_process(&mut self, value: u32) {
        self.stack_process = Some(value);
    }

    /// Gets the value of StackProcess
    pub fn get_stack_process(&self) -> Option<&u32> {
        self.stack_process.as_ref()
    }

    /// Sets the value of StackThread
    pub fn set_stack_thread(&mut self, value: u32) {
        self.stack_thread = Some(value);
    }

    /// Gets the value of StackThread
    pub fn get_stack_thread(&self) -> Option<&u32> {
        self.stack_thread.as_ref()
    }
}

