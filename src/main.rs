use std::collections::HashMap;

#[derive(Debug)]
struct Category {
    id: String,
    name: String,
    product_ids: Vec<String>,
}

#[derive(Debug)]
struct Product {
    id: String,
    name: String,
    category_id: String,
    supplier_id: String,
    stock: u32,
    price_history: Vec<f64>,
}

#[derive(Debug)]
struct Supplier {
    id: String,
    name: String,
    contact_info: String,
}

#[derive(Debug)]
struct InventorySystem {
    categories: HashMap<String, Category>,
    producsts: HashMap<String, Product>,
    suppliers: HashMap<String, Supplier>,
}

impl InventorySystem {
    fn new() -> Self {
        let category_map: HashMap<String, Category> = HashMap::new();
        let products_map: HashMap<String, Product> = HashMap::new();
        let suppliers_map: HashMap<String, Supplier> = HashMap::new();

        Self {
            categories: category_map,
            producsts: products_map,
            suppliers: suppliers_map,
        }
    }

    fn add_category(&mut self, id: &str, name: &str) {
        let mut product_ids: Vec<String> = Vec::new();
        product_ids.push(id.to_string());

        let new_category = Category {
            id: id.to_string(),
            name: name.to_string(),
            product_ids: product_ids,
        };
        self.categories.insert(id.to_string(), new_category);
    }
}

fn main() {
    println!(".rs");
    let mut inventorydb = InventorySystem::new();
    inventorydb.add_category("category1", "first item of category1");
    inventorydb.add_category("category2", "second item of category2");
    inventorydb.add_category("category3", "third item of category 3");

    println!("inevntory db : {:#?}", inventorydb);
}
