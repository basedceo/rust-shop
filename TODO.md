# /src/model.rs
create a struct for product information


TODO
# /src/handler.rs
create CRUD handlers for attributes
file:///home/kenny/code/frontend/E-commerce/Wordpress/Attributes%20%E2%80%B9%20Based.win%20%E2%80%94%20WordPress.html

Multiple Terms will belong to a specific attribute
ex: Attribute name = color; terms = ['red', 'blue', 'green', 'orange']

create CRUD handlers for categories
https://based.win/wp-admin/edit-tags.php?taxonomy=product_cat&post_type=product

create CRUD handlers for tags
https://based.win/wp-admin/edit-tags.php?taxonomy=product_tag&post_type=product

TODO tera
update the attributes label in create_product_form.html to be a dropdown menu where multiple values can be selected

the pre-defined labels in the attributes menu should come from the attributes CRUD handler

update the variations label in create_product_form.html to be a dropdown menu where multiple values can be selected

the pre-defined labels in the variations label should come from the Terms connected to attributes as defined by the attributes CRUD handler

the pre-defined labels in the categories label should come from the categories CRUD handler
