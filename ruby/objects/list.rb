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


	define_attrs ancestors: [
		::Quest::StepParents::Enumerable,
		::Quest::StepParents::Comparable,
		::Quest::StepParents::TruthyContainers,
		::Quest::Object
	] do

		define_attr :@text do
			::Quest::Text.new "[" + @list.map{|l| l.call_attr(:@text_inspect).call_into(:@text) }.join(', ') + "]"
		end

		define_attr :<=> do |rhs|
			::Quest::Number.new (@list <=> rhs.call_into(:@list)) || ::Float::NAN
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
			::Quest::List.new @list + rhs.call_into(:@list)
		end

		define_attr :'+=' do |rhs|
			@list = call_attr(:+, rhs).__list
			self
		end

		define_attr :cross do |rhs|
			::Quest::List.new @list.product(rhs.call_into :@list).map(&::Quest::List.:new)
		end

		define_attr :* do |rhs|
			if rhs.call_attr(:is_a, ::Quest::List).call_into :@bool
				call_attr :cross, rhs
			else
				::Quest::List.new @list * rhs.call_into(:@num)
			end
		end

		define_attr :'*=' do |rhs|
			@list = call_attr(:*, rhs).__list
			self
		end

		define_attr :[] do |start, stop=nil, step=nil|
			start = start.call_into :@num
			stop = stop.call_into :@num if stop
			step = step.call_into :@num if step

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
			when nil then # do nothing
			else ::Kernel::fail "All cmps for stop failed"
			end

			case
			when step then ::Kernel::fail "todo: step"
			when stop then ::Quest::List.new(@list[start..stop] || [])
			else @list[start] || ::Quest::Null.new
			end
		end

		define_attr :[]= do |start, stop=nil, step=nil, value|
			start = start.call_attr :@num
			stop = stop.call_attr :@num if stop
			step = step.call_attr :@num if step

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
			when nil then # do nothing
			else ::Kernel::fail "All cmps for stop failed"
			end

			case
			when step then ::Kernel::fail "todo: step"
			when stop then @list[start..stop] = value
			else @list[start] = value
			end

			self
		end

		define_attr :each do |block|
			@list.each do |element|
				block.call_attr :'()', element
			end
			self
		end
	end
end