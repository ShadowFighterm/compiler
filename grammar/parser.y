%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* yylex must be provided by the lexer (Rust or C bridge). */
int yylex(void);
void yyerror(const char *s);

/* Helper: duplicate string safely for semantic values */
static char* dupstr(const char *s) {
    if (!s) return NULL;
    size_t n = strlen(s) + 1;
    char *r = (char*)malloc(n);
    if (!r) { fprintf(stderr,"out of memory\n"); exit(1); }
    memcpy(r, s, n);
    return r;
}
%}


/* semantic value union */
%union {
    long long intval;   /* <- use 64-bit to match TokenBridge.intval / Rust i64 */
    double floatval;
    char* str;   /* for identifiers and string literals and types */
    int boolval; /* 0/1 */
}

/* Token declarations (with corresponding types from %union where applicable) */
%token T_FUNCTION
%token T_INT T_FLOAT T_BOOL T_STRING
%token T_RETURN T_IF T_ELSE T_FOR T_WHILE

%token <str> T_IDENTIFIER
%token <intval> T_INTLIT
%token <floatval> T_FLOATLIT
%token <str> T_STRINGLIT
%token <boolval> T_BOOLLIT

%token T_PARENL T_PARENR T_BRACEL T_BRACER T_COMMA T_SEMICOLON T_COLON

%token T_ASSIGNOP    /* = */
%token T_EQUALSOP    /* == */
%token T_NEQ         /* != */
%token T_LT T_GT T_LTE T_GTE

%token T_PLUS T_MINUS T_STAR T_SLASH T_PERCENT
%token T_NOT
%token T_ANDAND T_OROR
%token T_LSHIFT T_RSHIFT
%token T_AMP T_PIPE T_CARET T_TILDE

/* precedence & associativity */
/* from low to high precedence (lower listed earlier => lower prec) */
%left T_OROR
%left T_ANDAND
%left T_EQUALSOP T_NEQ
%left T_LT T_GT T_LTE T_GTE
%left T_PLUS T_MINUS
%left T_STAR T_SLASH T_PERCENT
%right T_NOT

/* resolve dangling-else explicitly */
%nonassoc LOWER_THAN_ELSE

/* removed: %nonassoc T_PARENL */
%right T_ASSIGNOP

/* start symbol */
%start program

%%
/* --------------------------
   Grammar rules follow
   -------------------------- */

/* Program: zero or more declarations */
program:
      /* empty */
    | program declaration
    ;

/* Declaration: function or global variable */
declaration:
      function_declaration
    | global_variable_declaration
    ;

/* Function declaration:
   "fn" IDENT "(" parameter-list? ")" [ ":" type ] "{" statement* "}" */
function_declaration:
      T_FUNCTION T_IDENTIFIER T_PARENL parameter_list_opt T_PARENR function_return_opt T_BRACEL statement_list_opt T_BRACER
    {
        /* debug print - replace with AST construction later */
        printf("FunctionDecl: name=%s\n", $2 ? $2 : "(null)");
        if ($2) free($2);
    }
   ;

/* optional parameter list */
parameter_list_opt:
      /* empty */          { /* no params */ }
    | parameter_list        { /* provided */ }
   ;

/* parameter list: comma separated parameters */
parameter_list:
      parameter
    | parameter_list T_COMMA parameter
   ;

/* parameter: <type> IDENT */
parameter:
      type T_IDENTIFIER
    {
        /* $1 is type token text if we choose to capture; $2 is name */
        printf("  Param: name=%s\n", $2 ? $2 : "(null)");
        if ($2) free($2);
    }
   ;

/* optional function return type */
function_return_opt:
      /* empty */              { /* no return type */ }
    | T_COLON type            { /* has return type */ }
    ;

/* Global variable declaration:
   <type> IDENT [ "=" <expression> ] ";" */
global_variable_declaration:
      type T_IDENTIFIER T_SEMICOLON
    {
        printf("GlobalVarDecl: name=%s (no initializer)\n", $2 ? $2 : "(null)");
        if ($2) free($2);
    }
    | type T_IDENTIFIER T_ASSIGNOP expression T_SEMICOLON
    {
        printf("GlobalVarDecl: name=%s (with initializer)\n", $2 ? $2 : "(null)");
        if ($2) free($2);
    }
    ;

/* Statement: multiple alternatives as in grammar.md */
statement:
      return_statement
    | if_statement
    | while_statement
    | for_statement
    | block_statement
    | expression_statement
    | declaration_statement
   ;

/* Return statement: "return" [ expression ] ";" */
return_statement:
      T_RETURN T_SEMICOLON
    {
        printf("ReturnStmt: (void)\n");
    }
    | T_RETURN expression T_SEMICOLON
    {
        printf("ReturnStmt: (with expr)\n");
    }
   ;

/* If statement: "if" "(" expression ")" statement [ "else" statement ] */
if_statement:
      T_IF T_PARENL expression T_PARENR statement %prec LOWER_THAN_ELSE
    {
        printf("IfStmt: without else\n");
    }
    | T_IF T_PARENL expression T_PARENR statement T_ELSE statement
    {
        printf("IfStmt: with else\n");
    }
   ;

/* While: "while" "(" expression ")" statement */
while_statement:
      T_WHILE T_PARENL expression T_PARENR statement
    {
        printf("WhileStmt\n");
    }
   ;

/* For: "for" "(" for-init expression? ";" expression? ")" statement
   for-init: declaration-stmt | expression-stmt | ";" */
for_statement:
      T_FOR T_PARENL for_init expression_opt T_SEMICOLON expression_opt T_PARENR statement
    {
        printf("ForStmt\n");
    }
   ;

/* for-init */
for_init:
      declaration_statement
    | expression_statement
    | T_SEMICOLON  /* stand-alone semicolon */
   ;

/* optional expression (used in for header) */
expression_opt:
      /* empty */  { /* no expr */ }
    | expression   { /* $1 used if needed */ }
   ;

/* Block statement: "{" statement* "}" */
block_statement:
      T_BRACEL statement_list_opt T_BRACER
    {
        printf("BlockStmt\n");
    }
    ;

statement_list:
    statement
  | statement_list statement
;

statement_list_opt:
    statement_list
  | /* empty */
;

/* Expression statement: expression ";" */
expression_statement:
      expression T_SEMICOLON
    {
        printf("ExprStmt\n");
    }
   ;

/* Declaration statement: like global var but local */
declaration_statement:
      type T_IDENTIFIER T_SEMICOLON
    {
        printf("LocalVarDecl: name=%s (no init)\n", $2 ? $2 : "(null)");
        if ($2) free($2);
    }
    | type T_IDENTIFIER T_ASSIGNOP expression T_SEMICOLON
    {
        printf("LocalVarDecl: name=%s (with init)\n", $2 ? $2 : "(null)");
        if ($2) free($2);
    }
    ;

/* Expression starts at assignment (right-associative) */
expression:
      assignment
    ;

/* Assignment: equality ( '=' assignment )? ; right-associative */
assignment:
      equality
    | equality T_ASSIGNOP assignment
    {
        /* assignment only allowed to identifiers semantically; we do not enforce here.
           Replace with AST node creation and semantic checks later. */
        printf("AssignmentExpr\n");
    }
   ;

/* Equality: comparison ( ( "==" | "!=" ) comparison )* */
equality:
      comparison
    | equality T_EQUALSOP comparison
    {
        printf("BinaryExpr: ==\n");
    }
    | equality T_NEQ comparison
    {
        printf("BinaryExpr: !=\n");
    }
   ;

/* Comparison: term ( ( "<" | ">" | "<=" | ">=" ) term )* */
comparison:
      term
    | comparison T_LT term
    {
        printf("BinaryExpr: <\n");
    }
    | comparison T_GT term
    {
        printf("BinaryExpr: >\n");
    }
    | comparison T_LTE term
    {
        printf("BinaryExpr: <=\n");
    }
    | comparison T_GTE term
    {
        printf("BinaryExpr: >=\n");
    }
   ;

/* Term: factor ( ( "+" | "-" ) factor )* */
term:
      factor
    | term T_PLUS factor
    {
        printf("BinaryExpr: +\n");
    }
    | term T_MINUS factor
    {
        printf("BinaryExpr: -\n");
    }
   ;

/* Factor: unary ( ( "*" | "/" | "%" ) unary )* */
factor:
      unary
    | factor T_STAR unary
    {
        printf("BinaryExpr: *\n");
    }
    | factor T_SLASH unary
    {
        printf("BinaryExpr: /\n");
    }
    | factor T_PERCENT unary
    {
        printf("BinaryExpr: %%\n");
    }
   ;

/* Unary: ( "-" | "!" ) unary | primary */
unary:
      T_MINUS unary
    {
        printf("UnaryExpr: -\n");
    }
    | T_NOT unary
    {
        printf("UnaryExpr: !\n");
    }
    | primary
    ;

/* Primary: IDENT [ "(" argument-list? ")" ] | INT | FLOAT | STRING | true | false | "(" expression ")" */
primary:
      T_IDENTIFIER
    {
        printf("Primary: IDENT (%s)\n", $1 ? $1 : "(null)");
        if ($1) free($1);
    }
    | T_IDENTIFIER T_PARENL argument_list_opt T_PARENR
    {
        printf("Primary: Call expr on %s\n", $1 ? $1 : "(null)");
        if ($1) free($1);
    }
    | T_INTLIT
    {
        printf("Primary: INTLIT (%lld)\n", $1);
    }
    | T_FLOATLIT
    {
        printf("Primary: FLOATLIT (%f)\n", $1);
    }
    | T_STRINGLIT
    {
        printf("Primary: STRINGLIT (%s)\n", $1 ? $1 : "(null)");
        if ($1) free($1);
    }
    | T_BOOLLIT
    {
        printf("Primary: BOOLLIT (%s)\n", ($1 ? "true" : "false"));
    }
    | T_PARENL expression T_PARENR
    {
        printf("Primary: grouped expression\n");
    }
   ;

/* argument-list: expression ("," expression)* or empty */
argument_list_opt:
      /* empty */    { /* no args */ }
    | argument_list
    ;

argument_list:
      expression
    | argument_list T_COMMA expression
   ;

/* type: "int" | "float" | "bool" | "string" */
type:
      T_INT    { /* type int */ printf("Type: int\n"); }
    | T_FLOAT  { printf("Type: float\n"); }
    | T_BOOL   { printf("Type: bool\n"); }
    | T_STRING { printf("Type: string\n"); }
    ;

%%

/* --------------------------
   Epilogue: error routine and main
   -------------------------- */

void yyerror(const char *s) {
    fprintf(stderr, "Parse error: %s\n", s);
}

/* main: initialize lexer from input file */
extern void lexer_init_from_file(const char* path);

int main(int argc, char **argv) {
    const char* path = (argc > 1) ? argv[1] : "sample.src";
    lexer_init_from_file(path); /* initialize Rust lexer stream */
    printf("Starting parse...\n");
    if (yyparse() == 0) {
        printf("Parsing finished successfully.\n");
    } else {
        printf("Parsing failed.\n");
    }
    return 0;
}
