## free fall

Well, this is a terminal-based game developed entirely for my own learning purposes (I wanted to fiddle around Rust and its interaction with the terminal). The game's based on a jumper who's got one DOF to move sideways to avoid the cliffs from hitting him (when the game ends).

![screenshot](https://raw.githubusercontent.com/Wafflespeanut/free-fall/master/screenshot.png)

Though this game's all setup and works pretty well, **it's a failure!** *(unfortunately)*. Because, I was involved in this only during my free time (coffee-breaks & power outages, for example), and I did have a lot of *fun times* along the way. But now, this game's at a stage where it demands a good AI for obstacle-generation and score-keeping, which is something that requires more time and commitment (which I cannot do), and so I've decided to hibernate this for a while.

This game was inspired by the [tetris game written in Rust](https://www.reddit.com/r/rust/comments/1yr2uz/tetris_game_in_rust/). I don't like *tetris* very much, but I was really amazed by the author's ideas, which has greatly helped me with developing this game.

## Usage

If the game doesn't work in its default configuration, then try resizing the terminal window and font size (and of course, the width and height) to fit the game in your default terminal (which I can do in my Ubuntu). If you have [xterm](https://en.wikipedia.org/wiki/Xterm), then you can do something like this...

``` bash
xterm -maximized -fa 'Monospace' -fs 10
```
