/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

#ifndef YY_YY_PARSER_TAB_H_INCLUDED
# define YY_YY_PARSER_TAB_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    YYEMPTY = -2,
    YYEOF = 0,                     /* "end of file"  */
    YYerror = 256,                 /* error  */
    YYUNDEF = 257,                 /* "invalid token"  */
    T_FUNCTION = 258,              /* T_FUNCTION  */
    T_INT = 259,                   /* T_INT  */
    T_FLOAT = 260,                 /* T_FLOAT  */
    T_BOOL = 261,                  /* T_BOOL  */
    T_STRING = 262,                /* T_STRING  */
    T_RETURN = 263,                /* T_RETURN  */
    T_IF = 264,                    /* T_IF  */
    T_ELSE = 265,                  /* T_ELSE  */
    T_FOR = 266,                   /* T_FOR  */
    T_WHILE = 267,                 /* T_WHILE  */
    T_IDENTIFIER = 268,            /* T_IDENTIFIER  */
    T_INTLIT = 269,                /* T_INTLIT  */
    T_FLOATLIT = 270,              /* T_FLOATLIT  */
    T_STRINGLIT = 271,             /* T_STRINGLIT  */
    T_BOOLLIT = 272,               /* T_BOOLLIT  */
    T_PARENL = 273,                /* T_PARENL  */
    T_PARENR = 274,                /* T_PARENR  */
    T_BRACEL = 275,                /* T_BRACEL  */
    T_BRACER = 276,                /* T_BRACER  */
    T_COMMA = 277,                 /* T_COMMA  */
    T_SEMICOLON = 278,             /* T_SEMICOLON  */
    T_COLON = 279,                 /* T_COLON  */
    T_ASSIGNOP = 280,              /* T_ASSIGNOP  */
    T_EQUALSOP = 281,              /* T_EQUALSOP  */
    T_NEQ = 282,                   /* T_NEQ  */
    T_LT = 283,                    /* T_LT  */
    T_GT = 284,                    /* T_GT  */
    T_LTE = 285,                   /* T_LTE  */
    T_GTE = 286,                   /* T_GTE  */
    T_PLUS = 287,                  /* T_PLUS  */
    T_MINUS = 288,                 /* T_MINUS  */
    T_STAR = 289,                  /* T_STAR  */
    T_SLASH = 290,                 /* T_SLASH  */
    T_PERCENT = 291,               /* T_PERCENT  */
    T_NOT = 292,                   /* T_NOT  */
    T_ANDAND = 293,                /* T_ANDAND  */
    T_OROR = 294,                  /* T_OROR  */
    T_LSHIFT = 295,                /* T_LSHIFT  */
    T_RSHIFT = 296,                /* T_RSHIFT  */
    T_AMP = 297,                   /* T_AMP  */
    T_PIPE = 298,                  /* T_PIPE  */
    T_CARET = 299,                 /* T_CARET  */
    T_TILDE = 300,                 /* T_TILDE  */
    LOWER_THAN_ELSE = 301          /* LOWER_THAN_ELSE  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 23 "parser.y"

    long long intval;   /* <- use 64-bit to match TokenBridge.intval / Rust i64 */
    double floatval;
    char* str;   /* for identifiers and string literals and types */
    int boolval; /* 0/1 */

#line 117 "parser.tab.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;


int yyparse (void);


#endif /* !YY_YY_PARSER_TAB_H_INCLUDED  */
