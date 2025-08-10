use crate::itemization::{EquipmentType, ItemQuantity, ItemType};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Item {
    id: String,
    name: String,
    item_type: ItemType,
    equipment_type: EquipmentType,
    maximum_quantity: ItemQuantity,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ItemBuilder {
    pub id: String,
    pub name: String,
    pub item_type: Option<ItemType>,
    pub equipment_type: Option<EquipmentType>,
    pub maximum_quantity: Option<ItemQuantity>,
}

impl Item {
    pub fn new(builder: ItemBuilder) -> Self {
        Item {
            id: builder.id,
            name: builder.name,
            item_type: builder.item_type.unwrap_or_default(),
            equipment_type: builder.equipment_type.unwrap_or_default(),
            maximum_quantity: builder.maximum_quantity.unwrap_or(1),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type
    }

    pub fn equipment_type(&self) -> EquipmentType {
        self.equipment_type
    }

    pub fn maximum_quantity(&self) -> ItemQuantity {
        self.maximum_quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create_item_with_minimum_options() {
        let item = Item::new(ItemBuilder {
            id: String::from("TestItem"),
            name: String::from("Test Item"),
            ..ItemBuilder::default()
        });

        assert_eq!(item.id, String::from("TestItem"));
        assert_eq!(item.name, String::from("Test Item"));
        assert_eq!(item.item_type, ItemType::Resource);
        assert_eq!(item.equipment_type, EquipmentType::None);
        assert_eq!(item.maximum_quantity, 1);
    }

    #[test]
    fn create_item_with_all_options() {
        let item = Item::new(ItemBuilder {
            id: String::from("TestItem"),
            name: String::from("Test Item"),
            item_type: Some(ItemType::Equipment),
            equipment_type: Some(EquipmentType::Head),
            maximum_quantity: Some(10),
        });

        assert_eq!(item.id, String::from("TestItem"));
        assert_eq!(item.name, String::from("Test Item"));
        assert_eq!(item.item_type, ItemType::Equipment);
        assert_eq!(item.equipment_type, EquipmentType::Head);
        assert_eq!(item.maximum_quantity, 10);
    }
}
