[
  "break"
  "case"
  "const"
  "continue"
  "default"
  "do"
  "else"
  "enum"
  "extern"
  "for"
  "goto"
  "if"
  "inline"
  "return"
  "sizeof"
  "static"
  "struct"
  "switch"
  "typedef"
  "union"
  "volatile"
  "while"
] @keyword

[
  "#define"
  "#elif"
  "#else"
  "#endif"
  "#if"
  "#ifdef"
  "#ifndef"
  "#include"
  (preproc_directive)
] @keyword

[
  "="
  "+="
  "-="
  "*="
  "/="
  "%="
  "&="
  "|="
  "^="
  "<<="
  ">>="
  "++"
  "--"
  "+"
  "-"
  "*"
  "/"
  "%"
  "~"
  "&"
  "|"
  "^"
  "<<"
  ">>"
  "!"
  "&&"
  "||"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "->"
  "?"
  ":"
] @operator

[
  "."
  ";"
  ","
] @punctuation.delimiter

[
  "{"
  "}"
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  (string_literal)
  (system_lib_string)
  (char_literal)
] @string

(comment) @comment

(number_literal) @number

[
  (true)
  (false)
  (null)
] @constant

(identifier) @variable

((identifier) @constant
 (#match? @constant "^_*[A-Z][A-Z\\d_]*$"))

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))
(function_declarator
  declarator: (identifier) @function)
(preproc_function_def
  name: (identifier) @function.special)

(field_identifier) @property
(statement_identifier) @label

[
  (type_identifier)
  (primitive_type)
  (sized_type_specifier)
] @type

; Functions

(call_expression
  function: (qualified_identifier
    name: (identifier) @function))

(template_function
  name: (identifier) @function)

(template_method
  name: (field_identifier) @function)

(template_function
  name: (identifier) @function)

(function_declarator
  declarator: (qualified_identifier
    name: (identifier) @function))

(function_declarator
  declarator: (field_identifier) @function)

; Types

((namespace_identifier) @type
 (#match? @type "^[A-Z]"))

(auto) @type

; Constants

(this) @variable.builtin
(null "nullptr" @constant)

; Keywords

[
 "catch"
 "class"
 "co_await"
 "co_return"
 "co_yield"
 "constexpr"
 "constinit"
 "consteval"
 "delete"
 "explicit"
 "final"
 "friend"
 "mutable"
 "namespace"
 "noexcept"
 "new"
 "override"
 "private"
 "protected"
 "public"
 "template"
 "throw"
 "try"
 "typename"
 "using"
 "concept"
 "requires"
 "virtual"
] @keyword

; Strings

(raw_string_literal) @string

