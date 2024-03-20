Feature: ResourceCapability
	Validate resource capabilities

    @integration_test @capability @processor
    Scenario: Validate processor capability
        Given hyper-v host running on localhost
	    When I get resource capability for "Processor"
	    Then the default setting path should be "Processor Default Settings"
	    | Name                     | Value |
        | VirtualQuantity          | 1     |
        | MaxProcessorsPerNumaNode | 24    |
        | ResourceType             | 3     |

    @integration_test @capability @memory
    Scenario: Validate memory capability
        Given hyper-v host running on localhost
        When I get resource capability for "Memory"
        Then the default setting path should be "Memory Default Settings"
          | Name         | Value   |
          | Reservation  | 512     |
          | Limit        | 1048576 |
          | ResourceType | 4       |