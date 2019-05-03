[X] Render dungeon

[X] Move player on cursor left/right/down

[ ] Show X lines of dungeon, keeping player in the center

Prevent player from moving over walls

Prevent player from moving out of bounds of dungeon







Generate the dungeon
```
        
########@##########
###.......#########
######.......######
###....####..######
######.####......##
######..####.######
####....###..######
####....###..######
#....D............#
```


Go to next dungeon when door (D) is reached

Manipulate dungeon generator by hacking terminals

Constrain movement and penalties

Fight guards/dev team

Pickup items





# Dungeon Generator in the game

- Terminals are scattered throughout the dungeon
- Can be hacked to change certain properties (based on D20 + hacking ability)
    - Guard strength
    - Item quality
    - Bigger holes in walls (less horizontal movement needed)
- Trapped project managers can be freed, they will run off and slow down development for subsequent levels
- QA managers can be killed, keeping bugs in the generator
    - A guard could randomly turn into an item
    - An item could randomly turn into a guard
    - a G could really be an item and vice versa (rendering glitch)
    - a wall tile could really be a hole, and vice versa
- QA maangers could also detect hacks
- Or maybe security consultants, so they can be killed too
- All "monsters" are really the dev team; killing/freeing them has influence on the level generator
- It's also about preventing new "Features" like .. traps, monsters, .... so not only manipulate current foes, prevent new foes for a while



# Example dungeon
```
        @
########.##########
###.......#########
######.......######
###.$..####GG######
######.####..T...##
######..####.######
####....###..######
####....###..######
#....D............#
```

# Gameplay/Brainstorm

- Hero can go left and right certain amount of steps
- If horizontal steps are exhausted, lose health with every other step
- Can only go down through holes/empty tiles (dots)
- Door (D) -> to the next floor
- Empty tiles can contain things: 
    - Coin ($)
    - Guards -> they are blocking, must be fought/killed
    - Treasure (T) -> typically worth a lot, often guarded
    - Items (I) -> to upgrade attributes for fighting
    - Initial combat system:
        if (Attack > Defense) then Health = Health - Damage
        From: https://www.gamedev.net/forums/topic/665820-roguelike-combat-system/

- Goal is to scroll all the way down    

- Player can influence how the next dungeon is generated
    - Number of guards
    - Value of treasure
    - Strength of guards
    - Destroy certain abilities to generate the above things
    - Introduce bugs/hacks to generate more gold, etc

- Player does have attributes like strength, etc, but also has ways to influence dungeon generation
    - Hacking something something
    - Destroy a power generator (reduce cpu cycles)
    - Unleash a project manager (slow down development)
    - Unleash MORE project managers
    - Kill QA personell
    - Destroy bug reports
    - Close doors to reduce cooling:  We are in a data center that defends itself .. but also generates its next levels. Evil AI and stuff. Or evil programmers/programming team/game dev team
    - Can also strive to enhance next dungeon, to have more of a challenge, and greater reward

- Coins/Loot can be used to upgrade at end of level 

- Or does have strength, etc, but in addition can level abilities that make game easier/different/harder?



