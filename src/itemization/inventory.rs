use crate::itemization::{Item, ItemContainer, ItemContainerBuilder, ItemQuantity};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Inventory {
    item_containers: Vec<ItemContainer>,
    capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        let mut item_containers = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            item_containers.push(ItemContainer::default());
        }

        Self {
            item_containers,
            capacity: capacity,
        }
    }

    pub fn item_containers(&self) -> &Vec<ItemContainer> {
        &self.item_containers
    }

    pub fn get(&self, index: usize) -> Option<&ItemContainer> {
        self.item_containers.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut ItemContainer> {
        self.item_containers.get_mut(index)
    }
}

// adding items
impl Inventory {
    pub fn add_item(&mut self, item: Item, mut quantity: ItemQuantity) -> Option<ItemContainer> {
        let matching_item_containers = self.item_containers.iter_mut().filter(|container| {
            container.is_empty() == false && container.item().unwrap().id() == item.id()
        });

        for matching_item_container in matching_item_containers {
            quantity = matching_item_container.increase_quantity(quantity);

            if quantity == 0 {
                break;
            }
        }

        if quantity == 0 {
            return None;
        }

        let empty_item_container = self
            .item_containers
            .iter_mut()
            .find(|container| container.is_empty());

        match empty_item_container {
            Some(container) => {
                let quantity_to_add = item.maximum_quantity().min(quantity);

                quantity -= quantity_to_add;

                // since we might need to add more items, we need to clone the item so we have another copy to continue
                // the adding if needed
                container.set(item.clone(), quantity_to_add);

                match quantity {
                    value if value > 0 => {
                        // since we might need to add more items, we need to clone the item so we have another copy to
                        // continue the adding if needed
                        self.add_item(item.clone(), value);
                    }
                    _ => {}
                }
            }
            None => {
                println!("no empty item container found");
            }
        };

        if quantity == 0 {
            return None;
        }

        Some(ItemContainer::new(ItemContainerBuilder {
            item: Some(item),
            quantity: Some(quantity),
        }))
    }

    pub fn add_item_container(
        &mut self,
        mut item_container: ItemContainer,
    ) -> Option<ItemContainer> {
        match item_container.take_item() {
            Some(item) => self.add_item(item, item_container.quantity()),
            None => {
                println!("item container is empty");

                None
            }
        }
    }

    pub fn add_item_containers(
        &mut self,
        item_containers: Vec<ItemContainer>,
    ) -> Option<Vec<ItemContainer>> {
        let mut remaining_item_containers = Vec::new();

        for item_container in item_containers {
            let remaining_item_container = self.add_item_container(item_container);

            match remaining_item_container {
                Some(remaining_item_container) => {
                    remaining_item_containers.push(remaining_item_container)
                }
                None => {}
            }
        }

        return if remaining_item_containers.len() > 0 {
            Some(remaining_item_containers)
        } else {
            None
        };
    }
}

// setting items
impl Inventory {
    pub fn set_item(
        &mut self,
        index: usize,
        item: Item,
        quantity: ItemQuantity,
    ) -> Option<ItemContainer> {
        let item_container = self.get_mut(index);

        if item_container.is_none() {
            return Some(ItemContainer::new(ItemContainerBuilder {
                item: Some(item),
                quantity: Some(quantity),
            }));
        }

        let item_container = item_container.unwrap();

        if item_container.item().is_some() {
            return Some(ItemContainer::new(ItemContainerBuilder {
                item: Some(item),
                quantity: Some(quantity),
            }));
        }

        if quantity > item.maximum_quantity() {
            return Some(ItemContainer::new(ItemContainerBuilder {
                item: Some(item),
                quantity: Some(quantity),
            }));
        }

        item_container.set(item, quantity);

        None
    }
}

// removing items
impl Inventory {
    pub fn remove_item(&mut self, item: &Item, quantity: ItemQuantity) {
        if let Some(item_container) = self.find_item_container(item) {
            item_container.remove(quantity);
        }
    }

    pub fn remove_item_container(&mut self, item_container: &ItemContainer) {
        self.item_containers
            .retain(|container| container != item_container);
    }
}

// capacity
impl Inventory {
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn get_used_capacity(&self) -> usize {
        self.item_containers
            .iter()
            .filter(|container| container.is_empty() == false)
            .count()
    }

    pub fn get_remaining_capacity(&self) -> usize {
        self.item_containers
            .iter()
            .filter(|container| container.is_empty())
            .count()
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity > self.item_containers.len()
    }

    pub fn has_capacity_for(&self, quantity: ItemQuantity) -> bool {
        self.capacity >= self.item_containers.len() + quantity
    }
}

#[cfg(test)]
mod tests {
    use crate::itemization::{ItemBuilder, ItemContainerBuilder};

    use super::*;
    use pretty_assertions::assert_eq;

    fn create_test_item() -> Item {
        Item::new(ItemBuilder {
            id: String::from("TestItem"),
            name: String::from("Test Item"),
            maximum_quantity: Some(1),
            ..ItemBuilder::default()
        })
    }

    fn create_stacking_item() -> Item {
        Item::new(ItemBuilder {
            id: String::from("TestItem"),
            name: String::from("Test Item"),
            maximum_quantity: Some(10),
            ..ItemBuilder::default()
        })
    }

    fn create_non_stacking_item_container() -> ItemContainer {
        ItemContainer::new(ItemContainerBuilder {
            item: Some(create_test_item()),
            quantity: Some(1),
        })
    }

    fn create_stacking_item_container(quantity: ItemQuantity) -> ItemContainer {
        ItemContainer::new(ItemContainerBuilder {
            item: Some(create_stacking_item()),
            quantity: Some(quantity),
        })
    }

    // create
    #[test]
    fn create_empty_inventory() {
        let inventory = Inventory::new(10);

        assert_eq!(inventory.get_used_capacity(), 0);
        assert_eq!(inventory.capacity(), 10);
    }

    // get
    #[test]
    fn get_item_by_valid_index() {}

    #[test]
    fn get_item_with_invalid_index() {}

    // check
    #[test]
    fn check_for_item_that_exists() {}

    #[test]
    fn check_for_item_that_does_not_exist() {}

    #[test]
    fn check_for_item_that_exists_without_enough_quantity() {}

    #[test]
    fn check_for_multiple_items_that_all_existing() {}

    #[test]
    fn check_for_multiple_items_where_one_does_not_exist() {}

    #[test]
    fn check_for_multiple_items_where_one_does_not_have_enough_quantity() {}

    // add
    #[test]
    fn add_single_item() {
        let mut inventory = Inventory::new(10);
        let item = create_test_item();

        inventory.add_item(item, 1);

        assert_eq!(inventory.get_used_capacity(), 1);
        assert_eq!(inventory.get_remaining_capacity(), 9);
    }

    #[test]
    fn add_item_container() {
        let mut inventory = Inventory::new(10);

        inventory.add_item_container(create_non_stacking_item_container());

        assert_eq!(inventory.get_used_capacity(), 1);
        assert_eq!(inventory.get_remaining_capacity(), 9);
    }

    #[test]
    fn add_multiple_item_containers() {
        let mut inventory = Inventory::new(10);
        let item_containers = vec![
            create_non_stacking_item_container(),
            create_non_stacking_item_container(),
        ];

        inventory.add_item_containers(item_containers);

        dbg!(&inventory);

        assert_eq!(inventory.get_used_capacity(), 2);
        assert_eq!(inventory.get_remaining_capacity(), 8);
    }

    #[test]
    fn adds_to_existing_item_container() {
        let mut inventory = Inventory::new(10);

        inventory.add_item_containers(vec![create_stacking_item_container(5)]);
        inventory.add_item_container(create_stacking_item_container(3));

        let item_container = inventory.get(0).unwrap();

        assert_eq!(*item_container.item().unwrap(), create_stacking_item());
        assert_eq!(item_container.quantity(), 8);
        assert_eq!(inventory.get_used_capacity(), 1);
        assert_eq!(inventory.get_remaining_capacity(), 9);
    }

    #[test]
    fn adds_to_existing_item_container_with_overflow_to_new_item_container() {
        let mut inventory = Inventory::new(10);

        inventory.add_item_containers(vec![create_stacking_item_container(5)]);
        inventory.add_item_container(create_stacking_item_container(7));

        let item_container1 = inventory.get(0).unwrap();
        let item_container2 = inventory.get(1).unwrap();

        assert_eq!(*item_container1.item().unwrap(), create_stacking_item());
        assert_eq!(item_container1.quantity(), 10);
        assert_eq!(*item_container2.item().unwrap(), create_stacking_item());
        assert_eq!(item_container2.quantity(), 2);
        assert_eq!(inventory.get_used_capacity(), 2);
        assert_eq!(inventory.get_remaining_capacity(), 8);
    }

    #[test]
    fn adds_item_that_immediately_overflows_to_new_item_container() {
        let mut inventory = Inventory::new(10);

        inventory.add_item(create_stacking_item(), 10);
        inventory.add_item(create_stacking_item(), 2);

        let item_container1 = inventory.get(0).unwrap();
        let item_container2 = inventory.get(1).unwrap();

        assert_eq!(*item_container1.item().unwrap(), create_stacking_item());
        assert_eq!(item_container1.quantity(), 10);
        assert_eq!(*item_container2.item().unwrap(), create_stacking_item());
        assert_eq!(item_container2.quantity(), 2);
        assert_eq!(inventory.get_used_capacity(), 2);
        assert_eq!(inventory.get_remaining_capacity(), 8);
    }

    #[test]
    fn adds_item_that_itself_would_overflow() {
        let mut inventory = Inventory::new(10);

        inventory.add_item(create_stacking_item(), 12);

        let item_container1 = inventory.get(0).unwrap();
        let item_container2 = inventory.get(1).unwrap();

        assert_eq!(*item_container1.item().unwrap(), create_stacking_item());
        assert_eq!(item_container1.quantity(), 10);
        assert_eq!(*item_container2.item().unwrap(), create_stacking_item());
        assert_eq!(item_container2.quantity(), 2);
        assert_eq!(inventory.get_used_capacity(), 2);
        assert_eq!(inventory.get_remaining_capacity(), 8);
    }

    #[test]
    fn adds_item_that_can_not_fit_in_the_remaining_capacity() {
        let mut inventory = Inventory::new(1);

        inventory.add_item(create_stacking_item(), 8);

        let remaining_item_container = inventory.add_item(create_stacking_item(), 4).unwrap();
        let item_container1 = inventory.get(0).unwrap();

        assert_eq!(*item_container1.item().unwrap(), create_stacking_item());
        assert_eq!(item_container1.quantity(), 10);
        assert_eq!(
            *remaining_item_container.item().unwrap(),
            create_stacking_item()
        );
        assert_eq!(remaining_item_container.quantity(), 2);
        assert_eq!(inventory.get_used_capacity(), 1);
        assert_eq!(inventory.get_remaining_capacity(), 0);
    }

    // set
    #[test]
    fn sets_item_that_is_currently_empty() {
        let mut inventory = Inventory::new(1);

        let remain_item_container = inventory.set_item(0, create_stacking_item(), 10);

        let item_container1 = inventory.get(0).unwrap();

        assert_eq!(remain_item_container.is_none(), true);
        assert_eq!(*item_container1.item().unwrap(), create_stacking_item());
        assert_eq!(item_container1.quantity(), 10);
        assert_eq!(inventory.get_used_capacity(), 1);
        assert_eq!(inventory.get_remaining_capacity(), 0);
    }

    #[test]
    fn sets_item_that_is_currently_not_empty() {
        let mut inventory = Inventory::new(1);

        inventory.set_item(0, create_stacking_item(), 10);
        let remain_item_container = inventory.set_item(0, create_stacking_item(), 5);

        let item_container1 = inventory.get(0).unwrap();
        let remain_item_container = remain_item_container.unwrap();

        assert_eq!(
            *remain_item_container.item().unwrap(),
            create_stacking_item()
        );
        assert_eq!(remain_item_container.quantity(), 5);
        assert_eq!(*item_container1.item().unwrap(), create_stacking_item());
        assert_eq!(item_container1.quantity(), 10);
        assert_eq!(inventory.get_used_capacity(), 1);
        assert_eq!(inventory.get_remaining_capacity(), 0);
    }

    #[test]
    fn sets_item_to_invalid_index() {
        let mut inventory = Inventory::new(1);

        let remain_item_container = inventory.set_item(10, create_stacking_item(), 10);

        let item_container1 = inventory.get(0).unwrap();
        let remain_item_container = remain_item_container.unwrap();

        assert_eq!(
            *remain_item_container.item().unwrap(),
            create_stacking_item()
        );
        assert_eq!(remain_item_container.quantity(), 10);
        assert_eq!(item_container1.item().is_none(), true);
        assert_eq!(item_container1.quantity(), 0);
        assert_eq!(inventory.get_used_capacity(), 0);
        assert_eq!(inventory.get_remaining_capacity(), 1);
    }

    // removal
    #[test]
    fn remove_by_index() {}

    #[test]
    fn remove_by_index_with_quantity() {}

    #[test]
    fn remove_by_item_and_quantity_but_without_enough_quantity() {}

    #[test]
    fn remove_with_invalid_index() {}

    #[test]
    fn remove_by_item_container() {}

    #[test]
    fn remove_with_multiple_item_containers() {}

    #[test]
    fn remove_with_multiple_item_containers_but_without_enought_quantity_for_one() {}

    #[test]
    fn remove_by_item_and_quantity_that_required_removing_from_mutlitple_item_containers() {}

    #[test]
    fn remove_by_item_and_quantity_that_required_removing_from_mutlitple_item_containers_leaving_one_partial()
     {
    }

    // swap
    #[test]
    fn swap_item_containers() {}

    // count
    #[test]
    fn get_item_count_by_item_id() {}

    // capacity
    #[test]
    fn increase_capacity() {}

    #[test]
    fn decrease_capacity_without_removing_items() {}

    #[test]
    fn decrease_capacity_with_removing_items() {}

    #[test]
    fn has_capacity_for_valid() {}

    #[test]
    fn has_capacity_for_invalid() {}

    // index
    #[test]
    fn check_index_has_item() {}

    #[test]
    fn check_index_does_not_have_item() {}

    // crafting
    #[test]
    #[ignore]
    fn craft_item_with_single_input_and_single_output() {}

    #[test]
    #[ignore]
    fn craft_item_with_multiple_inputs_and_multiple_outputs() {}

    #[test]
    #[ignore]
    fn craft_item_without_enough_quantity() {}

    #[test]
    #[ignore]
    fn craft_item_adds_to_existing_item_container() {}

    #[test]
    #[ignore]
    fn craft_and_return_instead_of_adding_to_inventory() {}
}
