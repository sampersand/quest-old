if(cond, if_true, if_false)

add_two = {
	@hi ||= 1;
	@0 + @1 + @hi
};

add_two(1, 2, hi = 3)

x = x['fred'] || 12

[false, 2, "hi"][1.2]

Car = {
	@locals += Vehicle;

	init = {
		@self.wheels = @wheels;
		@self.maker = @maker;
		@self
	};

	to_text = {
		"A car made by " + @self.maker + ", with " + @self.wheels
	};

	@locals
}();

{
	'init' = <func>,
	'to_text' = <func>
}


3 < x < 5

5 mod 2
5 pow 2
5 ^ 2

5 ^^ 2
5 | 2
5 bitor 2
5 bitxor 2


`3` `<[ x ]>=` `5`

=begin


=end

x <<= 3
`x` `_` `<` `<` `=` `_` `3`
`x` `<<` `=` `3`
`x` `<` `<=` `3`
`x` `<<=` `3`






















Car = {
	init = {
		@self.wheels = @wheels;
		@self.maker = @maker;
		return(@self);
	};
	to_text = {
		return("A car by " + @self.maker + " with " + @self.wheels);
	};
};



list = {
	2 => false
};

list.append(null);



list = {
	0 => "hi",
	1 => 2,
	2 => false
};

list = ["hi", 2, false];

list.append(null);
list = ["hi", 2, false, null];


list = {
	0 => "hi",
	1 => 2,
	2 => false,
	3 => null
};

car = Car().init(maker = "honda", wheels = 4);

disp("I own a: " + car.to_text());

'Car': {
	'init': function
}

=begin
	thi

=end

x <<= 1
`x` `<` `<` `=` `1`
`x` `<<` `=` `1`
`x` `<<=` `1`

4:30 - 5:30

if (my_var == 0) {
	x ++;
} else {
	y++;
}

if(my_var == 0, { x++ }, { y++ });
if(!my_var, { x++ }, { y++ });




3

//disp(true);

__EOF__
for = {
	@0{}; # execute the condition
	c = @1;
	d = @2;
	e = @3;
	while{ c, {
		d{};
		e{};
	};
	exit(1);
	#while{ @1, @2 } # execute the two others in block
};

sum = {
	b = @*;
	for({i = 0; tot = 0 }, { i < b.len() - 1 }, { i += 1 }, {
		tot += b.get(c = "@" + i; c.@var()){}{}{}{}{}{}{}{}{}{};
		exit(1);
	});
	exit(2);
};

a = (1, 9, 3);
disp(a, sum(a));
exit(0);