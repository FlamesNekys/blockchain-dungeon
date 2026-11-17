#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Inventory {
    items: Vec<String>,
    equipped: Option<String>,
}

impl Inventory {
    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }

    #[must_use]
    pub fn items(&self) -> &[String] {
        self.items.as_slice()
    }

    #[must_use]
    pub fn equipped(&self) -> Option<&str> {
        self.equipped.as_deref()
    }

    pub fn remove(&mut self, name: &str) -> Option<String> {
        let index = self.items.iter().position(|item| *item == name)?;
        Some(self.items.remove(index))
    }

    pub fn equip(&mut self, name: &str) -> bool {
        if let Some(item) = self.remove(name) {
            if let Some(prev) = self.equipped.replace(item) {
                self.items.push(prev);
            }
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn longest_item_name(&self) -> Option<&str> {
        let mut longest: Option<&str> = None;
        for item in &self.items {
            match longest {
                Some(name) if item.len() > name.len() => longest = Some(item),
                Some(_) => (),
                None => longest = Some(item),
            }
        }
        longest
    }

    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.items.contains(&String::from(name))
    }
}
