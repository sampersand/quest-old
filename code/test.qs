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