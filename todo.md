# TODO List

- keybind settings
- settings menu from pause
- move font assets out of ui/mod.rs into assets mod.

# Nice to haves, not important
- better game over screen
- go back button to return to the previous room (to help with testing)
- export/import save
- main menu splash
- characters with:
  - art_default (some kind of profile pic?)
  - theme_music (plays when they speak?)
  - speech_effect (like... shakes?)

---

### Bugs

- Scrollbar isn't fully pickable on the right-hand side; a few pixels of dead area.


### Do I want combat?

- I want some kind of system to allow alternative paths; This could just be that items are needed, or if you did/didn't do something. This probably means:
  - I need a way to track which rooms the player has been through and what choice they took. Will need to store in save file.
- If that's a thing - narrative, dialogue, choices that impact, do I really need combat at all?
  - What does combat add?
    - An extra dimension to the game; But is that good or bad?
    - More use for items
    - More narrative impact; can get in fights WITH characters
- TLDR; I think I do want to add combat.

### So what's combat about?

- health
- stats?
  - gives choice in build. but stats probably means levels. and levels means xp and idk about that. kinda might encourage murderhobo/grinding.
  - maybe stats is driven purely by items.
- combat items
  - means a UI for equipped stuff next to invent, drag-drop from invent to equip.
  - stats:
    - strength (dmg)
    - constitution (health)
    - but idk, unless there's meaningful choice between items they just seem like stat sticks.
      - probably means i need different styles of combat; magic, ranged, melee. classic rpg.
- healing items (potions, food, etc)
- temp items; strength potion, etc. for boosts in single combat.
- weapon effects: burn, poison, weaken
- active abilities on special items: positive effects like pots, cooldown(?)
- shops (can maybe go to common places whenever; means need a map? or just a list of locations)
  - can discover new ones to gate progression, secret reward shops for good choices

- what does combat actually look like JRPG style? take turns whacking it?
- or auto-battler, real time?

### Time to figure out what exactly combat is
- Simple but strategic and directed by 'style'.
  - e.g. can choose to be a wizard, fighter, ranger
    - each have a primary stat from which to derive power
      - fighter => tanky, medium damage
      - wizard  => squishy, high damage
      - ranger  => middle ground
    - these stats come from items
      - ties theme with power

'Iron longsword'
 + 3 bonk
   0 smarts
 + 1 sneakin'

    - where you can equip an item, 'You attack with your Iron longsword':
      - has a 'type' of bonk, and so only benefits from bonk stat.
    - other equipment can grant stats too. 'Ring of Intellect', +1 smarts.
      - Quite OSRS style
    - passive effects: ring of fire, 25% chance to burn for 1-3 fire on hit

 - different damage types:
  - normal stat line:
    - bonk
    - smarts
    - sneakin'
  - but others too:
    - fire
    - poison
    - lightning, etc
  - resistances to stats
    - 1 resistance to fire reduces all fire damage taken by 1
- all numbers are relatively small:
  - 12 health
  - 3 damage
  - etc

 - the problem is:
  - does this mean combat is just 'attack', 'attack', 'attack'?
  - how can i add in decision making and strategy?
    - swapping items in combat could be good - respond to what works and what doesn't
    - activating items (probably not strategy, probably just a 'do every fight', so not good)
    - abilities?
      - how gain? NOT levels. probably from items too
        - but then, how do they become strategic?
          - combat probably needs to be a bit like rock/paper/scissors
            - but need some core resource in combat to work as a time gate or something
              - like stamina? idk


### Combat TODO
- Invent UI for slots
- The combat log reports damage dealt without taking into account target armour
- player damage_type on attacks is hard-coded to Stab in FSM. Damage is hardcoded to 1.

### Half Finished (not tested/used)
- Items with Effects
  - Bleed, Burn, Poison, etc.
  - Specials, like cleanse status effects
  - Potions, one time use
- DamageType
- DamageResistance(Type)

### Not started
- Areas with combat instead of narration
  - Enemies (1-3)
    - Intro text ("A savage looking bear creeps out of the undergrowth, growling...")
    - Health
    - Armour
    - Abilities
      - Attack
      - Defend
      - Specials
        - (with telegraph info, to warn the player allowing to preempt)
    - Some kind of RNG system to decide what an enemy does
  - UI to show enemies
  - UI to show possible attacks
  - Text log listing out the combat events
- Speed to determine attack order
- Some kind of resource (Stamina?) to constrain how hard the player can go
  - Rewards holding off until a good time
- Armour Penetration, allow punching through high Armour enemies

### Combat TODO - Maybe
- revisitable areas
  - need to store what previous area was before going to revisitable area so can return
- Shops (have inventory, have prices, can buy)
