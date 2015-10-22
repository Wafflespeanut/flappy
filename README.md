## Free fall

A horribly *inefficient* game to fiddle around Rust and its interaction with the terminal. There's a jumper who's got one DOF (to move sideways) and avoid the cliffs from hitting him (when the game ends). This game was inspired by the [tetris game written in Rust](https://www.reddit.com/r/rust/comments/1yr2uz/tetris_game_in_rust/).

### Set your terminal attributes!

You need to find the system-dependent constant (TIOCGWINSZ) for your terminal and set it [here](https://github.com/Wafflespeanut/free-fall/blob/master/src/helpers.rs#L6). Since most of the unix-based OS have Python, you can do something like this...

``` python
import termios
print termios.TIOCGWINSZ
```

### Reduce the font size!

Before running, please resize the terminal window and font size (and of course, the width and height constants) to fit the game in your default terminal (which I can do in my Ubuntu). Well, this game works best in [xterm](https://en.wikipedia.org/wiki/Xterm). If you have it, then you can do something like this...

``` bash
xterm -maximized -fa 'Monospace' -fs 12
```
