// ANTLR 4.5.1 grammar for Mth
//
// Should work on http://lab.antlr.org/

grammar Mth;

module
    : topLevel* EOF
    ;

topLevel
    : typeDecl
    | map ';'
    | expr ';'
    ;

typeDecl
    : IDENT '::' typeExpr
    ;

map
    : IDENT paramList '->' expr
    ;

paramList
    : IDENT+
    ;

expr
    : literal
    | IDENT
    | functionCall
    | expr op=Operator expr
    ;

functionCall
    : IDENT '(' (expr (',' expr)*)? ')'
    ;

literal
    : INT
    | STRING
    | BOOL
    ;

typeExpr
    : typeTerm ('->' typeTerm)*
    ;

typeTerm
    : 'Int'
    | 'Bool'
    | 'String'
    | '(' typeExpr ')'
    ;

Operator
    : '+' | '-' | '*' | '/' | '==' | '!=' | '>' | '<'
    ;

BOOL
    : 'true'
    | 'false'
    ;

IDENT
    : [a-zA-Z_] [a-zA-Z0-9_]*
    ;

INT
    : [0-9]+
    ;

STRING
    : '"' (~["\\] | '\\' .)* '"'
    ;

WS
    : [ \t\r\n]+ -> skip
    ;

COMMENT
    : '|' '|'? ~[\r\n]* -> skip
    ;

# vim: ft=antlr
