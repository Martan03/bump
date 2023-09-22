# bump

Best Universal Music Player written in Rust and iced  
(not quite the best, but I'm trying my best)

## Configuration

Config files are saved in config folder (on Linux `~/.config/bump`) in file
`config.json`. There are few options you can set and I'll be adding more in the future.  
So far I don't have settings in the app, so this is the only place you can adjust settings.

## GUI

The GUI isn't quite finished, but at least it looks somewhat decent.

![image](https://github.com/Martan03/bump/assets/46300167/e5a48ebf-742c-49cb-b0d6-84756fd9cfbb)

## How to get it?

You have to compile it yourself, but that shouldn't a problem. Only thing you need to have is `cargo`:
```
cargo build -r
```

After it is done compiling, you can then start the binary `./target/release/bump`.

## Library for playing audio

I use library coded by my friend, so thanks!
The library is called [raplay](https://github.com/BonnyAD9/raplay).

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [bump](https://github.com/Martan03/bump)
- **Author website:** [martan03.github.io](https://martan03.github.io)
