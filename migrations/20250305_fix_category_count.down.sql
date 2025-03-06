-- Drop the updated trigger and function
DROP TRIGGER IF EXISTS after_product_insert ON products;
DROP FUNCTION IF EXISTS update_category_count();

-- Recreate the original function to update category count when a product is added
CREATE OR REPLACE FUNCTION update_category_count()
RETURNS TRIGGER AS $$
BEGIN
    -- Increment the count for the category of the new product
    UPDATE product_categories
    SET count = count + 1
    WHERE slug = NEW.category;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the function after a product is inserted
CREATE TRIGGER after_product_insert
AFTER INSERT ON products
FOR EACH ROW
EXECUTE FUNCTION update_category_count();
