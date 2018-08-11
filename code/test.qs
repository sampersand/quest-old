a = (1, 2);
foo = {
	a = @1 + @0;
	return(2);

};

b = foo{1, 2};

disp(b, a)