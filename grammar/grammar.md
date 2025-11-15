# Backus-Naur Form (BNF) Grammar for the Language

This mardown document describes the grammar of the language in Backus-Naur Form (BNF) notation. The grammar is derived from the recursive descent parser implementation.

## Terminals

The following are terminal symbols (tokens):

- Keywords: `fn`, `int`, `float`, `bool`, `string`, `return`, `if`, `else`, `for`, `while`, `true`, `false`
- Identifiers: sequences of letters, digits, and underscores starting with a letter or underscore
- Literals:
  - Integer literals: sequences of digits
  - Float literals: sequences of digits with a decimal point
  - String literals: sequences of characters enclosed in double quotes
- Operators:
  - Assignment: `=`
  - Equality: `==`, `!=`
  - Comparison: `<`, `>`, `<=`, `>=`
  - Arithmetic: `+`, `-`, `*`, `/`, `%`
  - Unary: `-`, `!`
- Punctuation: `(`, `)`, `{`, `}`, `,`, `;`, `:`

## Non-Terminals and Productions

### Program
```
<program> ::= <declaration>*
```

### Declaration
```
<declaration> ::= <function-declaration>
                | <global-variable-declaration>
```

### Function Declaration
```
<function-declaration> ::= "fn" IDENTIFIER "(" <parameter-list> ")" [ ":" <type> ] "{" <statement>* "}"
```

### Parameter List
```
<parameter-list> ::= <parameter> ( "," <parameter> )*
                  | ε
<parameter> ::= <type> IDENTIFIER
```

### Global Variable Declaration
```
<global-variable-declaration> ::= <type> IDENTIFIER [ "=" <expression> ] ";"
```

### Statement
```
<statement> ::= <return-statement>
              | <if-statement>
              | <while-statement>
              | <for-statement>
              | <block-statement>
              | <expression-statement>
              | <declaration-statement>
```

### Return Statement
```
<return-statement> ::= "return" [ <expression> ] ";"
```

### If Statement
```
<if-statement> ::= "if" "(" <expression> ")" <statement> [ "else" <statement> ]
```

### While Statement
```
<while-statement> ::= "while" "(" <expression> ")" <statement>
```

### For Statement
```
<for-statement> ::= "for" "(" <for-init> <expression>? ";" <expression>? ")" <statement>
<for-init> ::= <declaration-statement>
             | <expression-statement>
             | ";"
```

### Block Statement
```
<block-statement> ::= "{" <statement>* "}"
```

### Expression Statement
```
<expression-statement> ::= <expression> ";"
```

### Declaration Statement
```
<declaration-statement> ::= <type> IDENTIFIER [ "=" <expression> ] ";"
```

### Expression
```
<expression> ::= <assignment>
```

### Assignment
```
<assignment> ::= <equality> ( "=" <assignment> )?
```

### Equality
```
<equality> ::= <comparison> ( ( "==" | "!=" ) <comparison> )*
```

### Comparison
```
<comparison> ::= <term> ( ( "<" | ">" | "<=" | ">=" ) <term> )*
```

### Term
```
<term> ::= <factor> ( ( "+" | "-" ) <factor> )*
```

### Factor
```
<factor> ::= <unary> ( ( "*" | "/" | "%" ) <unary> )*
```

### Unary
```
<unary> ::= ( "-" | "!" ) <unary>
          | <primary>
```

### Primary
```
<primary> ::= IDENTIFIER [ "(" <argument-list> ")" ]
            | INTEGER_LITERAL
            | FLOAT_LITERAL
            | STRING_LITERAL
            | "true"
            | "false"
            | "(" <expression> ")"
```

### Argument List
```
<argument-list> ::= <expression> ( "," <expression> )*
                 | ε
```

### Type
```
<type> ::= "int" | "float" | "bool" | "string"
```

## Side Note

- `ε` denotes the empty string.
- `[ ... ]` denotes optional elements.
- `( ... )*` denotes zero or more repetitions.
- `|` denotes alternatives.
- Assignment is right-associative and only allowed on identifiers.
- Default values are assigned to variables if no initializer is provided in declarations.

