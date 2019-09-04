MAX = 100;
secret = rand(1, MAX);
guesses = 0;

try({ 
	uniq = $1;
	loop({
		guess = prompt("Pick a number from 1-" + MAX.@text()).@num();
		guesses += 1;
		{
			-1 = { disp("Too low!"); };
			1 = { disp("Too high!"); };
			0 = { disp("perfect!"); };
			$locals
		}()[guess <=> guesses]
	})
});