# Rust Markup Language
This is a small project that takes a file in the rml format and displays it.

# Rml reference
Rml is inspired by html and thus has multiple tags that can have attributes and children. A rml page needs two base tags, the head and body tags. Only some tags can contain other tags inside themselves.

## Supported head tags
- title, the title of the page
- script, a lua script

## Supported body tags and their attributes
- p, a paragraph
- h, a heading
- button, a button
    - onclick, lua code that runs when the button is clicked
- div, an element containing other elements
    - direction, can be up, down, left, or right, the direction the elements inside the div flow
    - align, can be min, center, or max, the alignment of elements perpendicularly to the direction
- space, adds empty space
- divider, a line
- weblink, a link that opens in the browser
    - dst, where the link leads
- link, a link to another rml page
    - dst, where the link leads
- fakelink, a link that acts as a button
    - onclick, lua code that runs when the link is clicked

# Lua reference
The lua scripting is run using Lua version 5.4 and there is a special api to interact with the rml page. Some of the functions in the api use a path, this is a table of indexes eg. the first element of the second element in the body tag would be `{1,0}`.

## Document api
- `document:set_text(path_to_element, text)`
- `document:set_inner(path_to_element, rml_as_string)`
- `document:set_attr(path_to_element, attribute_name, attribute_value)`
- `document:log(string_to_log)`
- `document:set_location(new_rml_page)`
- `document:open_url(url_to_open)`
- `document:set_title(new_title)`