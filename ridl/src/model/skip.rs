use super::*;
use std::collections::HashSet;

impl KMod {
    /// Removes all items with names in `skippings` list.
    pub fn retain_only_non_skipping_items(&mut self, skippings: &Vec<String>) {
        let set = skippings.iter().map(|x| x.as_str()).collect::<HashSet<&str>>();
        self.retain_only_non_skipping_items_in_set(&set);
    }
    fn retain_only_non_skipping_items_in_set(&mut self, skippings: &HashSet<&str>) {
        self.items.retain(|x| !skippings.contains(x.name()));
        for item in self.items.iter_mut() {
            item.retain_only_non_skipping_items_in_set(skippings);
        }
    }
}
impl KItem {
    fn retain_only_non_skipping_items_in_set(&mut self, skippings: &HashSet<&str>) {
        match self {
            KItem::Mod(x) => x.retain_only_non_skipping_items_in_set(skippings),
            _ => (),
        }
    }
}
