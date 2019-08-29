require_relative 'rq'

RQ::exec self do



# delete the `.` in `.(`
# delete the `_` in `_{`
# replace `__<SYMBOL>__` with the symbol itself
# replace `_RUBYKEYWORD` WITH `RUBYKEYWORD`
# delete the `{}` in `locals`
MAX = prompt.("Pick a maximum number: ").__ATSIGN__num.();
secret = random.(1, MAX);
guesses = 0;
dispn.("Guess a number from 1-" + MAX);

_loop.(_{
	guess = random.("> ").__ATSIGN__num.()
	guesses += 1
	dispn.(switch.(guess <=> secret, _{
		__MINUS____ONE__ = _{ "too low!" };
		__ZERO__ = _{ _return.(5) };
		__ONE__ = _{ "too high!" };
		p $binding
		$locals.(){}
	}.()));
});
dispn.("Correct!\n It took you " + guesses + " tries!");



