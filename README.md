# keybindings-parser

[![Rust](https://github.com/milchinskiy/keybindings-parser/actions/workflows/tests.yml/badge.svg)](https://github.com/milchinskiy/keybindings-parser/actions/workflows/tests.yml)

Tiny library for parsing keybindings from a string.
And handle them via keysym & mod mask.

```rust
// init with default configuration
let mut keyb = Keybindings::default();

// add human readable keybinding
keyb.add("command + a", Box::new(TestAction::Kill))?;

// handle keybinging
if let Some(handler) = keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_a) {
    assert_eq!(handler.modifier(), ModMask::MOD4);
    assert_eq!(handler.key(), keys::XK_a);
    assert_eq!(handler.origin(), "command + a");

    // action that you provide  early
    let action = handler.action();
    action.run()?;
}
```

Where `action` object is an object that implements `KeyAction` trait.

```rust
pub trait KeyAction {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
```

In real life keybindings handler will be placed somewhere in `event loop`, e.g.

```rust
// just for example, there are no event::next_event() constructions :)
loop {
    let (mask, key) = event::next_event()?;
    if let Some(handler) = keyb.handle(mask, key) {
        handler.action().run()?;
    }
}
```

## Acceptable Modificator keys

`MOD4` known as `super` key can be described like:

- super
- mod4
- win
- windows
- cmd
- command

`MOD1` known as `alt` key can be described like:

- alt
- mod1
- meta
- alt_l
- alt_r
- meta_l
- meta_r

`MOD3` known as `AltGr` (_does not exist on modern keyboards_)
key can be described like:

- alt_gr
- altgr
- mod3
- meta_gr
- metagr

`CONTROL` key can be described like:

- ctrl
- control
- ctrl_l
- ctrl_r

`SHIFT` key can be described like:

- shift
- shift_l
- shift_r

All modificator keys are described in `ModMask` enum.
All modificator keys are `u32` value.
All modificator keys are `bit flags`.
All modificator keys are case insensitive.
Modificators bitmasks are compatible with X11 `KeyPress` Event value `detail()`.

## Main key

Main key of a keybinding. For example, `mod4 + a`, where `a` is the main key.
Accepts all cleared names from `keysymdefs::keys`.
For example:

- `keys::XK_a` is just `a`
- `keys::XF86XK_MonBrightnessUp` is just `MonBrightnessUp`
- etc.

You free to use full canonical name of keys, like `XK_a`, `XF86XK_MonBrightnessUp`, etc.

Unlike modificator keys, main key is not case insensitive. In other words,
`mod4 + a` is *NOT* the same as `mod4 + A`. Moreover `mod4 + A` does not make
much sense in context of a keybinding. Because `A` is already has modificator
`SHIFT`. Instead of this `mod4 + A`, more correct way is handling `mod4 + shift + a`.
Nevertheless, library will accept this as valid and you should handle such cases
yourself, depending on your goals.

## Change defaults

```rust
let delimiter_char = '/';
let ghost_modifiers = ModMask::MOD4;

let keyb = Keybindings::new(delimiter_char, ghost_modifiers);

keyb.add("command / a", Box::new(TestAction::Kill))?;

// ...
// and then handle
let handler = keyb.handle(ModMask::MOD4, keysymdefs::keys::XK_a)?;
handler.run()?;
```

### Custom KeyAction implementation, used in examples

```rust
enum TestAction {
    Kill,
    ToggleFloat,
    ToggleFullscreen,
    ToggleTabBar,
    SwapWindows,
    Spawn(String),
}

impl KeyAction for TestAction {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TestAction::Kill => {}
            TestAction::Spawn(_) => {}
            TestAction::SwapWindows => {}
            TestAction::ToggleFloat => {}
            TestAction::ToggleFullscreen => {}
            TestAction::ToggleTabBar => {}
        };
        Ok(())
    }
}
```
You are free to use as many `KeyAction` implementations as you need for
your purposes. The above example is for demonstration purposes only.

## Ghost modifiers

There are keyboard switches that should not affect behavior
of keybindings in a general purposes.
Among them:

- `CAPS_LOCK`
- `MOD2` better known as `NumLock`
- `MOD5` better known as `ScrollLock`

All of them are ignored by default in `Keybindings::default()`.
If you want to change these preset options, use `new()`
instead of `default()`, e.g.:

```rust
let key_delimiter = '/';
let ghost_modifiers = ModMask::MOD4 | ModMask::MOD5;

let keyb = Keybindings::new(key_delimiter, ghost_modifiers);
```

## Limitations and cautions

- There are no limitations on the number of keybindings. You can add as many
  as you want.
- There are no limitations on the number of actions. You can add as many
  as you want.
- Keyboard shortcuts may not contain modifiers at all (e.g. `Print` key).
- But the keyboard shortcuts can't be without the main key (e.g. `"super"`)
- Modificator keys are case insensitive, main key is not
- Сomplete duplication of keyboard shortcuts will cause an error when adding
- Although, it is possible to add keybindings with the same main key and
  different modificator keys
- Shortcuts like `super + XK_A` or `super + A` are supported but does not
  make much sense, most likely you need `super + shift + a` or
`super + shift + XK_a` respectively
- There is no support for key combinations like `mod4 + a + b`, there are
should be single main key

## where should I get keysym?

short summary of a long story about keysym: _You must exchange the keycode from the keypress event with a keysym, which can be obtained from the device details and information about the keyboard key mapping on a specific device. There is no static keycode to keysym mapping table._
