require_relative 'quest'
require_relative 'examples/frac'

p Quest::Object.g(:init).(::Quest::Block.new{
	::Kernel::p g(:system).("ls")
	::Quest::List.new([1.to_q, 2.to_q, 3.to_q, 4.to_q, 5.to_q])
		.c(:map, _{|x| x.g(:**).(2.to_q)})
		.c(:find, _{|x| x.g(:>).(4.to_q) })
		# .c(:map_filter, _{ |x| c(:if, (x = x.c(:**, 2.to_q)).c(:>, 15.to_q), x) })
})