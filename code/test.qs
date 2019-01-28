`secret` = (0 :: `round`):((rand:($stack) * 100) $stack);
`guesses` = 0;

disp:("S" $stack);
{
	disp:(" S" $stack);
	{
		disp:("  S" $stack);
		{
			disp:("   S" $stack);
			{
				disp:("    S" $stack);
				return:($`-1`, $stack);
				disp:("    E" $stack);
			}!;
			disp:("   E" $stack);
		}!;
		disp:("  E" $stack);
	}!;
	disp:(" E" $stack);
}!;
disp:("E" $stack);
__END__
{
	{
		`x` = {
			{
				return:($2 1 $stack);
				disp:("a" $stack);
			}!
				disp:("b" $stack);
		}!; disp:(x $stack);
				disp:("c" $stack);
	}!
				disp:("d" $stack);
}!
				disp:("e" $stack);

__END__
{
	#disp:($2 $stack);
	return:($1, 1 $stack);
}!

(switch:(1 <=> 0, {
	`l` = $locals;
	($locals::`[]=`):(l, 0-1, { disp:("too low!" $stack); }, $stack);
	($locals::`[]=`):(l, 0, { disp:("correct!" $stack); }, $stack);
	($locals::`[]=`):(l, 1, { disp:("too high!" $stack); }, $stack);
	($locals::`[]~`):(l `l` $stack);
	$locals
}, $stack))!
#disp:(1 <=> 1, $stack);
__END__
while:({ ($2::`[]=`):($2 `guesses` input:('get' $stack) $stack); guesses } {

});
`x` = 0;
`c` = { $1 }!;
while:({x < 4}, {
	disp:(x $stack);
	#`a` = ;
	#disp:(a, $stack);
	(c::`[]=`):(c, `x`, x + 1, $stack);
}, $stack);
1