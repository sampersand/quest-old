`map` = { `x`=59; `y`=12; $locals }!;
`x` = {
	`$this` = map;
	{ -> `key`; ($this :: `[]`):($this key $stack) }
}!;

(map :: `[]=`):[map `get` x $stack]!;
(map :: `[]=`):[map `set` { `$this` = map; {
	`key` = @0; `val` = @1;
	($this :: `[]=`):($this key val $stack)
}}! $stack]!;
(map :: `[]`):(map `get` $stack):(`x` $stack);
(map :: `[]`):(map `set` $stack):(`z` 3 $stack);
(map :: `[]`):(map `get` $stack):(`z` $stack)

__END__
`map` = { `x`=59; `y`=12; $locals }!;
(map :: `[]=`):( map, `get`, {
	`$this` = map;
	{-> `key`; ($this :: `[]`) ($this key $stack) }
}!, $stack);


(map :: `[]`)(map `get` $stack) (`x` $stack)

__END__
`get` = {
	-> `key`;
	-> `map`;
	(map :: `[]`) (map, key, $stack)
};
`set` = {
	-> `val`; -> `key`; -> `map`;
	(map :: `[]=a`) (map, key, val, $stack)	
};

set (map, `get`, {
	`call` = map :: `[]`;
	{ -> `key`; call ($this, key, $stack) }
}!)

__END__
map = (
	x = 59;
	y = 12;
	$locals!
);

get = {
	map = @0!;
	key = @1!;
	(map! :: `[]`) (map! key! $stack!)
};


set! ( map! `set` { this = map!; {
	key = @0!;
	val = @1!;
	disp! (key! val! $stack!);
	(this! :: `[]=`) (this! key! val! $stack!)
}}! $stack! );

__END__
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