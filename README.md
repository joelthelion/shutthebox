# Shut the box!

Shut the Box! is a [traditional game](https://www.youtube.com/watch?v=DBtEtcivvIU) with a simple but not trivial strategy. At each turn, the player rolls two dice and must then "shut" tiles corresponding to the dice roll. For example, if the player rolls 8, he could choose to shut 8, 7+1, 3+5, 1+2+5, etc. If the player can't do this, the game stops and the player scores the total of remaining tiles. The goal is to have the smallest score possible.

## Reinforcement learning

[Reinforcement learning](https://en.wikipedia.org/wiki/Reinforcement_learning) is a family of machine learning methods that look to train an *agent* to perform a task. Recently, deep learning has been applied with great success to reinforcement learning, leading to state-of-the-art performance to various tasks: board games (Go, chess), video games (Atari, starcraft, LoL), and more serious tasks as well. Applying it to the real world remains challenging even for experts due to a variety of reasons.

For this experiment, we use a very simple approach called tabular Q-learning. In Q-learning, we try to learn the Q function, which is the mapping from (state,action) pairs to a future *reward*. The reward is a numerical value associated to a particular game outcome. Here, we simply use the score (inverted in order to get a positive reward).

States are just particular game settings that can occur during a game. At each non-terminal state, the player can choose between various legal moves, or actions. Q-learning attempts to learn the expected reward for all combinations of a game state and a chosen action.

The Q function can be represented and approximated in different ways. For games with very large or infinite state spaces, some universal function approximator such as neural nets is often used. In this case, however, we can simply store all values in a big table. That makes the algorithm simpler and it also makes it simpler to get convergence.

## Training

How do we learn the Q-function? 
