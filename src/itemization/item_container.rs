use crate::itemization::{Item, ItemQuantity};

#[derive(Default, PartialEq, Eq, Debug)]
pub struct ItemContainer {
    item: Option<Item>,
    quantity: ItemQuantity,
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct ItemContainerBuilder {
    pub item: Option<Item>,
    pub quantity: Option<ItemQuantity>,
}

impl ItemContainer {
    pub fn new(builder: ItemContainerBuilder) -> Self {
        Self {
            item: builder.item,
            quantity: builder.quantity.unwrap_or_default(),
        }
    }

    pub fn item(&self) -> Option<&Item> {
        self.item.as_ref()
    }

    pub fn take_item(&mut self) -> Option<Item> {
        self.item.take()
    }

    pub fn quantity(&self) -> ItemQuantity {
        self.quantity
    }

    pub fn is_empty(&self) -> bool {
        self.item.is_none() && self.quantity == 0
    }

    pub fn is_full(&self) -> bool {
        self.item()
            .map_or(false, |item| self.quantity == item.maximum_quantity())
    }

    pub fn set(&mut self, item: Item, quantity: ItemQuantity) -> Option<Self> {
        if self.item.is_none() == false {
            return Some(ItemContainer::new(ItemContainerBuilder {
                item: Some(item),
                quantity: Some(quantity),
            }));
        }

        self.item = Some(item);
        self.quantity = quantity;

        None
    }

    pub fn clear(&mut self) -> Self {
        Self {
            item: std::mem::take(&mut self.item), // Takes the value, leaving Default::default() in its place
            quantity: std::mem::replace(&mut self.quantity, 0), // Replaces with 0 and returns the old value
        }
    }

    pub fn increase_quantity(&mut self, added_quantity: ItemQuantity) -> ItemQuantity {
        match &self.item {
            Some(item) => {
                let new_desired_quantity = self.quantity + added_quantity;

                if new_desired_quantity > item.maximum_quantity() {
                    self.quantity = item.maximum_quantity();

                    println!("attempt to increase quantity that would exceed maximum quantity");

                    return new_desired_quantity - item.maximum_quantity();
                }

                self.quantity += added_quantity;
            }
            None => {
                println!("attempted to increase quantity to an item container that has no item")
            }
        }

        0
    }

    pub fn decrease_quantity(&mut self, removed_quantity: ItemQuantity) {
        match &self.item {
            Some(_) => {
                if removed_quantity > self.quantity {
                    println!(
                        "attempt to decrease an item container by {} but it only has {}",
                        removed_quantity, self.quantity
                    );

                    return;
                }

                self.quantity -= removed_quantity;
            }
            None => {
                println!("attempted to decrease quantity from an item container that has no item")
            }
        }

        if self.quantity > 0 {
            return;
        }

        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::itemization::item::ItemBuilder;

    use super::*;
    use pretty_assertions::assert_eq;

    fn create_none_stacking_item() -> Item {
        // Item::new(String::from("TestItem"), String::from("Test Item"))
        Item::new(ItemBuilder {
            id: String::from("TestItem"),
            name: String::from("Test Item"),
            ..Default::default()
        })
    }

    fn create_none_stacking_item2() -> Item {
        // Item::new(String::from("TestItem2"), String::from("Test Item2"))
        Item::new(ItemBuilder {
            id: String::from("TestItem2"),
            name: String::from("Test Item2"),
            ..Default::default()
        })
    }

    fn create_stacking_item() -> Item {
        // Item::new(String::from("TestItem2"), String::from("Test Item2")).with_maximum_quantity(10)
        Item::new(ItemBuilder {
            id: String::from("TestItem2"),
            name: String::from("Test Item2"),
            maximum_quantity: Some(10),
            ..Default::default()
        })
    }

    // creation
    #[test]
    fn create_new_empty_item_container() {
        let item_container = ItemContainer::new(ItemContainerBuilder::default());

        assert!(item_container.item.is_none());
        assert_eq!(item_container.quantity, 0);
    }

    #[test]
    fn create_new_item_container_with_item_and_quantity() {
        let item = create_none_stacking_item();
        let item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });

        assert!(item_container.item.is_some());
        assert_eq!(item_container.quantity, 5);
    }

    // utility
    #[test]
    fn clear() {
        let item = create_none_stacking_item();
        let mut item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });
        let removed_item_container = item_container.clear();
        let removed_item = removed_item_container.item.unwrap();

        assert!(item_container.item.is_none());
        assert_eq!(item_container.quantity, 0);
        assert_eq!(removed_item, create_none_stacking_item());
        assert_eq!(removed_item_container.quantity, 5);
    }

    #[test]
    fn check_quantity_is_full_when_it_is() {
        let item = create_none_stacking_item();
        let item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(1),
        });
        let is_full = item_container.is_full();

        assert_eq!(is_full, true);
    }

    #[test]
    fn check_quantity_is_full_when_it_is_not() {
        let item = create_stacking_item();
        let item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });
        let is_full = item_container.is_full();

        assert_eq!(is_full, false);
    }

    // set
    #[test]
    fn set_with_no_stored_item() {
        let item = create_none_stacking_item();
        let mut item_container = ItemContainer::new(ItemContainerBuilder::default());
        let existing_item_container = item_container.set(item, 5);

        assert!(existing_item_container.is_none());
        assert_eq!(item_container.item.unwrap(), create_none_stacking_item());
        assert_eq!(item_container.quantity, 5);
    }

    #[test]
    fn set_with_existing_stored_item() {
        let item = create_none_stacking_item();
        let item2 = create_none_stacking_item2();
        let mut item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });
        let existing_item_container = item_container.set(item2, 10).unwrap();

        assert_eq!(
            existing_item_container.item.unwrap(),
            create_none_stacking_item2()
        );
        assert_eq!(existing_item_container.quantity, 10);
        assert_eq!(item_container.item.unwrap(), create_none_stacking_item());
        assert_eq!(item_container.quantity, 5);
    }

    // increase
    #[test]
    fn increase_existing_quantity_below_limit() {
        let item = create_stacking_item();
        let mut item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });
        let remaining_quantity = item_container.increase_quantity(5);

        assert_eq!(remaining_quantity, 0);
        assert_eq!(item_container.item.unwrap(), create_stacking_item());
        assert_eq!(item_container.quantity, 10);
    }

    #[test]
    fn increase_existing_quantity_above_limit() {
        let item = create_stacking_item();
        let mut item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(9),
        });
        let remaining_quantity = item_container.increase_quantity(5);

        assert_eq!(remaining_quantity, 4);
        assert_eq!(item_container.item.unwrap(), create_stacking_item());
        assert_eq!(item_container.quantity, 10);
    }

    // decrease
    #[test]
    fn decrease_existing_quantity_below_zero() {
        let item = create_stacking_item();
        let mut item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });

        item_container.decrease_quantity(6);

        assert_eq!(item_container.item.unwrap(), create_stacking_item());
        assert_eq!(item_container.quantity, 5);
    }

    #[test]
    fn decrease_existing_quantity_above_zero() {
        let item = create_stacking_item();
        let mut item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });

        item_container.decrease_quantity(4);

        assert_eq!(item_container.item.unwrap(), create_stacking_item());
        assert_eq!(item_container.quantity, 1);
    }

    #[test]
    fn decrease_existing_quantity_to_zero() {
        let item = create_stacking_item();
        let mut item_container = ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(5),
        });

        item_container.decrease_quantity(5);

        assert!(item_container.item.is_none());
        assert_eq!(item_container.quantity, 0);
    }
}
