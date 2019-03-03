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

/* Normal Operators */
	// Equality
	STRICT_EQ = "==="; STRICT_NEQ = "!==";
	EQ = "=="; NEQ = "!=";
	
	// Comparison
	LT = "<"; GT = ">";
	LEQ = "<="; GEQ = ">=";
	CMP = "<=>";
	
	// Logical
	NOT = "!"; AND = "and"; OR = "or";

	// Bitwise
	BW_XOR = "^"; BW_AND = "&"; BW_OR = "|"; 
	BW_LSH = "<<"; BW_RSH = ">>"; BW_NOT = "~";

	// Mathematical
	ADD = "+"; SUB = "-"; MUL = "*"; DIV = "/";
	POW = "**"; MOD = "%"; 
	POS = "@+"; NEG = "@-";

/* Assignment Operators */
	ASSIGN = "=";
	ARROW_LEFT = "<-"; ARROW_RIGHT = "<-";

	ADD_ASSIGN = "+="; SUB_ASSIGN = "-="; MUL_ASSIGN = "*="; DIV_ASSIGN = "/=";
	MOD_ASSIGN = "%="; POW_ASSIGN = "**=";
	BW_XOR_ASSIGN = "^="; BW_AND_ASSIGN = "&="; BW_OR_ASSIGN = "|=";

/* Misc Operators */
	PERIOD = ".";
	COLON_COLON = "::";
	COMMA = ",";
	ENDLINE = ";";

/* Indexing and Calling */
	INDEX = "[]";
	INDEX_ASSIGN = "[]=";
	INDEX_DELETE = "[]~";
	INDEX_HAS = "[]?";
	EXECUTE = "()";

/* Misc */
	MISC_CLONE = "clone";
	MISC_LEN = "len";
}
