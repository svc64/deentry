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
            entry_str += format!("{}\n\n", group.to_string()).as_str();
        }
        entry_str
    }
}


#[cfg(test)]
mod tests {
    use crate::{DesktopEntry, DesktopEntryGroup, DesktopEntryGroupEntry, EntryValue};

    #[test]
    fn desktop_entry_writing() {
        let entry = DesktopEntry {
            groups: [
                DesktopEntryGroup {
                    group_name: "Asaf's favorite food".to_string(),
                    entries: [
                        DesktopEntryGroupEntry {
                            locale: Some("en_US".to_string()),
                            key: "food".to_string(),
                            value: EntryValue {
                                content: "I love pizza".to_string(),
                            }
                        },
                        DesktopEntryGroupEntry {
                            locale: Some("he_IL".to_string()),
                            key: "food".to_string(),
                            value: EntryValue {
                                content: "אני אוהב פיצה".to_string(),
                            }
                        },
                        DesktopEntryGroupEntry {
                            locale: None,
                            key: "food".to_string(),
                            value: EntryValue {
                                content: "Eat healthy!".to_string(),
                            }
                        }
                    ].to_vec(),
                },
                DesktopEntryGroup {
                    group_name: "No one's favorite food".to_string(),
                    entries: [
                        DesktopEntryGroupEntry {
                            locale: Some("en_US".to_string()),
                            key: "food1".to_string(),
                            value: EntryValue {
                                content: "I love rocks".to_string(),
                            }
                        },
                        DesktopEntryGroupEntry {
                            locale: Some("he_IL".to_string()),
                            key: "food2".to_string(),
                            value: EntryValue {
                                content: "אני דינוזאור".to_string(),
                            }
                        },
                        DesktopEntryGroupEntry {
                            locale: None,
                            key: "food3".to_string(),
                            value: EntryValue {
                                content: "Eat whatever!".to_string(),
                            }
                        }
                    ].to_vec()
                }
            ].to_vec()
        };
        let expected_content = 
r#"[Asaf's favorite food]
food[en_US]=I love pizza
food[he_IL]=אני אוהב פיצה
food=Eat healthy!

[No one's favorite food]
food1[en_US]=I love rocks
food2[he_IL]=אני דינוזאור
food3=Eat whatever!

"#;
        assert_eq!(expected_content, entry.to_string())
    }
}
