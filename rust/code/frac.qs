`Frac` = {
	`init` = {
		`$this` = {
			`numer` = @0;
			`denom` = @1;
			$locals
		}!;
		($this :: `[]=`) : ($this `+` (Frac :: `+`) $stack);
		($this :: `[]=`) : ($this `init` (Frac :: `init`) $stack);
		($this :: `[]=`) : ($this `@text` (Frac :: `@text`) $stack);
		$this
	};
	`+` = {
		`this` = @0;
		`rhs` = @1;
		`denom` = this :: `denom`;
		`numer` = this :: `numer`;
		(this :: `init`):(numer + rhs * denom, denom, $stack)
	};
	`@text` = {
		`numer` = @0 :: `numer`;
		`denom` = @0 :: `denom`;
		"" + numer + (denom == 1 and "" or "/" + denom)
	};
	$locals
}!;

`f` = (Frac :: `init`):(2 5 $stack);

disp:(f + 3, $stack);