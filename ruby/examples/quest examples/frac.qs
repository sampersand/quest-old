Frac = {
	$self = Object.init();
	init = {
		$self = Object.init.bind($self);
		numer = $1;
		denom = $2;
		$self
	};

	`+` = {
		if($1.is_a(Frac), {
			Frac.init(
				$self.numer * $1.denom + $self.denom * $1.numer,
				$self.denom * $1.denom
			)
		}, {
			Frac.init($self.numer + $self.denom * $1.@num(), $self.denom)
		})()
	};
}