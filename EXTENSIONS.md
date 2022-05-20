# Extensions
Extensions allow developers to add new functionality to Dlauncher, for example an extension that lets users search for symbols and copy them to their clipboard.

# Getting Started
Extensions in Dlauncher are made possible through FFI and shared object libraries (.so files) requiring us to write a lot of unsafe code.

First setup a new cargo project:
```shell
cargo new dlauncher_extension
cd dlauncher_extension
```

Then add the following to the `Cargo.toml` file:
```toml
[dependencies]
dlauncher = "0.1.0"
gtk = { version =  "0.15.5", features = ["v3_22"] }
lazy_static = "1.4.0"
log = "0.4.17"
```

Then add the following to the `src/lib.rs` file:
```rust
use dlauncher::{
  extension::{
    ExtensionContext,
    response::{ExtensionResponse, ExtensionResponseIcon},
  },
};
use dlauncher::util::init_logger;
use lazy_static::lazy_static;
use log::debug;

#[no_mangle]
pub unsafe extern "C" fn on_init(ctx: ExtensionContext) {
  init_logger();
  
  info!("Hello from extension!")
}
```

The `#[no_mangle]` attribute is required to prevent the compiler from mangling the function name so that it can be
called via `on_init` in dlauncher.
The `on_init` function is called when the extension is loaded, this is not required but is useful when the extension
needs to read data before it is used.
We are using the `log` crate to add support for logging, and the `init_logger()` function is used to initialize
the logger for use.
The `ctx` variable is an [`ExtensionContext`](ExtensionContext) struct containing things that let you interface with
the main dlauncher process and window.

## Enabling your extension
To test out and debug your extension the easiest way as of now is copy the built .so file to the `extensions` folder.
Before doing this you need to add the file name to the `extensions` array in the `dlauncher.toml` file.

```toml
extensions = ["dlauncher_extension.so"]
```

Then we can build the extension, copy it over, then run dlauncher.
```shell
cargo build --release
cp target/release/libdlauncher_extension.so ~/.config/dlauncher/extensions

# kill an already running dlauncher
pkill dlauncher

# this is assuming that the dlauncher bin is inside your PATH or usually /usr/bin
dlauncher
```
This can be shortenned into a one liner:
```shell
cargo build --release && cp target/release/libdlauncher_extension.so ~/.config/dlauncher/extensions && pkill dlauncher && dlauncher
```

## Listening to input events
To listen to when a user types in the input field we have to add the `on_input` function to the `src/lib.rs`.

```rust
use dlauncher::{
  extension::{
    ExtensionContext,
    response::{ExtensionResponse, ExtensionResponseIcon},
  },
};
use dlauncher::util::init_logger;
use lazy_static::lazy_static;
use log::debug;

#[no_mangle]
pub unsafe extern "C" fn on_init(ctx: ExtensionContext) {
  init_logger();
  
  info!("Hello from extension!")
}

#[no_mangle]
pub unsafe extern "C" fn on_input(ctx: ExtensionContext) {
  gtk::set_initialized();
  
  info!("input: {:#?}", ctx.input);
}
```

`gtk::set_initialized()` is used to make sure that the gtk library is sure that we initialized it. This is not needed
unless you are interfacing with GTK.

## Checking for a match
If we want to see if the users input matches something we can use the [`matches`](../util/fn.matches.html) function.
Heres a function that checks if the users input matches "zero width space". We also make sure that the input is not
[None](None) before we check it. The third argument passed in [`matches`](../util/fn.matches.html) is the least score
required for a match, a safe value for this is usually 60-80, if you want more precision for your match you can use a
higher value like 100-150, just remember that the input has to be very specific and may not yield the best results. Now
if we type in the input field something like "ze" we should see in our logs "matched: true".
```rust
#[no_mangle]
pub unsafe extern "C" fn on_input(ctx: ExtensionContext) {
  gtk::set_initialized();
  
  if let Some(input) = ctx.input {
    let matched = matches(input, "zero width space", 60);
    info!("matched: {}", matched);
  }
}
```

## Adding a result entry
Usually once the match is found we can add a result entry that will show up in the UI. This can be made easy by the
[`ExtensionResponse`](response/struct.ExtensionResponse.html) struct. This will allow you to make a "builder" that will allow you to easily
create a result entry with lines.

```rust
#[no_mangle]
pub unsafe extern "C" fn on_input(ctx: ExtensionContext) {
  gtk::set_initialized();
  
  if let Some(input) = ctx.input {
    let matched = matches(input, "zero width space", 60);
    if !matched {
      return;
    }
    
    let mut response = ExtensionResponse::builder(&ctx.name, None);
    response.line(
      "Zero Width Space",
      "Press enter to copy to your clipboard",
      ExtensionResponseIcon::themed("spacer-symbolic")
    );
  }
}
```
Here, the line function takes 3 arguments: name, description and icon. The [`ExtensionResponseIcon`](response/struct.ExtensionResponseIcon.html)
enum is used to specify the icon that will be used for the result entry. The `themed` function is used to specify a
themed icon, for example using the Papirus icon theme. The `svg` function is used to specify a svg string, which is useful
when you want to use a custom icons, or dynamic icons that can be made on the fly.

Now when we type in "ze" we should see the result entry showing Zero Width Space with the icon and everything. but when
we press enter the character doesn't get copied to the clipboard.

## Controlling what happens when a line is clicked/entered
The [`ExtensionResponse`](response/struct.ExtensionResponse.html) struct has a couple more functions that let you add actions that happen
when the user clicks or presses enter on the line. `ExtensionResponse::line_on_enter` adds a 4th argument that will take
a function.
```rust
#[no_mangle]
pub unsafe extern "C" fn on_input(ctx: ExtensionContext) {
  gtk::set_initialized();
  
  if let Some(input) = ctx.input {
    let matched = matches(input, "zero width space", 60);
    if !matched {
      return;
    }
    
    let mut response = ExtensionResponse::builder(&ctx.name, None);
    response.line_on_enter(
      "Zero Width Space",
      "Press enter to copy to your clipboard",
      ExtensionResponseIcon::themed("spacer-symbolic"),
      |ctx| {
        info!("hello! i was clicked");
      }
    );
  }
}
```
Now when we press enter on the line we should see the print in the logs. Inside the closure we can now use the utility
function [`copy_to_clipboard`](../util/fn.copy_to_clipboard.html) to copy the text to the clipboard.
```rust
response.line_on_enter(
  "Zero Width Space",
  "Press enter to copy to your clipboard",
  ExtensionResponseIcon::themed("spacer-symbolic"),
  |ctx| {
    copy_to_clipboard("\u{200B}");
  }
);
```

# States via static variables
Some extensions might require a prefix, like `sym equal` meaning that `sym` is the prefix and `equal` are the arguments
(This example is refering to an extension that lets you look up symbols and copy them to your clipboard). An inefficient
way of getting the user's prefix would be `ctx.config.get("prefix").unwrap_or("sym")` every time inside your `on_input`
function. To get around this we can use a static variable, and set the value during the `on_init` function.

To make sure that the value is thread safe and can be mutated we use a [Mutex](Mutex) wrapped in an [Arc](Arc).
We also use [lazy_static](https://crates.io/crates/lazy_static) as normally we can't call functions in static variables.

We add the following at the top of the file:
```rust
lazy_static! {
  #[derive(Debug)] static ref PREFIX: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}
```

Then inside of our `on_init` function we set the value of the static variable:
```rust
#[no_mangle]
pub unsafe extern "C" fn on_init(ctx: ExtensionContext) {
  init_logger();

  let mut prefix = PREFIX.lock().unwrap();
  *prefix = ctx
    .config
    .get("prefix")
    .unwrap_or_else(|| "sym".to_string())
    + " ";
}
```
The following gets a lock of the prefix value, then sets the value of it to the `prefix` key in the extension config,
and if it doesn't exist it sets it to `sym`.

In our `on_input` function we can now use the static variable to get the prefix:
```rust
#[no_mangle]
pub unsafe extern "C" fn on_input(ctx: ExtensionContext) {
  gtk::set_initialized();

  if let Some(input) = ctx.input {
    let prefix = &*PREFIX.lock().unwrap();

    if input.is_empty() || !input.to_lowercase().starts_with(prefix) {
      return;
    }
    
    // do stuff now.
  }
} 
```

That's it! Feel free to explore the API, as extensions let you have full control over everything that happens. 