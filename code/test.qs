return(1, 2);
foo = {
	disp("b 1");
	{
		disp("c 1");
		{
			disp("d 1");
			return(1, return(1, 2));
			disp("d 2");
		}();
		disp("c 2");
	}();
	disp("b 2");
};

disp("a 1");
disp(foo(1));
disp("a 2");

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