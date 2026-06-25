# TODO List

- ???

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

- what happens if hits 0 health? death? start again?
  - maybe need concept of checkpoints at key areas for death. can also reset to checkpoint if not happy.
    - maybe add a hardcore mode that means auto-die and can't reset to checkpoint.

### Combat TODO
- full track of path taken (and choices selected) stored in save file
- choices that have a requirement (i.e. a certain choice made in the past; item in invent;)
- checkpoints on enter areas; store latest checkpoint in save file;
  - menu option to return to last checkpoint
  - hardcore mode to disable return to last checkpoint
- revisitable areas
  - need to store what previous area was before going to revisitable area so can return
