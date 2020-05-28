# Table of Contents

- [Project Structure](#Project-Structure)
- [Templates List](#Templates-List)

# Project Structure

- `src`: project for source files
    - `database`: database related code, including database connection, schema and models
    - `server`: server related code
- `templates`: template files for front end
- `migrations`: migration files for Diesel
- `docs`: documentation files for this project

# Templates

The template files are stored in the `src/templates` folder.

The templates should be written in Liquid.

## Templates List

- `index.liquid`: the template rendered for home page.
- `posts.liquid`: the template rendered for displaying all posts.
- `post.liquid`: the template rendered for a single post.
- `404.liquid`: the template rendered when the resource requested is not found.
- `partials`: **the folder containing all partials.**
    - the partials can be imported directly with `<% include 'partial_name' %>`
- More templates to be added later
