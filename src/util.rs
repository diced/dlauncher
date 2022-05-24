use std::path::Path;

use gtk::{
  gdk::SELECTION_CLIPBOARD,
  glib::{spawn_async, SpawnFlags},
  Clipboard,
};
use libc::setsid;

use crate::{
  entry::app_entry::AppEntry,
  fuzzy::{get_matching_blocks, get_score, MatchingBlocks},
  script::Script,
};

pub fn no_match() -> MatchingBlocks {
  (vec![], 0)
}

/// Copy `text` to the clipboard.
/// Requires gtk::set_initialized() to be called first if inside an extension.
pub fn copy_to_clipboard(text: &str) {
  let clipboard = Clipboard::get(&SELECTION_CLIPBOARD);
  clipboard.set_text(text);
  clipboard.store();
}

/// Checks if `text` matches `comparison` using fuzzy search. The must_be parameter is used to
/// determine what the least text of the match should be.
pub fn matches(query: &str, text: &str, min_score: usize) -> Option<MatchingBlocks> {
  let score = get_score(query, text);

  if score >= min_score {
    Some(get_matching_blocks(query, text))
  } else {
    None
  }
}

/// Checks if a user's query matches an apps description, name, and executable file.
pub fn matches_app(
  app: &AppEntry,
  query: &str,
  min_score: usize,
) -> Option<(MatchingBlocks, usize)> {
  let app_score = get_score(query, &app.name);
  let score = vec![
    app_score as f64,
    get_score(query, &shell_words::join(&app.exec)) as f64 * 0.8,
    get_score(query, &app.description) as f64 * 0.7,
  ]
  .into_iter()
  .map(|x| x as usize)
  .max()
  .unwrap();

  if score >= min_score {
    Some((get_matching_blocks(query, &app.name), app_score))
  } else {
    None
  }
}

/// Checks if a user's query matches a scripts description and name.
pub fn matches_script(
  script: &Script,
  query: &str,
  min_score: usize,
) -> Option<(MatchingBlocks, usize)> {
  let script_score = get_score(query, &script.meta.name);
  let score = vec![
    script_score as f64,
    get_score(query, &script.meta.desc) as f64 * 0.7,
  ]
  .into_iter()
  .map(|x| x as usize)
  .max()
  .unwrap();

  if score >= min_score {
    Some((get_matching_blocks(query, &script.meta.name), script_score))
  } else {
    None
  }
}

/// Initialize a logger, used for extensions.
///
/// # Example
/// ```rust
/// use dlauncher::extension::ExtensionContext;
/// use dlauncher::util::init_logger;
/// use log::debug;
///
/// #[no_mangle]
/// pub unsafe extern "C" fn on_init(ctx: ExtensionContext) {
///   init_logger();
///   debug!("Extension initialized"); // if init_logger was not called then nothing would be printed.
/// }
/// ```
pub fn init_logger() {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "info");
  }

  env_logger::init();
}

/// Launch a executable that is detached from the main dlauncher program, achieved through
/// `libc::setsid`. Meaning that once dlauncher closes, the executable will continue to run.
/// Due to the way glib works, it asks for Vec<&Path>. You can easily convert a vector of strings to
/// a vector of &Paths.
///
/// The second argument is additional environment variables that you want to pass to the executable.
/// By default launch_detached uses the environment variables passed to itself. Paths inside of this
/// vector should be formatted as a traditional environment variable `TEST=hello` =
/// `&Path::new("TEST=test")`, it looks a bit weird, but glib requires a Path.
///
/// # Example
/// An extension that launches dlauncher again when its initialized (I don't know why you would do
/// this).
/// ```rust
/// use dlauncher::util::launch_detached;
///
/// #[no_mangle]
/// pub unsafe extern "C" fn on_init() {
///   launch_detached(vec![
///     "/usr/bin/dlauncher"
///   ], vec![]);
///
///   // dlauncher wouldn't work as run as DISPLAY is not set (example of extra environment variables)
///   launch_detached(vec![
///     "/usr/bin/dlauncher"
///   ], vec![
///     "DISPLAY=idk"
///   ]);
/// }
/// ```
///
pub fn launch_detached<T: Into<String>>(spawn_args: Vec<T>, spawn_env_extra: Vec<T>) {
  // The things I do for convenience...
  let spawn_args = spawn_args
    .into_iter()
    .map(|x| x.into())
    .collect::<Vec<String>>();
  let spawn_args = spawn_args
    .iter()
    .map(|x| &**x)
    .map(Path::new)
    .collect::<Vec<&Path>>();

  let spawn_env_extra = spawn_env_extra
    .into_iter()
    .map(|x| x.into())
    .collect::<Vec<String>>();
  let spawn_env_extra = spawn_env_extra
    .iter()
    .map(|x| &**x)
    .map(Path::new)
    .collect::<Vec<&Path>>();

  let args = std::env::vars();
  let mut formatted = Vec::new();

  for (k, v) in args {
    if k != "GDK_BACKEND" {
      formatted.push(format!("{}={}", k, v));
    }
  }

  let mut spawn_env: Vec<&Path> = formatted.iter().map(Path::new).collect();
  spawn_env.extend(spawn_env_extra.iter().map(Path::new));

  spawn_async(
    None as Option<&Path>,
    spawn_args.as_slice(),
    spawn_env.as_slice(),
    SpawnFlags::SEARCH_PATH_FROM_ENVP | SpawnFlags::SEARCH_PATH,
    Some(Box::new(|| unsafe {
      setsid();
    })),
  )
  .unwrap();
}

/// Open something using xdg-open.
///
/// # Example
/// ```rust
/// xdg_open(vec!["file:///home/user/"]) // this will open a file manager usually
/// ```
pub fn xdg_open(args: Vec<&str>, spawn_env_extra: Vec<&str>) {
  let mut spawn_args = vec!["xdg-open"];
  spawn_args.extend(args);

  launch_detached(spawn_args, spawn_env_extra);
}
