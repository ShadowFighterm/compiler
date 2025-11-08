#include "parser.tab.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Rust FFI (implemented in Rust: src/ffi.rs) */
extern void* lexer_new_from_c(const char* src);
extern void lexer_free(void* lex);
extern int lexer_next_bridge(void* lex, void* out);

/* TokenBridge layout must match the Rust repr(C) */
typedef struct {
    int kind;
    long long intval;
    double floatval;
    char* strptr;
    int boolv;
} TokenBridge;

static void* g_lexer = NULL;

/* initialize lexer from a file (called from parser main) */
void lexer_init_from_file(const char* path) {
    if (!path) return;
    FILE* f = fopen(path, "rb");
    if (!f) return;
    if (fseek(f, 0, SEEK_END) != 0) { fclose(f); return; }
    long sz = ftell(f);
    if (sz < 0) { fclose(f); return; }
    rewind(f);
    char* buf = (char*)malloc((size_t)sz + 1);
    if (!buf) { fclose(f); return; }
    size_t read = fread(buf, 1, (size_t)sz, f);
    buf[read] = '\0';
    fclose(f);
    g_lexer = lexer_new_from_c(buf);
    free(buf);
}

/* yylex: bridge between Bison parser and Rust lexer */
int yylex(void) {
    if (!g_lexer) return 0; /* EOF */

    TokenBridge tb;
    if (!lexer_next_bridge(g_lexer, &tb)) {
        return 0; /* EOF */
    }

    switch (tb.kind) {
        /* literals / identifiers */
        case 1: /* IDENT */
            yylval.str = tb.strptr;
            return T_IDENTIFIER;
        case 2: /* INTLIT */
            yylval.intval = tb.intval;
            return T_INTLIT;
        case 3: /* FLOATLIT */
            yylval.floatval = tb.floatval;
            return T_FLOATLIT;
        case 4: /* STRINGLIT */
            yylval.str = tb.strptr;
            return T_STRINGLIT;
        case 5: /* BOOLLIT */
            yylval.boolval = tb.boolv;
            return T_BOOLLIT;

        /* keywords */
        case 10: return T_FUNCTION;
        case 11: return T_RETURN;
        case 12: return T_IF;
        case 13: return T_ELSE;
        case 14: return T_FOR;
        case 15: return T_WHILE;
        case 20: return T_INT;
        case 21: return T_FLOAT;
        case 22: return T_BOOL;
        case 23: return T_STRING;

        /* punctuation */
        case 30: return T_PARENL;
        case 31: return T_PARENR;
        case 32: return T_BRACEL;
        case 33: return T_BRACER;
        case 40: return T_COMMA;
        case 41: return T_SEMICOLON;
        case 42: return T_COLON;

        /* operators */
        case 50: return T_ASSIGNOP;
        case 51: return T_EQUALSOP;
        case 52: return T_NEQ;
        case 60: return T_LT;
        case 61: return T_GT;
        case 62: return T_LTE;
        case 63: return T_GTE;
        case 70: return T_PLUS;
        case 71: return T_MINUS;
        case 72: return T_STAR;
        case 73: return T_SLASH;
        case 74: return T_PERCENT;
        case 80: return T_NOT;
        case 81: return T_ANDAND;
        case 82: return T_OROR;
        case 90: return T_LSHIFT;
        case 91: return T_RSHIFT;
        case 92: return T_AMP;
        case 93: return T_PIPE;
        case 94: return T_CARET;
        case 95: return T_TILDE;

        default:
            /* unknown token: treat as EOF (or return an error token if desired) */
            return 0;
    }
}

/* optional cleanup helper (call from main after parse if desired) */
void lexer_destroy(void) {
    if (!g_lexer) return;
    lexer_free(g_lexer);
    g_lexer = NULL;
}