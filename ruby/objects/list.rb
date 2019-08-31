require_relative 'object'

class Array
	def to_q; Quest::List.new self end
end

class Quest::List < Quest::Object
	def initialize list
		::Quest::if_debug do
			unless list.is_a? ::Array
				::Quest::warn "List::initialize received a non-Array arg '#{list.inspect}'"
			end

			list.each do |ele|
				unless ::Quest::quest_object? ele
					::Quest::warn "List::initialize recieved a non-Object element '#{ele.inspect}"
				end
			end
		end

		@list = list
		super()
	end

	def clone
		::Quest::List.new @list.clone
	end

	def __list
		@list
	end

	def inspect
		"List(#{@list.inspect})"
	end


	define_attrs stepparents: [
		::Quest::StepParents::Comparable,
		::Quest::StepParents::TruthyContainers
	] do

		define_attr :@text do
			::Quest::Text.new "[" + @list.map{|l| l.call_attr(:@text_inspect).__text }.join(', ') + "]"
		end

		define_attr :<=> do |rhs|
			::Quest::Number.new (@list <=> rhs.call_attr(:@list).__list) || ::Float::NAN
		end

		define_attr :@list do
			clone
		end

		define_attr :length do
			::Quest::Number.new @list.length
		end

		define_attr :== do |rhs|
			::Quest::Boolean.new rhs.is_a?(::Quest::List) && @list == rhs.__list
		end

		define_attr :+ do |rhs|
			::Quest::List.new @list + rhs.call_attr(:@list).__list
		end

		define_attr :cross do |rhs|
			::Quest::List.new @list.product(rhs.call_attr(:@list).__list).map(&::Quest::List.:new)
		end

		define_attr :* do |rhs|
			if rhs.call_attr(:is_a, ::Quest::List).call_attr(:@bool).true?
				call_attr :cross, rhs
			else
				::Quest::List.new @list * rhs.call_attr(:@num).__num
			end
		end

		define_attr :[] do |start, stop=start, step=nil|
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
			::Quest::List.new(@list[start..stop] || '')
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