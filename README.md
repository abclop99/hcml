# HCML

Note: AI generated

HCML ~~(Hypertext Compact Markup Language)~~ is a lightweight, whitespace-insensitive markup language designed to simplify writing HTML. It provides a concise and expressive syntax for defining HTML structure, classes, IDs, and attributes, while maintaining readability and flexibility.

Inspired by languages like HAML and Pug, HCML aims to reduce boilerplate code and make HTML authoring faster and more enjoyable.

---

## Features

- **Whitespace Insensitive**: No need to worry about indentation or spacing.
- **Concise Syntax**: Write less code with shorthand for elements, IDs, and classes.
- **Rust-Like Strings**: Supports multi-line strings, raw strings, and string literals.
- **Attributes in Parentheses**: Define attributes using a simple `key="value"` syntax.
- **Self-Closing Elements**: End self-closing elements with a `;`.
- **Conditional Comments**: Use `![if IE]` for conditional comments.
- **HTML Comments**: Add HTML comments with `//-` or `/*- ... -*/`.
- **Rust Integration**: Comes with a Rust-based parser and translator.

---

## Syntax Overview

### Elements
Define elements using their tag name, followed by optional IDs and classes:
```hcml
div#main.container "Hello, World!"
```
Translates to:
```html
<div id="main" class="container">Hello, World!</div>
```

### IDs and Classes
IDs start with `#`, and classes start with `.`. The `div` tag can be omitted:
```hcml
#header.bg-blue "Welcome!"
```
Translates to:
```html
<div id="header" class="bg-blue">Welcome!</div>
```

### Attributes
Use parentheses to define attributes:
```hcml
a(href="/example", target="_blank") "Click me"
```
Translates to:
```html
<a href="/example" target="_blank">Click me</a>
```

### Self-Closing Elements
End self-closing elements with a `;`:
```hcml
img(src="/logo.png");
```
Translates to:
```html
<img src="/logo.png" />
```

### Nested Elements
Use curly braces `{}` for nesting. Braces are optional for single-child elements:
```hcml
ul {
    li "Home"
    li "About"
    li "Contact"
}
```
Translates to:
```html
<ul>
    <li>Home</li>
    <li>About</li>
    <li>Contact</li>
</ul>
```

### Strings
Supports multi-line strings and Rust-like raw strings:
```hcml
p "This is a multi-line string: \
   Line one. \
   Line two."
p ##"This is a raw string with #"# inside it."##
```
Translates to:
```html
<p>This is a multi-line string: Line one. Line two.</p>
<p>This is a raw string with #"# inside it.</p>
```

### Comments
- **HCML Comments**: Ignored during translation.
  ```hcml
  // This is a line comment
  /* This is a block comment */
  ```
- **HTML Comments**: Preserved in the output.
  ```hcml
  //- This is an HTML comment
  /*- This is also an HTML comment -*/
  ```

### Conditional Comments
Use `![...]` for conditional comments:
```hcml
![if IE] "This is for Internet Explorer"
```
Translates to:
```html
<!--[if IE]>This is for Internet Explorer<![endif]-->
```

---

## Installation

HCML comes with a Rust-based parser and translator. To use it:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/hcml.git
   cd hcml
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the translator:
   ```bash
   cargo run -- input.hcml output.html
   ```

---

## Example

### Input (`input.hcml`)
```hcml
//- This is an HTML comment
#header.bg-blue "Welcome to HCML!"

div.content {
    h1 "Hello, World!"
    p "This is a paragraph with a " a(href="/link") "link".
    p "Here's a multi-line string: \
       Line one. \
       Line two."
    p ##"This is a raw string with #"# inside it."##
}

![if IE] "This is for Internet Explorer"
```

### Output (`output.html`)
```html
<!-- This is an HTML comment -->
<div id="header" class="bg-blue">Welcome to HCML!</div>

<div class="content">
    <h1>Hello, World!</h1>
    <p>This is a paragraph with a <a href="/link">link</a>.</p>
    <p>Here's a multi-line string: Line one. Line two.</p>
    <p>This is a raw string with #"# inside it.</p>
</div>

<!--[if IE]>This is for Internet Explorer<![endif]-->
```

---

## Grammar

HCML uses a `.pest` grammar for parsing. See the [grammar file](src/hcml.pest) for details.

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

---

## License

HCML is licensed under the ~~MIT License~~. See [LICENSE](LICENSE) for details.

---

Enjoy writing HTML with HCML! 🚀

--- 

Let me know if you need further adjustments!
