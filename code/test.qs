#disp _D [ "ls"! $stack ]!;
disp! ( "cat code/foo.qs"!, $stack! )

__END__
`x` = 1;

disp! _D [ "`" + x! + "` is", (if! _D [x! "truthy" "falsey" $stack!]!), $stack!]!;
(if! _D [
	x!
	{ disp! _D [ "`" + x! + "` is truthy", $stack!]! },
	#{ disp! _D [ "`" + x! + "` is falsey", $stack!]! }
	$stack!
]!)!;

`x` = 3;
l = $locals!;
($locals! :: `[]=`) _D [l! `x` 4 $stack!]!;
x!
__END__
map = {
	x = 59;
	y = 12;
	$locals!
}!;

get = {
	map = @0!;
	key = @1!;
	(map! :: `[]`) _D [map! key! $stack!]!
};

set = {
	map = @0!;
	key = @1!;
	val = @2!;
	(map! :: `[]=`) _D [map! key! val! $stack!]!
};

set! _D [ map! `set` { this = map!; {
	key = @0!;
	val = @1!;
	disp! _D [key! val! $stack!]!;
	this! :: `[]=` _D [this! key! val! $stack!]!
}}! $stack! ]!;

(get! _D [map! `set` $stack! ]!) _D [z 3 $stack!]!;
(get! _D [map! `z` $stack! ]!) _D [z 3 $stack!]!;


#map!::`[]` _D [map! x $stack!]!

#add_two = { @0! }; #-> a; -> b; a! + b! };
#add_two! _D [ 1 5 9 4 $stack! ]! # => 3
__END__
`x` = 4;
`y` = {
	`x` = 3;
	{ 1 + x! }
};
{ @0! } _D [1]

#(y _D [1, 2])
#y!!! # => returns `4`

__END__
#(1 + 2)

#{2 + 3}::`()` ! {2 + 3}
__END__
`y` = 3;
`x` = `y`;
4 + 5 * 6;
4 * 5 ** 6;

`y` = 3;
`x` = `y`;
4 + 5 * 6;
4 * 5 ** 6;
`y` = 3;
`x` = `y`;
4 + 5 * 6;
4 * 5 ** 6;
`y` = 3;
`x` = `y`;
4 + 5 * 6;
4 * 5 ** 6

//x!!; //123 + 456
__END__
#foo = {1};
#x = foo! :: `()`;
#5 * {4 + 4}_!
#1, 2
x = 1, y = 2;;
a = b = 3;
$locals