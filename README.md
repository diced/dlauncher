# Dlauncher (A Rust Ulauncher port)
Basically a one to one copy of Ulauncher, with a different backend.

# Installing
As of now Dlauncher is not available on any distributions. You can build it though!
## Building
```shell
git clone https://github.com/diced/dlauncher
make build
```
The executables are located in the `target/release` folder.

## Running
If you are using xinit, you can add `dlauncher &` to it.

## Toggling the window
If you are running in daemon mode, you can run the `dlauncher-toggle` command to toggle the window from appearing.

# Migrating from Ulauncher
Due to how Dlauncher is built, it is 100% compatible with Ulauncher themes! All you need to do is move your theme from `~/.config/ulauncher/user-themes`
to `~/.config/dlauncher/themes`. 
```shell
cp -r ~/.config/ulauncher/user-themes/* ~/.config/dlauncher/themes`
```

If you would like to migrate your recent apps that show at the start, you can 

# Why?
I love the way Ulauncher works and looks, but I had some issues: Ulauncher's preferences are controlled through a Webkit frontend, which can consume a lot of memory. This coupled with the fact its using python also contributes to the fact that it uses a lot of memory (it isnt really that much its like a few hundred mb's but can total up to 500 sometimes with "ghost" webkit processes, etc).

Many other factors such as extensions can also add tons of memory to the process, dlauncher even with ports of the extensions I use, runs at the same 40-60 MB that the base binary runs at.

# How? & Motivation
Since Ulauncher is made using GTK libraries, it made it easy to create a clone of Ulauncher since I could reuse the design files for Ulauncher (which is what I did). After doing so, I could use the code from Ulaunchers codebase, to figure out how it would translate the best in Rust, which was also made very easy due to how similar the GTK and GDK implementations are.

Once I was able to replicate the same UI that Ulauncher had I started to focus more on the backend, and how extensions would work

# Extensions?
At the start, extensions would have been a command ran by Dlauncher then its stdout would get parsed yet that did not work well and had absolutely zero functionality. I was able to figure out how to use FFI and shared object libraries (`.so` files) which could be loaded in at runtime using `libloading`. This effectively made extensions much more robust and flexible as to what the developer wanted to do. As of now the API is kind of limited, yet it is possible to do whatever you would like to do.

# What Changed?

## Backend
* The backend is now written in Rust, allowing the launcher to use less resources.
* The way search works might still act different from the original Ulauncher. I found a library called [fuzzywuzzy-rs](https://github.com/logannc/fuzzywuzzy-rs) which had a method called `get_matching_blocks` which I have just decided to copy over to here since I did not want to import this as a dependency/use anything from it except that.
* Recents are stored in a file called `dlauncher.druncache`
* The configuration is entirely based in a file instead of being managed through a UI. (I might add an external program that manages the file, so it doesn't interfere with the main process)
* Extensions (basically entirely different lol)

Dlauncher runs consistently at around 40-60 MB compared to almost the 200-400 MB that Ulauncher uses (sometimes extensions can make this go up even more).

## Frontend
Nothing! Your Ulauncher themes will work perfectly with Dlauncher.

# Future
I plan to keep working on making Dlauncher more performant! The code also is kinda garbage, any help is appreciated!

# Copyright Notice
Modifications done to Ulauncher's original source
* data/result.ui
* data/DlauncherWindow.ui
* data/themes/light