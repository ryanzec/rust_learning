use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub enum ItemType {
    Resource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EquipmentType {
    None,
}

pub type ItemQuantity = u16;

pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub item_type: ItemType,
    pub equipment_type: EquipmentType,
    pub maximum_quantity: ItemQuantity,
}

pub struct ItemNewOptions {
    pub name: String,
    pub item_type: Option<ItemType>,
    pub equipment_type: Option<EquipmentType>,
    pub maximum_quantity: Option<ItemQuantity>,
}

fn main() {}

impl Item {
    pub fn new(options: ItemNewOptions) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: options.name,
            item_type: options.item_type.unwrap_or(ItemType::Resource),
            equipment_type: options.equipment_type.unwrap_or(EquipmentType::None),
            maximum_quantity: options.maximum_quantity.unwrap_or(1),
        }
    }
}

// it is pretty standard for the test to live in the same file as the code it tests

// this will make sure the test code is not included in builds / runs
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    // this allow you to ignore a test
    #[ignpore]
    fn can_create_item_will_minimum_options() {
        let item = Item::new(ItemNewOptions {
            name: "Test Item".to_string(),
            item_type: None,
            equipment_type: None,
            maximum_quantity: None,
        });

        assert_ne!(item.id, Uuid::default());
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.item_type, ItemType::Resource);
        assert_eq!(item.equipment_type, EquipmentType::None);
        assert_eq!(item.maximum_quantity, 1);
    }

    #[test]
    fn can_create_item_with_all_options() {
        let item = Item::new(ItemNewOptions {
            name: "Test Item".to_string(),
            item_type: Some(ItemType::Resource),
            equipment_type: Some(EquipmentType::None),
            maximum_quantity: Some(10),
        });

        assert_ne!(item.id, Uuid::default());
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.item_type, ItemType::Resource);
        assert_eq!(item.equipment_type, EquipmentType::None);
        assert_eq!(item.maximum_quantity, 10);
    }
}
