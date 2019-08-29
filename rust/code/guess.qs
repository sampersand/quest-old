`secret` = (0 :: `round`):((rand:($stack) * 100) $stack);
`guesses` = 0;
`l` = $locals;

loop:({
	`guess` = input:("Guess: " $stack);
	`guess` = (guess::`@num`) : (guess $stack);
	l::`[]=` : (l `guesses` guesses + 1, $stack);
	switch:(guess <=> secret, {
		`l` = $locals;	(l::`[]=`):(l, 0-1, { disp:("too small" $stack); } $stack);
		(l::`[]=`):(l, 0, { return:($`-1` $stack); } $stack);
		(l::`[]=`):(l, 1, { disp:("too large" $stack); } $stack);
		$locals
	} $stack):($stack);
} $stack);
disp:("It took you " + guesses + " tries.", $stack);
