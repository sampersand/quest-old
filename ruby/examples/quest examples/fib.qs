factorial = {
	$self = Object.init();
	`memo` = {
		$self = Object.init();
		0 = 1;
		$self
	}();

	`()` = {
		num = $1;
		val = $self.memo[num];
		if(val, {
			val
		}, {
			$self.memo[num] = $self(num-1) + $self(num-2)
		})()
	};
	$self
}();
