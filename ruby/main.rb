$DEBUG = true
require_relative 'quest'
require_relative 'examples/frac'

p Quest::Object.g(:init).(::Quest::Block.new{
	::Quest::Text.new("12345")
		.get_attr(:map).(_{ |x| x.c :@num })
		.get_attr(:map).(_{ |x| x.c :**, 2.to_q })
		.get_attr(:filter).(_{ |x| x.g(:>).(4.to_q) })

	# fac = ::Quest::Block.g(:init).(_{
	# 	set_attr :memo, Quest::Object.c(:init, _{
	# 		set_attr 0.to_q, 1.to_q
	# 	})
	# 	1.to_q
	# }).();
	# fac1 = Quest::Object.c(:init, _{
	# 	# self.s :memo, { 0.to_q => 1.to_q, 1.to_q => 1.to_q }.to_q;
	# 	self.set_attr :memo, Quest::Object.c(:init, _{
	# 		set_attr 0.to_q, 1.to_q
	# 		self
	# 	})

	# 	self.s :'()', _{ |number|
	# 		val = (memo = g('memo'.to_q)).get_attr number;
	# 		g(:if).(val, _{ val }, _{
	# 			memo.set_attr(
	# 				number,
	# 				number.call_attr(:*, self.call(number.call_attr(:-, 1.to_q)))
	# 			)
	# 		}).()
	# 	}
	# 	self
	# });

	# fac.call(8.to_q)


	# m = ::Quest::Map.new({'a'.to_q => 2.to_q})
	# m.call_attr(:[]=, 'b'.to_q, 3.to_q)
	# m.call_attr(:map, _{ |k, v| ::Quest::List.new [k, v.call_attr(:**, 2.to_q)]})
	# ::Kernel::p g(:system).("ls")
	# ::Quest::List.new([1.to_q, 2.to_q, 3.to_q, 4.to_q, 5.to_q])
	# ::Quest::Text.new("12345")
	# 	.get_attr(:map).(_{|x| x.call_attr(:@num) })
	# 	.get_attr(:map).(_{|x| x.get_attr(:**).(2.to_q) })
	# 	.get_attr(:filter).(_{|x| x.g(:>).(4.to_q) })
		# .c(:map_filter, _{ |x| c(:if, (x = x.c(:**, 2.to_q)).c(:>, 15.to_q), x) })
})