import type { SyntaxNode } from "tree-sitter";

export function bashCorrections(node: SyntaxNode) {
  if (node.type === "word" && node.text.startsWith("-")) return "constant";
  if (node.type === "word" && node.parent?.type === "command_name") return;
  if (node.type === "word") return "";
  if (node.text === ">&" && node.parent?.type === "file_redirect") return "";
}

export function cCorrections(node: SyntaxNode, captures: string[]) {
  if (captures.includes("constant")) return "constant";
  if (node.type === "preproc_arg") {
    if (!isNaN(Number(node.text))) return "number";
    return "";
  }
  if (node.type === "defined" && node.parent?.type === "preproc_defined")
    return "";
}

export function cSharpCorrections(node: SyntaxNode) {
  if (
    node.type === "identifier" &&
    node.parent?.type.includes("member_declaration")
  )
    return "property";

  if (
    (node.parent?.type.includes("declaration") ||
      node.parent?.type.includes("type") ||
      node.parent?.type === "base_list" ||
      node.parent?.type === "generic_name") &&
    node.type === "identifier"
  )
    return "type";

  if (node.parent?.type === "attribute") return "attribute";
}

export function cppCorrections(node: SyntaxNode, captures: string[]) {
  const c = cCorrections(node, captures);
  if (c !== undefined) return c;

  if (node.type === "namespace_identifier") return "type";
  if (node.text === "::") return "";
  if (
    node.type === "identifier" &&
    node.parent &&
    ["qualified_identifier", "destructor_name"].includes(node.parent.type)
  )
    return "function";
  if (node.type === "operator" && node.parent?.type === "operator_name")
    return "function";
}

export function cssCorrections(node: SyntaxNode) {
  if (
    node.type === "tag_name" &&
    node.parent?.type === "pseudo_element_selector"
  )
    return "attribute";

  if (node.type === "keyframes_name") return "variable";
  if (node.type === "plain_value") return "";
}

export function goCorrections(node: SyntaxNode) {
  if (node.type === "package_identifier") return "";
  if (
    node.type === "identifier" &&
    node.parent &&
    [
      "var_spec",
      "range_clause",
      "parameter_declaration",
      "argument_list",
      "expression_switch_statement",
      "expression_list",
      "type_conversion_expression",
      "index_expression",
      "binary_expression",
      "unary_expression",
    ].includes(node.parent.type)
  )
    return "variable";

  if (node.type === "field_identifier") {
    if (node.parent?.parent?.type === "call_expression") return "function";
    return "property";
  }
}

export function htmlCorrections(node: SyntaxNode) {
  if (["text", "raw_text"].includes(node.type)) return "";
  if (node.type === "doctype") return "constant";
}

export function javaCorrections(node: SyntaxNode) {
  if (node.type === "identifier") {
    if (node.parent?.type === "class_declaration") return "type";
    if (node.parent?.type === "enum_declaration") return "enum";
    if (node.parent?.type === "constructor_declaration") return "constructor";
    if (node.parent?.type === "method_declaration") return "function";
    if (node.parent?.type === "field_access") return "property";
    if (node.parent?.type.includes("constant")) return "constant";
  }
}

export function jsCorrections(node: SyntaxNode, captures: string[]) {
  if (node.type === "?" && node.parent?.type === "ternary_expression")
    return "operator";

  if (node.type === "identifier" && node.parent?.type === "new_expression")
    return "constructor";

  if (node.type === "identifier" && captures.includes("type")) return "type";

  if (
    node.type === "property_identifier" &&
    node.parent?.type === "method_definition"
  )
    return "function";

  if (
    node.type === "property_identifier" &&
    node.parent?.type === "member_expression" &&
    node.parent.parent?.type === "call_expression"
  )
    return "function";
}

export function ocamlCorrections(node: SyntaxNode) {
  if (node.type === "method_name") return "function";
}

export function phpCorrections(node: SyntaxNode, captures: string[]) {
  if (
    node.type === "name" &&
    node.parent &&
    ["namespace_name", "object_creation_expression"].includes(node.parent.type)
  )
    return "constructor";

  if (
    node.type === "name" &&
    node.parent &&
    [
      "trait_declaration",
      "interface_declaration",
      "class_declaration",
      "base_clause",
      "class_interface_clause",
      "use_declaration",
      "use_instead_of_clause",
    ].includes(node.parent.type)
  )
    return "type";

  if (
    node.type === "name" &&
    node.parent &&
    ["enum_declaration"].includes(node.parent.type)
  )
    return "enum";

  if (
    node.type === "name" &&
    node.parent &&
    [
      "method_declaration",
      "function_definition",
      "function_call_expression",
    ].includes(node.parent.type)
  )
    return "function";

  if (
    node.type === "name" &&
    node.parent &&
    ["namespace_use_clause", "qualified_name"].includes(node.parent.type)
  ) {
    if (captures.length > 0) return captures[0];
    return "";
  }
}

export function pythonCorrections(node: SyntaxNode, captures: string[]) {
  if (
    node.text === "self" &&
    node.parent &&
    ["attribute"].includes(node.parent.type)
  )
    return "variable";

  if (
    node.type === "identifier" &&
    node.parent &&
    [
      "assignment",
      "argument_list",
      "comparison_operator",
      "binary_operator",
      "parameters",
      "list_splat_pattern",
      "keyword_argument",
      "dictionary_splat_pattern",
      "typed_parameter",
      "typed_default_parameter",
      "default_parameter",
      "subscript",
      "delete_statement",
      "tuple",
      "while_statement",
      "not_operator",
      "augmented_assignment",
      "boolean_operator",
      "global_statement",
      "nonlocal_statement",
      "lambda",
      "if_statement",
      "for_statement",
      "conditional_expression",
      "return_statement",
      "for_in_clause",
      "list_comprehension",
      "list",
      "tuple_pattern",
      "generator_expression",
      "if_clause",
    ].includes(node.parent.type)
  ) {
    if (captures.length > 0) return captures[0];
    return "variable";
  }

  if (
    node.type === "identifier" &&
    node.parent?.type === "attribute" &&
    node.parent.parent?.type === "call"
  )
    return "function";

  if (node.type === "identifier" && node.parent?.type === "decorator")
    return "function";

  if (node.type === "identifier" && node.parent?.type === "class_definition")
    return "type";
}

export function regexCorrections(node: SyntaxNode) {
  if (node.type === "decimal_digits") return "number";
  if (node.type === "-" && node.parent?.type === "class_range")
    return "operator";
}

export function rubyCorrections(node: SyntaxNode, captures: string[]) {
  if (node.type === "identifier" && node.parent?.type === "method")
    return "function.method";

  if (node.type === "identifier" && captures.includes("keyword"))
    return "keyword";

  if (
    node.type === "constant" &&
    node.parent &&
    ["assignment", "program"].includes(node.parent.type)
  )
    return "constant";

  if (node.type === "identifier" && captures.includes("constant.builtin"))
    return "constant.builtin";

  if (
    node.type === "identifier" &&
    node.parent?.type === "body_statement" &&
    captures.includes("function.method")
  )
    return "function.method";
}

export function rustCorrections(node: SyntaxNode, captures: string[]) {
  if (node.type === "identifier" && node.parent?.type === "attribute")
    return "attribute";

  if (
    node.type === "identifier" &&
    node.parent?.type === "token_tree" &&
    captures.length === 0
  )
    return "variable";

  if (node.type === "identifier" && node.parent?.type === "unary_expression")
    return "";

  if (node.type === "/" && node.parent?.type === "outer_doc_comment_marker")
    return "comment";

  if (node.type === "identifier" && node.parent?.type === "macro_invocation")
    return "function";

  if (
    node.type === "identifier" &&
    node.parent &&
    [
      "type_cast_expression",
      "arguments",
      "closure_parameters",
      "index_expression",
      "let_declaration",
      "for_expression",
      "ref_pattern",
      "tuple_expression",
      "tuple_pattern",
      "match_arm",
      "mod_item",
      "use_declaration",
    ].includes(node.parent.type) &&
    captures.length === 0
  )
    return "variable";

  if (
    node.type === "identifier" &&
    node.parent?.type === "scoped_identifier" &&
    node.parent.parent?.type === "call_expression"
  ) {
    if (captures.length > 0) return captures[0];
    return "function";
  }

  if (
    node.type === "field_identifier" &&
    node.parent?.type === "field_expression" &&
    node.parent.parent?.type === "call_expression"
  )
    return "function";

  if (
    node.type === "identifier" &&
    node.parent?.type === "call_expression" &&
    captures.includes("type")
  )
    return "type";

  if (
    node.type === "identifier" &&
    node.parent &&
    ["const_item", "field_initializer"].includes(node.parent.type)
  ) {
    if (captures.includes("constant")) return "constant";
    return "variable";
  }

  if (node.type === "_" && node.text === node.type) return "";

  if (node.type === "shorthand_field_identifier") return "variable";
}

export function scalaCorrections(node: SyntaxNode, captures: string[]) {
  if (
    node.type === "identifier" &&
    node.parent &&
    ["import_declaration"].includes(node.parent.type) &&
    captures.length === 0
  ) {
    return "namespace";
  }

  if (
    node.type === "identifier" &&
    node.parent &&
    [
      "val_definition",
      "var_definition",
      "lambda_expression",
      "type_parameters",
      "interpolation",
      "colon_argument",
      "infix_expression",
      "arguments",
      "ascription_expression",
      "export_declaration",
      "field_expression",
    ].includes(node.parent.type) &&
    captures.length === 0
  ) {
    return "variable";
  }

  if (node.parent && ["comment", "block_comment"].includes(node.parent.type))
    return "comment";
}

export function tsCorrections(node: SyntaxNode, captures: string[]) {
  const c = jsCorrections(node, captures);
  if (c !== undefined) return c;
}

export function tsxCorrections(node: SyntaxNode, captures: string[]) {
  const c = tsCorrections(node, captures);
  if (c !== undefined) return c;

  if (
    node.type === "identifier" &&
    node.parent &&
    [
      "jsx_opening_element",
      "jsx_closing_element",
      "jsx_self_closing_element",
    ].includes(node.parent.type)
  )
    return "tag";

  if (
    node.type === "property_identifier" &&
    node.parent?.type === "jsx_attribute"
  )
    return "attribute";

  if (node.type === "jsx_text") return "text";
}
