(Note: the version I'm working on is under the branch `restart`)

# What is Quest
Quest is a runtime language designed with maximum extensiblity in mind. The language is based entirely around key-value pairs to allow for redefinition of every part of the language, including the parser.

Note: As Quest is still in the development stages, the exact syntax is probably going to change, but not very drastically.

# Features
- Everything is an object, including operators
- No "keywords"—things like `if` and `while` are actually normal functions
- TODO: Add more features in the example

# Example

```
// You can write comments like this
# Like this
/* or like this */

// assignment is actually just the '=' operator called on variable literals.
// the following two lines are equivalent:
`x`.`=`(3 + 4); // call the '=' function on '`x`' with the argument '3 + 4'
`x` = 3 + 4; // this is just syntactic sugar

`name` = input("What's your name"); 
`fav_color` = input('What\'s your favorite color?'); // you can also use single quotes
`age` = input("How old are you, " + name + "?").`@num`(); // conversions are easy


disp("Hello there,", name);

// 'if' is a function like anything else.
disp("Your age is", if(age % 2 == 0, "n't"), ''), " an even multiple of 2");

// You can also pass blocks of code to if statements
if(fav_color.`lowercase`() == "green", {
	disp("Hey, I like green too!");
}, {
	disp("I guess " + fav_color + " is cool too!");
})(); // And then call the result to execute the correct block


# Functions aren't special at all, but are really just unexecuted code:

`add_two` = { @0 + @1 }; // Add the first two arguments together and return the result.
disp("9 + 3.4 = ", add_two(9, 3.4)); #=> 9 + 3.4 = 12.4

`fibonacci` = {
	`amount` = @0;
	`numbers` = @1 or [0, 1]; // undefined variables are 'null'

	// Quest uses falsey values, so '!0' will be true
	if(amount <= 0, {
		// return numbers up two levels in the stack.
		//   $0 is the current scope,
		//   $1 is the outer scope (ie the body of the function)
		//   $2 is the calling function
		return (numbers, $2);
	})(); // executing 'null' just returns 'null'

	fibonacci(amount - 1, numbers + [numbers[-1] + numbers[-2]])
};

# Classes are just maps, created by executing code blocks.

Car = {
	// the "initialize" function is actually just calling the 'Car' object,
	// and isn't anything special

	`()` = {
		`this` = Map(); // make a new empty map
		// add all of the values from 'Car' in.
		// note that 'this' is currently defined, and thus accessible to the '@text' function.
		this.`update`(Car);
		this.`maker` = @0;
		this.`wheels` = @1 or 4; // wheels are optional and default to 4
	};

	`@text` = {
		"A car by " + this.`maker` + " with " + this.`wheels` + " wheels"
	};

	$locals // returned all variables defined in this scope, as a dictionary
}();

my_car = Car('honda');
disp(my_car); #=> A car made by honda with 4 wheels

```

TODO: Show examples of how to change what operators do

TODO: Add a license and contributing file



