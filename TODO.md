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
    - [ ] make a pause menu
        - [ ] has score
        - [ ] has conitue button
        - [ ] has rage quite button
        - [ ] has view/edit controls button
    - [ ] show pause menu when paused
    - [ ] pause player actions, computer actions, & computer timer when paused
- [ ] add screens (each screen as a Bevy pluggin)
    - [ ] welcome screen (`Play`, `Controls`, or `How To Play`)
    - [ ] mode select screen (`vs. computer`, `vs. human (LAN)`, or `spectate`)
    - [ ] controls screen (to show/edit controls)
    - [ ] "how to play" screen (shows how to play the game)
    - [ ] touch scored screen (announce the scoring of a touch)
        - the user should be presented with:
            - the new score
            - a button to go to next bout
            - a button to rage quit
    - [ ] match victory screen (announce that a player won the match)

## Notes

- all buttons should be both clickable w/ the mouse & selectable with the keyboard

