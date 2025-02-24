CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
  IF NOT EXISTS product_attributes (
      id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
      name TEXT NOT NULL,
      slug TEXT NOT NULL UNIQUE,
      order_by TEXT NOT NULL, --how terms will be ordered
      --terms TEXT[] NOT NULL,
      created_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW(),
          updated_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW()
  );

CREATE TABLE
  IF NOT EXISTS product_categories (
      id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
      parent_id UUID REFERENCES product_categories(id) ON DELETE CASCADE,
      lvl TEXT NOT NULL,
      parent TEXT NOT NULL,
      name TEXT NOT NULL,
      description TEXT NOT NULL,
      slug TEXT NOT NULL UNIQUE,
      display_type TEXT NOT NULL, --how terms will be ordered
      thumbnail VARCHAR(255) NOT NULL, --image uploaded with product category
      created_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW(),
          updated_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW()
  );

-- Step 1: Add a count column to the product_categories table
ALTER TABLE product_categories
ADD COLUMN count INTEGER NOT NULL DEFAULT 0;

-- Create a function to update count
CREATE OR REPLACE FUNCTION update_count()
RETURNS TRIGGER AS $$
BEGIN
    -- Increment the count when a product is added
    IF (TG_OP = 'INSERT') THEN
        UPDATE product_categories
        SET count = count + 1
        WHERE slug = NEW.category;
    END IF;

    -- Decrement the count when a product is deleted
    IF (TG_OP = 'DELETE') THEN
        UPDATE product_categories
        SET count = count - 1
        WHERE slug = OLD.category;
    END IF;

    -- Adjust the count when a product's category is updated
    IF (TG_OP = 'UPDATE') THEN
        -- Decrement the old category count
        UPDATE product_categories
        SET count = count - 1
        WHERE slug = OLD.category;

        -- Increment the new category count
        UPDATE product_categories
        SET count = count + 1
        WHERE slug = NEW.category;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Creating default "Uncategorized" category
INSERT INTO product_categories (
    id,
    lvl,
    parent,
    name,
    description,
    slug,
    display_type,
    thumbnail,
    created_at,
    updated_at,
    count
  )
VALUES (
    uuid_generate_v4(),                  -- Unique ID for the category
    '0',                                 -- Level of the category
    '-1',                                -- Parent ID (indicating no parent)
    'Uncategorized',                     -- Name of the category
    '-',                                 -- Description of the category
    'uncategorized',                     -- Slug for the category
    'Default',                           -- Display type
    'frontend/img/categories/woocommerce-placeholder-300x300.png', -- Thumbnail image
    NOW(),                               -- Created timestamp
    NOW(),                               -- Updated timestamp
    0                                    -- Initial product count
  );

CREATE TABLE
  IF NOT EXISTS product_terms (
      product_id UUID REFERENCES product_attributes(id) ON DELETE CASCADE, -- Foreign key to `product_attributes`
      --TODO update term_1 to name, term_2 to slug, and term_3 to description
      name TEXT NOT NULL,    -- First string in the array (NOT NULL)
      slug TEXT NOT NULL,    -- Second string in the array (NOT NULL)
      description TEXT,             -- Third string in the array (can be NULL)
      PRIMARY KEY (product_id, name, slug) -- Composite primary key
  );

CREATE TABLE
  IF NOT EXISTS products (
      -- if an item in model.rs does not have a an Option<> then you must use NOT NULL in the db
      id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
      title VARCHAR(255) NOT NULL UNIQUE,
      description TEXT NOT NULL,
      category VARCHAR(100) NOT NULL,
      price TEXT NOT NULL,
      sku TEXT NOT NULL,
      product_type TEXT NOT NULL,
      stock TEXT NOT NULL,
      -- allow_backorders BOOLEAN DEFAULT FALSE,
      allow_backorders VARCHAR(8) NOT NULL,
      low_stock_threshold VARCHAR(3) NOT NULL,
      shipping_weight VARCHAR(8) NOT NULL,
      product_gallery VARCHAR(255) NOT NULL,
      attributes VARCHAR(255) NOT NULL,
      variations VARCHAR(255) NOT NULL,
      shipping_dimensions VARCHAR(255) NOT NULL,
      shipping_class VARCHAR(255) NOT NULL,
      tax_status VARCHAR(255) NOT NULL,
      tax_class VARCHAR(255) NOT NULL,
      published BOOLEAN DEFAULT FALSE,
      created_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW(),
          updated_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW()
  );

-- Step 3: Create a trigger on the products table
CREATE TRIGGER product_category_count_trigger
AFTER INSERT OR DELETE OR UPDATE ON products
FOR EACH ROW
EXECUTE FUNCTION update_count();
