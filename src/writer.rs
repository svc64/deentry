use crate::{DesktopEntryGroup, DesktopEntry};

impl DesktopEntryGroup {
    /// Convert this group (only this group) to a string in the desktop entry format
    pub fn to_string(&self) -> String {
        let mut group_str = String::new();
        for entry in &self.entries {
            if entry.locale.is_some() {
                group_str += format!("{}[{}]={}\n", entry.key, entry.locale.as_ref().unwrap(), entry.value.content).as_str();
            } else {
                group_str += format!("{}={}\n", entry.key, entry.value.content).as_str();
            }
        }
        group_str.trim().to_string()
    }
}

impl DesktopEntry {
    /// Convert this object back to a string in the desktop entry format
    pub fn to_string(&self) -> String {
        let mut entry_str = String::new();
        for group in &self.groups {
            entry_str += format!("[{}]\n", &group.group_name).as_str();
            entry_str += format!("{}\n", group.to_string()).as_str();
        }
        entry_str
    }
}
