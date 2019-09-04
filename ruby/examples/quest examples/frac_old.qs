old_if = Builtins.if
Builtins.if = {
	old_if(rand(1,100) == 1, {
		Builtins.while($1, $2)
	}, {
		old_if($1, $2, $3)
	})
}

Builtins.if = {
	old_if(rand(1, 100) == 1, {
		$self = Builtins.while;
		retry();
	}, {
		old_if($1, $2, $3)
	})
}


square = { $1 ** 2 };
square2 = { result = $1 ** 2; $locals };

Person = {
	name = 'sam';
	age = 21;
	@text = {
		"Person. name " + $self.name + ", age" +  $self.age.@text()
	};
	$locals
}();

Frac = {
	hello_world = 7;
	init = {
		numer = $1;
		denom = $2;
		child = $self.super('init').bind($self)();
		child.numer = numer;
		child.denom = denom
		child
	};

	@text = {
		if($self.denom == 1, {
			$self.numer.@text()
		}, {
			$self.numer.@text() + "/" + $self.denom.@text()
		})()
	}

	+ = {

	};

	$locals
}();


Frac <=> { init: {...}, +: {...}, hello_world: 7 }



frac = Frac.init(3, 4);

disp(frac)

frac + 3 => null