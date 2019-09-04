MAX = prompt("Pick a maximum number").@num();
secret = rand(1, MAX);
tries = 0;
loop({
	guess = prompt("Pick a number from 1-" + MAX.@text()).@num();
	tries += 1;
	{
		-1 = { disp("Too low!") };
		 0 = { disp("Perfect! It took you", tries, 'tries!'); return(3); };
		 1 = { disp("Too high!") };
	}()[guess <=> secret]
});