# LOLCODE test cases and expected results

Assumes the fixes from my review are applied (`=` and `/` allowed, `<br>`, bold may contain a variable use, argument count 2, line fallback 1).
Whitespace in the HTML is approximate; only tags, order, and text matter.
On any error: message goes to the console, NO .html file is created, browser does not open.
Lexical errors are always reported before syntax errors, because the whole file is tokenized first.

## Valid files (compile and open in the browser)

### v01_basic_document.lol
Browser tab title: My First Page. Page shows "Welcome to my page.", a line break, then "This is the second line."
```html
<html>
<!-- A basic test file -->
<head>
<title> My First Page </title>
</head>
<p> Welcome to my page. <br>
This is the second line. </p>
</html>
```

### v02_inline_formatting.lol
Shows: "Plain text then **bold words here** and more plain text." / "Visit [www.towson.edu] for info." / "Done!" (comment invisible).
```html
<html>
<p> Plain text then <b> bold words here </b> and more plain text. <br>
Visit <a href="www.towson.edu">www.towson.edu</a> for info. <br>
<!-- this is an inline comment -->
Done! </p>
</html>
```

### v03_list_variations.lol
Title: Grocery List. Bulleted list with 5 items: Apples / **Organic** milk / Recipes at [link] / Eggs (line break) dozen / Bread.
```html
<html>
<head>
<title> Grocery List </title>
</head>
<p> Things to buy: <br>
<ul>
<li> Apples </li>
<li> <b> Organic </b> milk </li>
<li> Recipes at <a href="www.recipes.com">www.recipes.com</a> </li>
<li> Eggs <br>
dozen </li>
<li> <!-- hidden note -->
Bread </li>
</ul>
</p>
</html>
```

### v04_global_variable.lol
Needs the bold-with-variable fix. Shows: "I have a dog at home." / "My dog is called Rex and the **dog** is friendly." / bullets "Feed the dog", "Walk the dog".
```html
<html>
I have a dog at home. <br>
<p> My dog is called Rex and the <b> dog </b> is friendly.
<ul>
<li> Feed the dog </li>
<li> Walk the dog </li>
</ul>
</p>
</html>
```

### v05_nested_scoping.lol
Shows: Outside: Red / In paragraph: Blue / bullets "Item one: Green", "Item two: Blue" / After list: Blue / Back outside: Red.
```html
<html>
Outside: Red <br>
<p> In paragraph: Blue <br>
<ul>
<li> Item one: Green </li>
<li> Item two: Blue </li>
</ul>
After list: Blue </p>
Back outside: Red </html>
```

### v06_case_insensitive.lol
Annotations in any case work; plain text keeps its case. Title: Case Test. Shows "Text keeps its case: Hello World", line break, "**SHOUT**".
```html
<html>
<!-- Mixed case comment -->
<head>
<title> Case Test </title>
</head>
<p> Text keeps its case: Hello World <br>
<b> SHOUT </b> </p>
</html>
```

### v07_comments_everywhere.lol
All comments are invisible. Shows "Some text more text" and one bullet "Item text". Title: Comments.
```html
<html>
<!-- first comment -->
<!-- second comment -->
<head>
<title> Comments </title>
</head>
<p> <!-- comment at start of paragraph -->
Some text <!-- comment in the middle -->
more text
<ul>
<li> <!-- comment in item -->
Item text </li>
</ul>
</p>
<!-- comment in the body -->
</html>
```

### v08_body_without_paragraph.lol
Text, bold, link, and list are all legal directly in the body. Shows loose text, line break, bold text, a link, one bullet, then two paragraphs.
```html
<html>
Loose text in the body. <br>
<b> Bold in the body </b> <a href="www.example.com">www.example.com</a> <ul>
<li> Listed outside a paragraph </li>
</ul>
<p> Paragraph one <!-- c -->
</p>
<p> Paragraph two </p>
</html>
```

### v09_empty_paragraph_and_multiword_value.lol
First paragraph is empty (shows nothing). Second shows "one two three" (multi-word variable value).
```html
<html>
<p> </p>
<p> one two three </p>
</html>
```

### v10_minimal.lol
Blank page, no errors.
```html
<html>
</html>
```

### v11_big_document.lol
Needs the bold-with-variable fix. Title: The Simpsons Family. Last bullet of the first list is "Ned Flanders" (item-local variable), the others are "Simpson". Second paragraph says Springfield; after both paragraphs, lastname is Simpson again.
```html
<html>
<!-- Big test combining every feature -->
<head>
<title> The Simpsons Family </title>
</head>
<p> The members of the <b> Simpson </b> family are:
<ul>
<li> Homer Simpson </li>
<li> Marge Simpson </li>
<li> Bart Simpson </li>
<li> Ned Flanders </li>
</ul>
<br>
</p>
<p> They all live in Springfield . <br>
Learn more at <a href="www.thesimpsons.com">www.thesimpsons.com</a> <br>
<ul>
<li> <b> Springfield </b> is in the family Simpson saga </li>
</ul>
</p>
The end of the Simpson document. </html>
```

### v12_sibling_scopes.lol
Variables do not leak between sibling blocks. Shows "I feel happy" then "I feel sad".
```html
<html>
<p> I feel happy </p>
<p> I feel sad </p>
</html>
```

### v13_touching_annotations.lol
No whitespace needed before a #. Shows "Line one", "Line two", then "**tight** next" on separate lines.
```html
<html>
<p> Line one <br>
Line two <br>
<b> tight </b> next </p>
</html>
```

### v14_punctuation_and_addresses.lol
Needs the allowed-characters fix ('=' and '/'). Text is printed exactly as written. Link goes to www.example.com/page?x=5.
```html
<html>
<p> Hello, world. "Quoted": is it 100% true? Yes! 50% = half/half. <br>
<a href="www.example.com/page?x=5">www.example.com/page?x=5</a> </p>
</html>
```

## Invalid files (must fail)

| File | Expected console output | Why |
|---|---|---|
| e01_missing_hai.lol | `Syntax error at line 1: expected '#HAI' but found 'Hello'` | File must start with #HAI. |
| e02_missing_kbye.lol | `Syntax error at line 4: expected '#KBYE' but found end of file` | File never ends with #KBYE. |
| e03_text_after_kbye.lol | `Syntax error at line 4: found 'Extra' after #KBYE` | Nothing may follow #KBYE. |
| e04_unknown_annotation.lol | `Lexical error at line 4: unknown annotation '#HEY'` | #HEY is not an annotation. Lexical errors are reported before any syntax errors. |
| e05_illegal_character.lol | `Lexical error at line 2: illegal character '$'` | $ is not an allowed text character. |
| e06_undefined_variable.lol | `Static semantic error at line 3: variable 'name' used before it was defined` | Variable never defined. |
| e07_variable_out_of_scope.lol | `Static semantic error at line 6: variable 'secret' used before it was defined` | Variable only lives inside the paragraph that defined it. |
| e08_use_before_define.lol | `Static semantic error at line 2: variable 'late' used before it was defined` | Used before any definition (a later definition in an inner block doesn't count). |
| e09_nested_paragraph.lol | `Syntax error at line 4: a paragraph cannot be inside another paragraph` | Paragraphs cannot nest. |
| e10_list_without_item.lol | `Syntax error at line 3: a list must start with #GIMMEH ITEM, found 'Not'` | A list must begin with an item. |
| e11_head_after_body_text.lol | `Syntax error at line 3: expected '#KBYE' but found '#MAEK'` | Head must come right after #HAI (only comments may precede it). |
| e12_define_not_first.lol | `Syntax error at line 4: expected '#MKAY' but found '#IHAZ'` | #IHAZ must be the first thing in its block. |
| e13_unclosed_bold.lol | `Syntax error at line 4: expected '#OIC' but found '#MKAY'` | Bold is missing its #OIC. |
| e14_annotation_in_title.lol | `Syntax error at line 3: expected '#OIC' but found '#GIMMEH'` | Titles may only contain plain text. |
| e15_variable_without_value.lol | `Syntax error at line 2: variable 'empty' needs a value after #ITIZ` | Definition has no value. |
| e16_second_hai.lol | `Syntax error at line 3: expected '#KBYE' but found '#HAI'` | #HAI cannot be repeated. |
| e17_nested_list.lol | `Syntax error at line 4: expected '#OIC' but found '#MAEK'` | List items cannot contain lists. |
| e18_newline_in_head.lol | `Syntax error at line 3: expected '#GIMMEH' but found '#NEWLINE'` | Only a title is allowed inside the head. |
| e19_unknown_gimmeh_keyword.lol | `Syntax error at line 3: expected '#MKAY' but found '#GIMMEH'` | #GIMMEH ITALICS does not exist. |
| e20_empty_file.lol | `Syntax error at line 1: expected '#HAI' but found end of file` | Empty file (line is 1 only after the error_line fallback fix). |
| e21_wrong_extension.txt | `Usage error: the input file must have a .lol extension` | Wrong extension. Run: lolcompiler e21_wrong_extension.txt |
| e22_unclosed_paragraph.lol | `Syntax error at line 4: expected '#MKAY' but found '#KBYE'` | Paragraph never closed. |
| e23_link_missing_address.lol | `Syntax error at line 2: expected an address after LINX but found '#OIC'` | Link has no address. |
