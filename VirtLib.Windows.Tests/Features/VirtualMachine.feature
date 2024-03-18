Feature: VirtualMachine
	Validate VM logic and functionality

    @integration_test @vm
    Scenario: Get virtual machine
        Given hyper-v host running on localhost
	    When I get vm with name "devbox1"
	    Then vm should exist with name "devbox1"