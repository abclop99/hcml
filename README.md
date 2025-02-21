# HCML

HCML is inspired by [HAML][haml], [Pug][pug], and [HBML][hbml].
It was created to address some of the limitations of using HAML and Pug in Jekyll.

## Features

- **Witespace insensitive:** Structure does not rely on whitespace (unlike [HAML][haml] and [Pug][pug]), so templating engines can insert HTML content without special handling.
- **Element tags:**
	- The ID and class of an element can be defined directly in the tag
		```hcml
		name#id.class1.class2
		```
		↓
		```html
		<name id="id" class="class1 class2">
			...
		</name>
		```
	- The `div` element name can be omitted if the element tag has an ID or class
		```hcml
		#titlebar
		```
		↓
		```html
		<div id="titlebar">
			...
		</div>
		```
- **Attributes:**
	- Defined in parentheses after the tag
		```hcml
		a(href="/example")
		```
		↓
		```html
		<a href="/example"></a>
		```
	- Separated by either whitespace or a comma
	- Classes and IDs can also be defined in the attributes, but it's more concise to define them in the tag
- **Child elements:**
	- Child elements follow the tag of the element
		```hcml
		div p "HCML!"
		```
		↓
		```html
		<div>
			<p>HCML!</p>
		</div>
		```
	- Multiple child elements must be surrounded by curly braces
		```hcml
		ul {
			li "one"
			li "two"
			li "three"
		}
		```
		↓
		```html
		<ul>
			<li> one   </li>
			<li> two   </li>
			<li> three </li>
		</ul>
		```
	- Self-closing elements end with a `;`
		- HCML does not know which elements should be void
		```hcml
		br;
		```
		↓
		```html
		<br>
		```
- **Strings:**
	- Strings can be created by surrounding text with `"`
		- Newlines and other characters are included verbatim.
		```hcml
		p "This text is <strong>strong!</strong>"
		```
		↓
		```html
		<p> This text is <strong>strong!</strong> </p>
		```
	- Raw strings can be created like `r##" #"# "##`, as in Rust
		- This can be used to insert HTML.
		```hcml
		div r#"
			<div class="example">
				Maybe inserted by some program
			</div>
		"#
		```
		↓
		```html
		<div>
			<div class="example">
				Maybe inserted by some program
			</div>
		</div>
		```
- **Special tags:**
	- Not yet implemented
	- Tags beginning with `!`, intended to provide special behavior
- **Comments:**
	- Regular comments (`//`, `/* */`) are ignored
		```hcml
		// This will be ignored
		p /* This too */ "testing123"
		```
		↓
		```html
		<p> testing123 </p>
		```
	- HTML comments can be created using `//-` or `/*- -*/`
		- Note: Will not work everywhere yet.
		```hcml
		//- Comment 1
		/*- Comment 2 -*/
		p "Hello world"
		```
		↓
		```html
		<!-- Comment 1 -->
		<!-- Comment 2 -->
		<p> Hello world </p>
		```
[haml]: https://haml.info/
[hbml]: https://reddit.com/r/ProgrammerHumor/comments/zd8ljb/i_taught_the_chat_bot_an_alternative_syntax_for
[pug]: https://pugjs.org/
