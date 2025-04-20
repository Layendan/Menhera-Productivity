# TODO List

Core functionality

-   [x] Screen sharing
-   [ ] Menhera character/icon/emotes on desktop
    -   [x] Figure out framework/tools (language) ==> we are using Tauri + SvelteKit (TypeScript frontend, Rust backend)
    -   [ ] States of Menhera (e.g. focused, distracted, very distracted, etc.)
-   [ ] Data collection
    -   [ ] Labeled data = screenshots with either "distracted" or "focused"
    -   [ ] "distracted" activity is like YouTube, games, etc.
    -   [ ] "focused" = lots of words, studying, writing essays, reading, etc.
-   [ ] Train AI/ML classifier model to determine swapping of states
    -   [ ] Train in Python, connect in Rust
    -   [ ] Input: screenshots of screen (TBD)
    -   [ ] Output: state (e.g. focused vs. distracted)
        -   i think probably best to keep the ML model output simple;
        -   we can have the frontend handle the state of Menhera (e.g. what image she shows) such as a timer
        -   e.g. distracted for 5+ seconds, then distracted level 1 --> 10+ seconds, distracted level 2, etc.
        -   focused for 30+ seconds, encouragement image level 1 --> 60+ seconds, encouragement level 2, etc.

Side functionality (small things to do if we have extra time)

-   [ ] Motivational messages from Menhera
-   [ ] Audio involvement? (perhaps separate ML classifier, but also this probably is not really necessary)
