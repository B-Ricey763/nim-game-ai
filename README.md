# Nim with AI

![nim game with ai](<nim game ai thumbnail.png>)

Nim is a subtraction game where two players alternately take one or more objects from one of a number of stacks, where the last player who moves wins the game.

I replicated this game's functionality within the terminal, and you can play with 2 human players or play alone against the computer.

## Installation
Head to the [releases](https://github.com/B-Ricey763/nim-game-ai/releases/latest) and download the binary for your OS, and you're good to run it! If your antivirus complains, just know the code does not do anything malicious, but you can verify yourself by looking at the code. 

Alternatively, you can install using cargo:
```bash
cargo install nim-game-ai
```
And run by just opening a terminal and entering `nim-game-ai`

## How the AI works
Because of funky math involved with combinatorial game theory that I barely understand myself, you are able to do a 'nim sum' of the numbers to get a value. When the nim sum is 0, no move can keep it at 0, and when it isn't 0, there is one or more moves that can bring it to 0. If a player can keep the nim sum of the game at 0 at each move, they win the game. The nim sum is calculated by converting the number of items in each stack into binary, and then applying the XOR (exclusive or) operator to each number. 

Since the calculation for the nim sum is a simple logical operator, it's quite trivial to create an AI to play the game. The way my specific implemetation works is by minimizing the nim sum, optimally always reaching 0. However, a very skilled player can choose P1 or P2 depending on the stack setup and technically beat the AI if they play optimally. The game prints out the nim sum at each turn as well. 
