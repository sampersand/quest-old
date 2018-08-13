true

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