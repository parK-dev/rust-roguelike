# parK's Dungeon Crawler
Dungeon Crawler Game Made in Rust



Features:![Screenshot from 2022-08-16 20-47-30](https://user-images.githubusercontent.com/77372584/184872345-69165545-2ef1-4183-8c7b-1f622e688ab0.png)

 - Entity Component System
 - Procedural Map Generation (Cellular Automata, DrunkardWalk, Room&Corridors)
 - AI Pathfinding for monsters to track player
 - Turn-based combat system
 - Field of vision (Player and Monster)
 - Items with effects such as weapons, maps and potions
 - Multiple levels
 - Data-driven entity spawning using a .ron template
 - Heads-up display for game information
 - Item toolbar
 - WASM-readiness for web deployment

To try the game you can:
- Download the park-dungeon-crawler.zip file and run the executable.
- Clone the repo and launch the game using ``` cargo run --release ``` (Requires Rust and Cargo to be installed on your machine)

Controls:
- Use the arrows to move
- Use 'g' to pickup items on the floor
- Use [1-9] to use carried items

Combat:
- Walk into an enemy to inflict damage.

Victory condition:
- Explore the dungeon and fight off monsters until you find the Amulet.
