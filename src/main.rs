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
    suppliers: HashMap<String, Supplier>,
    products: HashMap<String, Product>,
}

impl InventorySystem {
    fn new() -> Self {
        let category_map: HashMap<String, Category> = HashMap::new();
        let products_map: HashMap<String, Product> = HashMap::new();
        let suppliers_map: HashMap<String, Supplier> = HashMap::new();

        Self {
            categories: category_map,
            products: products_map,
            suppliers: suppliers_map,
        }
    }

    fn add_category(&mut self, id: &str, name: &str) {
        let new_category = Category {
            id: id.to_string(),
            name: name.to_string(),
            product_ids: vec![],
        };
        self.categories.insert(id.to_string(), new_category);
    }

    fn add_supplier(&mut self, supplier_id: &str, supplier_name: &str, supplier_contact: &str) {
        let supplier_man = Supplier {
            id: supplier_id.to_string(),
            name: supplier_name.to_string(),
            contact_info: supplier_contact.to_string(),
        };
        self.suppliers.insert(supplier_id.to_string(), supplier_man);
    }

    fn add_product(
        &mut self,
        id: &str,
        name: &str,
        category_id: &str,
        supplier_id: &str,
        stock: u32,
        initial_price: f64,
    ) {
        if self.suppliers.contains_key(&supplier_id.to_string()) {
            let new_product = Product {
                id: id.to_string(),
                name: name.to_string(),
                category_id: category_id.to_string(),
                supplier_id: supplier_id.to_string(),
                stock: stock,
                price_history: vec![initial_price],
            };

            self.products.insert(id.to_string(), new_product);

            if let Some(newproduct) = self.categories.get_mut(&category_id.to_string()) {
                // println!("product found : {:?}", newproduct);
                newproduct.product_ids.push(id.to_string());
            } else {
                println!("noithing");
            }
        } else {
            println!("supplier id does not exists");
        }
    }
}

fn main() {
    println!(".rs");
    let mut inventorydb = InventorySystem::new();
    inventorydb.add_category("category1", "first item of category1");
    inventorydb.add_category("category2", "second item of category2");
    inventorydb.add_category("category3", "third item of category 3");

    println!("inevntory categories : {:#?}", inventorydb.categories);

    inventorydb.add_supplier("supplier1", "first_supplier", "supplier1@supplier.com");
    println!("inventory suppliers : {:#?}", inventorydb.suppliers);

    inventorydb.add_product(
        "product1",
        "first product",
        "category1",
        "supplier1",
        3,
        45.0,
    );
    println!("inventory products : {:#?}", inventorydb.products);

    println!("inventory categories : {:#?}", inventorydb.categories);
}
