MAX = 100;
secret = rand().`%`(MAX);
guesses = 0;

done = false;
guesses.`+=`(1);

while({ done.`!`() }, {
	guess = prompt("Pick a number from 0 to " + MAX).@num();
	guesses.`@++`();
	if( guess.`<`(secret), {
		disp("too low!");
	}, if( guess.`>`(secret), {
		disp("too high!");
	}, {
		$99.get(`@locals`).set(`done`, true);
	}))();
});

disp("it took you", guesses, "tries to guess the number");
