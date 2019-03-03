macro_rules! define_consts {
	($($name:ident = $val:expr;)*) => (
		$(pub const $name: &str = $val;)*
	)
}
define_consts!{
/* Conversions */
	AT_TEXT = "@text";
	AT_BOOL = "@bool";
	AT_NUM = "@num";
	AT_LIST = "@list";
	AT_MAP = "@map";
	AT_VAR = "@var";

/* Normal Operators */
	// Equality
	STRICT_EQL = "==="; STRICT_NEQ = "!==";
	EQL = "=="; NEQ = "!=";
	
	// Comparison
	LTH = "<";  GTH = ">";
	LEQ = "<="; GEQ = ">=";
	CMP = "<=>";
	
	// Logical
	NOT = "!"; AND = "and"; OR = "or";

	// Bitwise
	B_XOR = "^"; B_AND = "&"; B_OR = "|"; 
	B_LSH = "<<"; B_RSH = ">>"; B_NOT = "~";

	// Mathematical
	ADD = "+"; SUB = "-"; MUL = "*"; DIV = "/";
	POW = "**"; MOD = "%"; 
	POS = "@+"; NEG = "@-";

/* Assignment Operators */
	ASSIGN = "=";
	ARROW_LEFT = "<-"; ARROW_RIGHT = "->";

	ADD_ASSIGN = "+="; SUB_ASSIGN = "-="; MUL_ASSIGN = "*="; DIV_ASSIGN = "/=";
	MOD_ASSIGN = "%="; POW_ASSIGN = "**=";
	B_XOR_ASSIGN = "^="; B_AND_ASSIGN = "&="; B_OR_ASSIGN = "|=";

/* Misc Operators */
	COMMA = ",";
	ENDLINE = ";";
	CALL = "()";

/* Indexing and Calling */
	INDEX = "[]";
	INDEX_ASSIGN = "[]=";
	INDEX_DELETE = "[]~";
	INDEX_HAS = "[]?";

	ACCESS = ".";
	ACCESS_ASSIGN = ".=";
	ACCESS_DELETE = ".~";
	ACCESS_HAS = ".?";

	COLON_COLON = "::";


/* Literals */
	L_CLONE = "clone";
	L_LEN = "len";
	L_EVAL = "eval";
	L___ID__ = "__id__";
	L___MAP__ = "__map__";
	L___ENV__ = "__env__";
}


