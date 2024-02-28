# Mastery Test 1
Olof Tingskull
2023-02-28

## Ad Nauseam
### Pseudo Code
```c
function CanAllMachinesBeRestarted(r, f):
  // Calculate the total time required to restart all machines
  Define total_r = r * (Length Of f)

  // Sort the array of functioning times in ascending order
  Sort Array f In Ascending Order

  // Iterate over the sorted array to check if the task can be completed
  For Each Machine i in f:
    // Calculate if the current machines functioning time 
    // plus the time required to restart all subsequent machines (including itself) 
    // is greater than the total restart time for all machines
    If f[i] + (i + 1) * r <= total_r:
      // If the condition is not met for any machine, the task cannot be completed
      Return False

  // If the condition is met for all machines, the task can be completed
  Return True
```

## Caveat Venditor
### Psudo Code
```c
// Define Offer with properties: free, buy 
Define Offer with properties: free, buy 

// Define Payment with properties: pay_for_amount, using_offers 
Define Payment with properties: pay_for_amount, using_offers 
        
// Function to find the best price for DVDs using offers
Function FindBestPrice(total_to_buy, offers, price_per_movie):
    // Initialize DP array to find the optimal payment strategy
    Initialize quantity_payment_dp 
        = Array Of Payment Of Size (total_to_buy + 1) With 
            pay_for_amount = 0
            using_offers = Empty Set

    // Iterate through each possible purchase quantity
    For current_quantity = 1 Through total_to_buy Inclusive:
        // Gather all payment options for the current quantity
        Initialize payment_options = Empty Array Of Payment 

        // Incremental purchase without using any new offers
        no_offer_payment = quantity_payment_dp Index (current_quantity - 1)
        Increment pay_for_amount In no_offer_payment By One

        // Include the no-offer scenario as a valid option
        Append no_offer_payment To payment_options

        // Evaluate each offers applicability and benefit
        For Each offer In offers:
            // Direct application of an offer if it covers the current quantity
            If current_quantity <= offer.buy:
                Initialize new_payment = New Payment With 
                    pay_for_amount = (offer.buy - offer.free)
                    using_offers = Set Including offer

                Append new_payment To payment_options

            // Combine offers with previous payments for larger quantities
            Else:
                Initialize remaining_amount = current_quantity - offer.buy
                Initialize remaining_payment = quantity_payment_dp Index remaining_amount

                If offer Not In remaining_payment.using_offers:
                    Initialize new_payment = Copy remaining_payment
                    
                    Increment pay_for_amount By (offer.buy - offer.free) In new_payment 
                    Append offer To using_offers In new_payment

                    Append new_payment to payment_options
    
        // Select the payment option that minimizes cost
        Initialize best_payment = Min pay_for_amount in payment_options
        quantity_payment_dp Index current_quantity = best_payment
                
    // Calculate the total cost based on the optimal payment strategy
    Initialize best_payment = quantity_payment_dp Index total_to_buy
    Return (pay_for_amount In best_payment) * price_per_movie
```

## Coniunctis Viribus
### Psudo Code
```c
// Define Train structure with two properties: start and end indices of a train
Define Train With Properties: start, end

// Main function to calculate the minimum number of trains based on capacities
Function run(capacities):
    // Determine the total number of carriages
    Define n = Length Of capacities

    // Initialize dynamic programming table for storing optimal train configurations
    Initialize trains_dp = Array With Size n + 1 Of Optional Array Of Train As None
    // Base case: no carriages means no trains needed
    Set trains_dp[0] = Empty Array Of Train

    // Iterate through each carriage to explore starting a train from this point
    For i = 0 Through n Exclusive:
        // Retrieve the best train configuration up to carriage i
        Initialize start_at_train = trains_dp[i]
        // Skip if no configuration is found (should not happen after initialization)
        If start_at_train Is None: Continue

        // Reset capacity for the new potential train starting at i
        Initialize current_capacity = 0

        // Explore extending the train from i to j
        For j = i+1 Through n Inclusive:
            // Accumulate capacities to support the train from i to j
            Increment current_capacity by capacities[j-1]
            // Calculate the length and maintenance requirement for the current train
            Initialize current_train_length = j - i
            Initialize maintenance = current_train_length ^ 2

            // Continue to next possibility if capacity does not meet maintenance
            If current_capacity Is Less Than maintenance: Continue

            // Create a new train configuration by adding the train from i to j
            Initialize trains_to_here = Copy start_at_train
            Append (Train With start = i, end = j) To trains_to_here
            
            // Check if this is the shortest configuration for reaching carriage j
            Initialize so_far_shortest_here = trains_dp[j]
            // Update trains_dp[j] with the current configuration if it's shorter
            If so_far_shortest_here is None Or Length Of trains_to_here < Length Of so_far_shortest_here:
                Set trains_dp[j] = trains_to_here

    // Return the optimal train configuration for all n carriages
    Return trains_dp[n]
```

## Quid Pro Quo
### Psudo Code
´´´c
Define Structure Trade with properties: with, get, give

Define Structure State with properties:
    player_resources: Array Of Integer
    character_resources: Array Of Integer
    character_goals: Array Of Integer

Function get_surplus(resources, goals):
    Return Array Of Resources - Goals For Each Resource

Function hash(state):
    Calculate and return a unique hash for the state

Function recursive_trade(state, explored_states):
    Define (
        player_resources, 
        character_resources, 
        character_goals
    ) = state

    Define n = Length Of player_resources
    Define m = Length Of character_resources
    
    Initialize state_hash = Run hash Function With state
    If state_hash is in explored_states: Return False

    Add state_hash To explored_states

    If All In player_resources Is Greater Than 0:
        Return True

    If Sum Of player_resources Is Less Than n: 
        Return False

    Initialize possible_trades = Empty Array Of Trade
    Initialize character_surplus 
        = Run get_surplus Function With character_resources, character_goals

    For i = 0 Through n Exclusive:
        Initialize player_get = player_resources[i]
        If player_get Is Greater Than 0: Continue

        Initialize can_trade_this_resource = False

        For j = 0 Through n Exclusive:
            If i Is Equal To j: Continue

            Initialize player_give = player_resources[j]
            If player_give Is Less Than 1: Continue

            For k = 0 Through m Exclusive:
                Initialize npc_give_surplus = character_surplus[k][i]
                Initialize npc_get_surplus = character_surplus[k][j]

                If npc_give_surplus Is Less Than 1: Continue
                If npc_get_surplus Is Greater Than -1: Continue

                Initialize new_trade = New Trade With 
                    with = k, 
                    get = i, 
                    give = j

                Append new_trade To possible_trades
                Set can_trade_this_resource = True
        
        If Not can_trade_this_resource:
            Return False

    For trade In possible_trades:
        Define (
            with, 
            get,
            give
        ) = trade

        Initialize player_resource_after_trade = Copy player_resources
        Increment player_resource_after_trade[get] By 1
        Decrement player_resource_after_trade[give] By 1

        Initialize character_resource_after_trade = Copy character_resources
        Increment character_resource_after_trade[with][give] By 1
        Decrement character_resource_after_trade[with][get] By 1

        Initialize new_state = New State With 
            player_resources = player_resource_after_trade, 
            character_resources = character_resource_after_trade, 
            character_goals = character_goals

        Initialize trade_successful
            = Run recursive_trade Function With new_state, explored_states

        If trade_successful:
            Return True
        Else:
            Continue

    Return false

Function PlayerCanGetAllResources(player_resources, character_resources, character_goals):
    Initialize state = New State With 
        player_resources = player_resources,
        character_resources = character_resources,
        character_goals = character_goals

    Initialize explored_states = Empty Set Of Hashes

    Return 
        Run recursive_trade Function With state, explored_states
´´´