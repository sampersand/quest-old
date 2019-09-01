require_relative 'util'

Quest::Object.get_attr(:init).call(_{
	max = 100._;
	secret = rand(1._, max);
	guesses = 0._;
	guess = nil._;

	_while(_{ guess != secret }, _{
		guess = prompt("Guess from 1-"._ + max).c(:@num);
		'++@'.(guesses);
		disp(
			_if(guess < secret, _{"too low!"._}, _{
				_if(guess > secret, "too high!"._, "perfect!"._)
			}).()
		);
	});
	disp("It took you "._ + guesses.c(:@text) + ' tries!'._);
});


