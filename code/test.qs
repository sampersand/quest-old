`MAX` = 10;
`secret` = (rand! * MAX).`round`!;
`guesses` = 0;
`l_assign` = $locals.`[]=`;

`SECRET_HANDLER` = { // so its not reinitialized every time
	$locals.`[]=` : (0-1, { disp:("too small" $stack); } $stack);
	$locals.`[]=` : (0 { return:($`-1` $stack); } $stack);
	$locals.`[]=` :(1 { disp:("too large" $stack); } $stack);

	$locals
};

disp:($0 $stack);
loop:({
	`guess` = (input:("Guess from 0-" + MAX + ": ", $stack)).`@num`!;
	l_assign : (`guesses` guesses + 1, $stack);

	switch:(guess <=> secret, SECRET_HANDLER, $stack):($stack);
} $stack);

disp:("It took you " + guesses + " tries.", $stack);

currently, you have to pass the stack to every function call invocation, i’m working on that for later, but here’s 