require_relative 'rq'
include RQ
RQ.exec do

x = 0
_while.(_{x < 10}, _{
	x += 1;
	_if.(x < 5, _{
		disp.(x + " < 5\n")
	}, _{
		disp.("#{x} >= 5\n")
	}).()
})

# fib = _{
# 	memo = ::RQ::Delegator.new # hack

# 	memo.__ZERO__ = 1;
# 	_{
# 		(cache = memo.__get_attr(@_1)) && _return.(2, cache);
# 		memo.__set_attr(@_1, @_1 * fib.(@_1 - 1))
# 	}
# }.[];


# p fib.(2)
end