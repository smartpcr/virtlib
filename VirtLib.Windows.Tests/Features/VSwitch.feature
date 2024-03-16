Feature: VSwitch
	Validate virtual switch logic

    @integration_test @network
    Scenario: Get virtual switch on hyper-v host
	    Given hyper-v host running on localhost
	    When I get virtual switch with name "Default Switch"
	    Then the virtual switch should exist
	    And the virtual switch connection type should be "Internal"
	    And the virtual switch vlan should be disabled