macro_rules! define_consts {
	($($name:ident = $val:expr;)*) => (
		$(pub const $name: &str = $val;)*
	)
}
define_consts!{
	// Conversions
	AT_TEXT = "@text";
	AT_BOOL = "@bool";
	AT_NUM = "@num";

	// Equality
	STRICT_EQ = "==="; STRICT_NEQ = "!==";
	EQ = "=="; NEQ = "!=";

	// Comparison
	LT = "<"; GT = ">";
	LEQ = "<="; GEQ = ">=";
	CMP = "<=>";

	// Boolean operations
	NOT = "!"; AND = "and"; OR = "or";

	// Bitwise operators
	B_XOR = "^"; B_AND = "&"; B_OR = "|"; 
	B_LSH = "<<"; B_RSH = ">>"; 

	// Assignment Operators
	ARROW_LEFT = "<-";
	ARROW_RIGHT = "<-";

	// Misc
	CLONE = "clone";
}