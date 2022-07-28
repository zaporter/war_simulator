# War Card Game Simulator

This is super simple simulator for the card game "War". I have followed the [bicycle cards rules](https://bicyclecards.com/how-to-play/war/) when implementing this game. 

This has lead to the following stats after running 10000 games:

Mean rounds per game = 178.0

Stddev of rounds per game = 135.98

Mean wars per game = 13.3

Probability that player1 will win: 0.4955


Similarly, we can create the following histogram for the number of rounds to expect to play for a game:

![histogram](https://github.com/zaporter/war_simulator/blob/master/rounds_dist.png)



(I am using the word "round" to indicate the period when both players place down their cards up until there is a winner for the round and the winner collects their cards and they may begin a new round. Each round may contain multiple "wars")


# Purpose

This serves no purpose whatsoever and didn't even teach me any new Rust. I just thought it was a fun idea.
