use device_query::Keycode;
use std::fmt;
use std::str::FromStr;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct HotKey {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub win: bool,
    pub key: Keycode,
}

impl HotKey {
    pub fn pressed(self, keys: &Vec<Keycode>) -> bool {
        if self.alt {
            if !keys.contains(&Keycode::LAlt) && !keys.contains(&Keycode::RAlt) {
                return false
            }
        }
        if self.ctrl {
            if !keys.contains(&Keycode::LControl) && !keys.contains(&Keycode::RControl) {
                return false
            }
        }
        if self.shift {
            if !keys.contains(&Keycode::LShift) && !keys.contains(&Keycode::RShift) {
                return false
            }
        }
        if self.win {
            if !keys.contains(&Keycode::LMeta) && !keys.contains(&Keycode::RMeta) {
                return false
            }
        }
        if keys.contains(&self.key) {
            return true
        }
        false
    }
}

impl FromStr for HotKey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let alt = s.contains('!');
        let ctrl = s.contains('^');
        let shift = s.contains('+');
        let win = s.contains('#');
        let cleaned_str: String = s
            .chars()
            .filter(|&c| !"+#!^".contains(c))  // Keep characters that are not +#!^
            .collect();
        match Keycode::from_str(&cleaned_str) {
            Ok(key) => {
                Ok(HotKey {
                    alt, ctrl, shift, win, key
                })
            },
            Err(_) => Err(String::from("KeyCode invalid!")),
        }
        
    }

    
}

impl fmt::Display for HotKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{:?}",
            if self.alt { "!" } else { "" },
            if self.ctrl { "^" } else { "" },
            if self.shift { "+" } else { "" },
            if self.win { "#" } else { "" },
            self.key
        )
    }
}
