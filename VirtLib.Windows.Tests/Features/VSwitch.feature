Feature: VSwitch
	Unit tests on virtual switch in hyper-v

@network
Scenario: Check virtual switch on hyper-v host
	Given hyper-v host running on localhost
	When I check virtual switch with name "Default Switch"
	Then the virtual switch should exist
	And the virtual switch connection type should be "External"
	And the virtual switch vlan should be disabled