r = rand();

a = 1;
b = 2;
disp(a);
disp(b);

__END__

foo = [];;;
bar = foo.clone();

foo.push(1);

bar

__END__
disp(bar.len());
disp(foo; bar);

__END__
foo = [3, 4];
#bar = foo.clone();


#foo.`.=`(0, 2);

foo.push(2);
disp(foo, bar);

__END__

disp(foo.get(0))

__END__
Car = {
	init = {
		@self.wheels = 1;
	};
};

car = Car();



__END__
define_attr(`<<`, 1, 1, {
	@lhs.`<<`(@rhs)
})
`l` = [];
l.`<<`(1);

if(l, {
	disp("l is true:", l);
}, {
	disp("l is false:", l)
})();

disp(l);
disp(1, 2, 3);

__END__
if(true, )
`a` = 3;
a.`.=`(`b`, 9);
a.`b`
__END__
`a` = { @stack.`len`.{} };
a(1, 9, 4)
#a . { 1 };
#`a` = { (@stack.`len`)() };
#a.(1, 9, 4)
__END__
`a` = {
	@stack.`len`
};

a(-4);`@stack`

__END__
#4 + 3 9;;

#99 + 1, 4 * 6

#4 * 5;

__EOF__
#1 + 2 * 3
__EOF__
#1 * 2 ^ 3 * 4
__END__"hi there \# a";]
[1 2]
()

__EOF__
{
	{ 1 } 2    `true!` { 3 4 }
}
__EOF__
{
	1 "}" 2 // }
}

#123 2 4.912 #1 + 2
#foo = "hi i am sam";

#foo = 3;