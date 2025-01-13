-- Add up migration script here

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
      name TEXT NOT NULL,
      slug TEXT NOT NULL UNIQUE,
      order_by TEXT NOT NULL, --how terms will be ordered
      created_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW(),
          updated_at TIMESTAMP
      WITH
          TIME ZONE DEFAULT NOW()
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
