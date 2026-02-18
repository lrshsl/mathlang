grammar mth;


module
	: toplevel* EOF
	;

toplevel
	: fn_decl ';'
	| var_assign ';'
	| expr ';'
	;

fn_decl
	: IDENT '(' paramlist ')' '=' expr
	;

paramlist
	: (IDENT ',')* IDENT?
	;

var_assign
	: IDENT '=' expr
	;

expr
	// lower precedence first

	// Logical ops
	: expr 'or' expr                                    # logical_or
	| expr 'and' expr                                   # logical_and
	
	// Bitwise ops
	| expr 'binary_or' expr                             # bitwise_or
	| expr 'xor' expr                                   # bitwise_xor
	| expr 'binary_and' expr                            # bitwise_and
	
	// Comparison ops
	| expr ( '==' | '<' | '>' | '<=' | '>=' ) expr      # comparison
	
	// Arithmetic
	| expr ( '+' | '-' ) expr                           # add_sub
	| expr ( '*' | '/' ) expr                           # mul_div
	| expr '^' expr                                     # power
	
	| primary                                           # atom
	;

primary
	:'(' expr ')'
	| fn_call
	| IDENT
	| INT
	;

fn_call
	: IDENT '(' (expr ',')* expr? ')'
	;


INT  : [0-9]+ ;
IDENT: [a-zA-Z_][a-zA-Z_0-9]* ;

WS: [ \t\n\r\f]+ -> skip ;

// vim: et! sw=3 ts=3 sts=3
