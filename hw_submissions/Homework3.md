# Homework 3

Version 0.0.1 w/ Latest Commit - https://github.com/swamulism/aether_of_enclaves/releases/tag/0.0.1

1. What was planned for this deadline, from our design document:
    - Generate basic graphical interface with simple menu, using Piston (one class/file).
    - Beginnings of the Command class (input handler) written and incorporate with above.
    - Have skeleton of Game loop written.
    - Have a list of NPCs/items we would like to include.

2. What was accomplished:
    - The game successfully draws a game window using Piston with a basic "player" on screen, who can be controlled using WASD. The interface updates with user input, including WASD, Tab (to "open the menu"), and Return (to "begin the game"). The screen additionally writes text to the screen. We never defined exactly what the "simple menu" we wanted to implement would be, so we are counting this a success -- especially because writing text to the screen was a pain in the butt.
    - The basic Game loop is written to update with user input and display the screen. It will need to be flushed out to handle game events.
    - The Command programming design pattern was applied in the input_handler object. The Input Handler changes the game state according to the input (e.g. toggling between the menu and the gameplay, or moving the player.)
    - Additionally, we implemented the beginnings of a number of objects we hadn't intended to at this point, including a Player object. Most of these (graphics, map, and items) are not required / used in compilation at the current moment.
    - The list of NPCs and Items we'll aim to implement include:
    	- Chef, Fighter, Crafter
    	- Wood (material - crafting), Dagger (weapon), Grune (material - cooking), Bisket (food - yes the spelling is intentional)
    - Sam made a wiki (lol).

3. Planned for next deadline (we will continue as originally planned as we are on schedule):
   - Implementation of Airship and Player class integrated with the Game.
   - Graphics code for Airship and Player written in Flyweight class.
   - Testing for Airship, Player, and Input class developed.
   - If possible, beginnings of enumerations / object for NPC classes.

4. Screenshots below.

![title](hw3_title.png)


![ingame](hw3_ingame.png)


![menu](hw3_menu.png)
