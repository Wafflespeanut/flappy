## Free fall

Well, this is a game developed entirely for my own learning purposes (I wanted to fiddle around Rust and its interaction with the terminal). The game's based on a jumper who's got one DOF to move sideways to avoid the cliffs from hitting him (when the game ends).

This game was inspired by the [tetris game written in Rust](https://www.reddit.com/r/rust/comments/1yr2uz/tetris_game_in_rust/). I don't like *tetris* very much, but I was really amazed by the author's ideas (especially because it was written when Rust was at its infancy).

### Set your terminal attributes!

You need to find the system-dependent constant (TIOCGWINSZ) for your terminal and set it [here](https://github.com/Wafflespeanut/free-fall/blob/master/src/main.rs#L16). Since most of the unix-based OS have Python, you can do something like this...

``` python
import termios
print termios.TIOCGWINSZ
```

### Reduce the font size!

Before running, please resize the terminal window and font size (and of course, the width and height) to fit the game in your default terminal (which I can do in my Ubuntu). If you have [xterm](https://en.wikipedia.org/wiki/Xterm), then you can do something like this...

``` bash
xterm -maximized -fa 'Monospace' -fs 10
```
