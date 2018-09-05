MAX = 100;
secret = rand() % MAX;
guesses = 0;

done = false;

while({ done == false }, {
	guess = prompt("Pick a number from 0 to " + MAX + " (guess #" + guesses + ")").@num();
	guesses.`@++`();
	if(guess < secret, {
		disp("too low!");
	}, if(guess > secret, {
		disp("too high!");
	}, {
		$2.get(`@locals`).set(`done`, true);
	}))();
});

disp("it took you", guesses, "tries to guess the number");
