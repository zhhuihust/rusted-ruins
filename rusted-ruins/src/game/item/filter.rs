
use common::gobj;
use common::gamedata::GameData;
use common::gamedata::item::*;

/// Used for creating filtered list and saving filtering state
#[derive(Clone, Copy, Debug)]
pub struct ItemFilter {
    pub all: bool,
    pub kind: Option<ItemKind>,
}

impl ItemFilter {
    pub fn new() -> ItemFilter {
        ItemFilter::default()
    }
    
    pub fn all() -> ItemFilter {
        let mut filter = ItemFilter::default();
        filter.all = true;
        filter
    }
    
    /// Given item will be filtered (false) or not (true)
    pub fn judge(&self, item: &Item) -> bool {
        if self.all { return true; }
        let o = gobj::get_obj(item.idx);
        
        if let Some(kind) = self.kind {
            if o.content.kind() != kind { return false; }
        }
        
        true
    }

    pub fn kind(mut self, kind: ItemKind) -> ItemFilter {
        self.kind = Some(kind);
        self
    }
}

impl Default for ItemFilter {
    fn default() -> ItemFilter {
        ItemFilter {
            all: false,
            kind: None,
        }
    }
}

#[derive(Clone)]
pub struct FilteredItemList<'a> {
    item_list: &'a ItemList,
    location: ItemListLocation,
    filter: ItemFilter,
    count: usize,
}

impl<'a> FilteredItemList<'a> {
    pub fn new(item_list: &'a ItemList, location: ItemListLocation,
               filter: ItemFilter) -> FilteredItemList<'a> {
        
        FilteredItemList {
            item_list, location, filter, count: 0,
        }
    }

    pub fn all(item_list: &'a ItemList, location: ItemListLocation) -> FilteredItemList<'a> {
        
        FilteredItemList {
            item_list, location, filter: ItemFilter::all(), count: 0,
        }
    }
}

impl<'a> Iterator for FilteredItemList<'a> {
    type Item = (ItemLocation, &'a Item, u32);
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.item_list.items.len() <= self.count {
                return None
            }
            let a = &self.item_list.items[self.count];

            let prev_count = self.count;
            self.count += 1;

            if self.filter.judge(&*a.0) {
                return Some(((self.location, prev_count as u32), &*a.0, a.1));
            }
        }
    }
}

pub trait FilteredListHolder {
    fn get_filtered_item_list(&self, list_location: ItemListLocation, filter: ItemFilter)
                                   -> FilteredItemList;
}

impl FilteredListHolder for GameData {
    fn get_filtered_item_list(&self, list_location: ItemListLocation, filter: ItemFilter)
                              -> FilteredItemList {
        let item_list = self.get_item_list(list_location);
        FilteredItemList::new(item_list, list_location, filter)
    }
}