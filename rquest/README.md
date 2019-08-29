#RQuest

This is another take on quest, where I'm trying to match the syntax to ruby syntax. All code here
is valid ruby code, but also works like quest would (barring a few quirks).
For example, this
```ruby
include RQ
RQ.exec do
	x = 0;
	_while.(_{x < 10}, _{
		x += 1;
		_if.(x < 5, _{
			disp.(x, "< 5\n")
		}, _{
			disp.(x, ">= 5\n")
		}).()
	})
end
```
is equivalent to the Quest
```
x = 0;
while({x < 0}, {
	x += 1;
	if(x < 5, {
		disp(x,"< 5\n")
	}, {
		disp(x, ">= 5\n")
	})();
});
```

However, a few issues with the `binding`s that Ruby uses made this method untenable, and it's kept here
just for posterity