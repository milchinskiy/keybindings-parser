mod masks;
pub use masks::ModMask;

#[derive(Debug)]
pub struct Keybindings {
    binds: Vec<Keybinding>,
    ghost_modifiers: ModMask,
    key_delimiter: char,
}

impl Keybindings {
    pub fn new(key_delimiter: char, ghost_modifiers: ModMask) -> Self {
        Self {
            binds: Vec::new(),
            ghost_modifiers,
            key_delimiter,
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            binds: Vec::new(),
            ghost_modifiers: ModMask::MOD2 | ModMask::MOD5 | ModMask::CAPS_LOCK,
            key_delimiter: '+',
        }
    }
}

pub struct Keybinding {
    origin: String,
    modifier: ModMask,
    key: u32, // keysym
    action: Box<dyn KeyAction>,
}

impl Keybinding {
    pub fn modifier(&self) -> ModMask {
        self.modifier
    }
    pub fn key(&self) -> u32 {
        self.key
    }
    pub fn action(&self) -> &dyn KeyAction {
        self.action.as_ref()
    }
    pub fn origin(&self) -> &str {
        &self.origin
    }
}

impl std::fmt::Debug for Keybinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keybinding")
            .field("modifier", &self.modifier)
            .field("key", &self.key)
            .finish()
    }
}

impl Keybindings {
    pub fn add(
        &mut self,
        human_key_string: &str,
        action: Box<dyn KeyAction>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.binds.iter().any(|x| x.origin == human_key_string) {
            return Err("duplicate keybinding")?;
        }
        let mut parts: Vec<&str> = human_key_string
            .split(self.key_delimiter)
            .map(|x| x.trim())
            .collect();
        let mut keymod = ModMask::NONE;

        let key = match parts.pop() {
            Some(k) => k.to_string(),
            None => return Err("no key provided")?,
        };

        let (strict, lazy) = (
            keysymdefs::get_item_by_name(&key),
            keysymdefs::get_item_by_cleared_name(&key),
        );

        let mut key = 0u32;
        if let Some(lazy) = lazy {
            key = lazy.keysym();
        }
        if let Some(strict) = strict {
            key = strict.keysym();
        }
        if key == 0 {
            return Err("Key not found".to_string())?;
        }

        while let Some(m) = parts.pop() {
            keymod |= match m.to_lowercase().as_str() {
                "super" | "mod4" | "win" | "windows" | "cmd" | "command" => ModMask::MOD4,
                "alt" | "mod1" | "meta" | "alt_l" | "alt_r" | "meta_l" | "meta_r" => ModMask::MOD1,
                "alt_gr" | "mod3" | "altgr" | "meta_gr" | "metagr" => ModMask::MOD3,
                "ctrl" | "control" | "ctrl_l" | "ctrl_r" => ModMask::CONTROL,
                "shift" | "shift_l" | "shift_r" => ModMask::SHIFT,
                _ => return Err("invalid modifier")?,
            }
        }

        self.binds.push(Keybinding {
            origin: human_key_string.to_string(),
            modifier: keymod,
            key,
            action,
        });

        Ok(())
    }

    pub fn handle(&self, modifiers: ModMask, keysym: u32) -> Option<&Keybinding> {
        let find = self.binds.iter().find(|k| {
            (k.modifier | self.ghost_modifiers) == (modifiers | self.ghost_modifiers)
                && k.key == keysym
        })?;
        Some(find)
    }
}

pub trait KeyAction {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
