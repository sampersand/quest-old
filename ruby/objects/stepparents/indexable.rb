require_relative '../object'

class Quest::StepParents::Indexable < Quest::Object
	define_attrs do
		# We're expecting that we'll have `length` 

		define_attr :index do |start, stop=start, step=nil|
			start = start.call_attr(:@num).__num
			stop = stop.call_attr(:@num).__num
			step = step.call_attr(:@num).__num if step
			list = call_attr(:@list).__list

			case start
			when 0    then ::Kernel::raise "Zero not allowed for 'start'"
			when 1..  then start -= 1
			when ..-1 then start += list.length
			else ::Kernel::fail "All cmps for start failed"
			end

			case stop
			when 0    then ::Kernel::raise "Zero not allowed for 'stop'"
			when 1..  then stop -= 1
			when ..-1 then stop += list.length
			else ::Kernel::fail "All cmps for stop failed"
			end

			::Kernel::fail "todo: step" if step

			get_attr(:__parent__).get_attr(:birth).call list[start..stop]
		end

		define_attr :[]= do |start, stop=start, step=nil, value|
			start = start.call_attr(:@num).__num
			stop = stop.call_attr(:@num).__num
			step = step.call_attr(:@num).__num if step

			case start
			when 0    then ::Kernel::raise "Zero not allowed for 'start'"
			when 1..  then start -= 1
			when ..-1 then start += @list.length
			else ::Kernel::fail "All cmps for start failed"
			end

			case stop
			when 0    then ::Kernel::raise "Zero not allowed for 'stop'"
			when 1..  then stop -= 1
			when ..-1 then stop += @list.length
			else ::Kernel::fail "All cmps for stop failed"
			end

			::Kernel::fail "todo: step" if step
			(list = @list.dup)[start..stop] = value.call_attr(:@list).__list
			::Quest::List.new list
		end

		define_attr :each do |block|
			@list.each_char do |char|
				block.call ::Quest::List.new char
			end
			self
		end
	end
end