# TODO List

- text draw sound-effect?
- texture atlases for UI frames: https://bevy.org/examples/ui-user-interface/ui-texture-slice/

# Nice to haves, not important
- go back button to return to the previous room (to help with testing)
- pause menu settings
- export/import save
- main menu splash
- hover effect on inventory icon, bg colour, something for inteaction

---

- define characters.ron:
  - display_name
  - art_default
  - theme_music?
  - speech_effect?
  - text_color?
  - text_font?

---

### Bugs

- Scrollbar isn't fully pickable on the right-hand side; a few pixels of dead area.
- Tooltip content doesn't seem to fade in like the tooltip container does
- tooltips targeting an entity that gets despawned remains visible until a new tooltip is hovered
