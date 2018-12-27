__EOF__
add_two = { + 2 };

disp(1.`@env`)
disp(add_two(3));


__END__

if(x){
	y = 3
} else {
	y = 4;
}


y = if(x, 3, 4);

if(x, {
	$1.`.=`(`y`, 3)
}, {
	$1.`.=`(`y`, 4)
})();

{
	 $1.`.=`(`x`, 1);
}();

disp(x);

__END__

x = {
	$1.`.=`(`y`, 1);
}();

disp(
	4(3 + 2)
);

__END__
//disp(y);

//fib = {
//	arr = @2 or [0, 1];
//	if(@2.`not`(), { @1 -= 2; });
//
//	if(@1 < 0, { return($2, @2) });
//
//	@2.`push`(@2.-1);
//	disp(arr);
//};

fib = {
	arr = @2 or [0, 1];
	arr.`push`(2);
	disp(arr).`get`(1)
};

disp(fib(3));

__END__

add_two = { @1 + 2 };
disp(add_two(3));

__END__
MAX = 10;
secret = rand() % MAX;
guesses = 0;

done = false;

while({ done == false }, {
	guess = prompt("Pick a number from 0 to " + MAX + " (guess #" + guesses + ")").@num();
	guesses.`@++`();
	if(guess < secret, {
		disp("too low!");
	}, if(guess > secret, {
		disp("too high!");
	}, {
		$2.`.=`(`done`, true);//(`@locals`).set(`done`, true);
	}))();
});

disp("it took you", guesses, "tries to guess the number");



__END__
1 - 3;
#true.`foo`()
true.`foo`

__END__
fib = {
	arr = @2 or [0, 1];
	if(@2.`not`(), { @1 -= 2; });

	if(@1 < 0, { return($2, @2) });

	@2.`push`(@2.-1);
	disp(arr);
};

disp(fib(3));

return($-1);

i = 0;
while({ i < 10 }, {
	i += 1;
	if( (i % 2).`not`(), {
		return($2);
	})();
	disp("i", i);
})();

disp(i);
add_two = {
	one + two
};

disp(
	add_two(
		`one` = 1,
		`two` = 2
	)
);


fac = {

	if(@1 <= 0, {
		return($3, 1);
	})();

	@1 * fac(@1 - 1)
};

disp(fac(5));
return($-1);
`x` = true;
x x;
`foo` = {
	#y = @0;
	y = @0;
	z = @0;
	disp($1.`@stack`);
};

disp(foo(99 12));
__END__
x = true;
y = {
	x, x, x,
	@stack
}();

exit = {
	z = @0;
	disp($1.`@stack`);
	return($-1, @0 or 1);
};

exit(2 3 4 5 );
x.`.=`(`+`, {
	disp($-2.`@locals`);
	+ x.@num() // 1
});

disp(x + 9);
__END__
x = false;
x.`.`(`.=`)(2, "hi");

x.`.=`(`+`, {
	x.`@ + 1;
});

disp(x.@num());

__END__
map = { @locals }();
map.set("hi", 2);
map.get("hi");

name = "sam";
map.("hi " + name) = 2;

foo = {
	if(@0 == "hi", {
		return($0, "hi");
		aoefhiawef
	});

	switch(x){
		case 1: disp("hi");
		case 2: disp("no"); break;
		default: disp("lol"); break;
	};

	{
		1 = { disp("hi"); $1.get(2)() };
		2 = ...

	}().get(x, { disp("lol" )})


	{
		while({i < 10}, {
			i += 1;
			return($0);
			return($1);
		})
	}();
};



`foo`.`=`(1);

`foo` = 1;
`bar` = foo + 2;

`x` = true;
x.@class().`.=`(`=`, { rhs == true });

x = false

`=`







disp(true.1);

__END__
x = {@locals}();
x.foo = 2;

__END__
disp(x);


exit = { return($-1) };
disp(@locals);
exit();
__END__
add_two = {
	res = one + two;

	disp($-1.`@locals`);#.result = 1;
	$-1.`.=`(`result`, res);
	res
};

disp({1 2 @stack}());

disp(add_two(one = 5; two = 9;));
disp(result);


__END__
foo = {
	x = ;
	disp(" b+");
	{
		disp("  c+");
		{
			disp("   d+");
			return($3, 4);
			disp("   d-");
		}();
		disp("  c-");
	}();
	disp(" b-");
};

disp("a+");
disp(foo(1));
disp("a-");

__EOF__
disp(4.`^`(2));
disp(4.`^`(2));

num = true;
num.`.=`(`+`, {
	@1
});

disp(num + 1);

__END__
list = {
	1
	2
	@stack
}();

map = {
	a = 1;
	b = 2;
	@locals
}();

disp(list, map);
__END__



Car = {
	init = {

	};
	@locals
}();

car = Car.get(`init`)(Car.clone());
car