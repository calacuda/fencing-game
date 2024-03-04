# TODO

- [x] refactor into pluggins to make my life easier
    - [x] startup
        if it runs at start up, it goes here.
    - [x] player
        if it involve spawning, moving, or controlling the player character, it goes here.
    - [x] combat
        if it involves combat logic (scoring, detecting a win, etc) it goes here.
    - [x] computer AI player (called: ai)
        if it involves the computer contrtolled fighter AI, it goes here.
- [x] enable parrying
- [x] enable ai player
- [x] rework parrying after ai player can fight back
- [x] add bounds detection
- [x] stop fighters from flipping sides. (detect side flip and resset match if so.)
- [x] fix bounds detection
- [x] only set Fighter.position and make another system to set sprite location based on that
- [x] add icon to represent the player's and the opponent's gaurd
- [x] add score board
- [x] make parrying, gaurd dependant
- [ ] make gaurd icon only apear breafly when the player changes gaurds and at the beginning of the match
- [ ] implement a beat (parry with out a lunge to steal right of way)
- [ ] add a q-learning agent to control the computer player
- [ ] add multiplayer LAN games
- [ ] add pausing for `vs. comp` games
    - [ ] make a pause menu,
    - [ ] show pause menu when paused
    - [ ] pause player actions, computer actions, & computer timer when paused
- [ ] add screens
    - [ ] welcome screen (`Play` or `Show Controls`)
    - [ ] mode select screen (`vs. computer` or `vs. human (LAN)`)
    - [ ] controls screen (to show controls)
