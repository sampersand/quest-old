require_relative 'object'

class String
	def to_q; Quest::Text.new self end
end

class Quest::Text < Quest::Object
	def initialize text
		::Quest::if_debug do
			unless text.is_a? ::String
				::Quest::warn "Text::initialize received a non-String arg '#{text.inspect}'"
			end
		end

		@text = text.freeze
		super()
	end

	def clone
		::Quest::Text.new @text
	end

	def hash
		@hash ||= @text.to_sym.hash
	end

	def eql? rhs
		(self == rhs) || rhs.is_a?(::Symbol) && @text = rhs.to_s
	end

	def __text
		@text
	end

	def inspect
		"Text(#{@text.inspect})"
	end


	define_attrs parents: [
		::Quest::StepParents::Comparable,
		::Quest::StepParents::TruthyContainers,
		::Quest::Object
	] do
		define_attr :@text do
			clone
		end

		define_attr :<=> do |rhs|
			::Quest::Number.new (@text <=> rhs.call_into(:@text)) || ::Float::NAN
		end

		define_attr :@text_inspect do
			::Quest::Text.new @text.inspect
		end

		define_attr :@list do
			::Quest::List.new @text.each_char.map(::Quest::Text.:new)
		end

		define_attr :@num do 
			::Quest::Number.new @text.to_f
		end

		define_attr :== do |rhs|
			::Quest::Boolean.new rhs.is_a?(::Quest::Text) && @text == rhs.__text
		end

		define_attr :+ do |rhs|
			::Quest::Text.new @text + rhs.call_into(:@text)
		end

		define_attr :'+=' do |rhs|
			@text = call_attr(:+, rhs).__text
			self
		end

		define_attr :* do |rhs|
			::Quest::Text.new @text * rhs.call_into(:@num)
		end

		define_attr :'*=' do |rhs|
			@text = call_attr(:*, rhs).__text
			self
		end

		define_attr :[] do |start, stop=nil, step=nil|
			start = start.call_into :@num 
			stop = stop.call_into :@num if stop
			step = step.call_into :@num if step

			case start
			when 0    then ::Kernel::raise "Zero not allowed for 'start'"
			when 1..  then start -= 1
			when ..-1 then start += @text.length
			else ::Kernel::fail "All cmps for start failed"
			end

			case stop
			when 0    then ::Kernel::raise "Zero not allowed for 'stop'"
			when 1..  then stop -= 1
			when ..-1 then stop += @text.length
			when nil then #do nothing
			else ::Kernel::fail "All cmps for stop failed"
			end

			::Kernel::fail "todo: step" if step
			::Quest::Text.new(@text[start..(stop || start)] || '')
		end

		define_attr :[]= do |start, stop=nil, step=nil, value|
			start = start.call_into :@num
			stop = stop.call_into :@num if stop
			step = step.call_into :@num if step

			case start
			when 0    then ::Kernel::raise "Zero not allowed for 'start'"
			when 1..  then start -= 1
			when ..-1 then start += @text.length
			else ::Kernel::fail "All cmps for start failed"
			end

			case stop
			when 0    then ::Kernel::raise "Zero not allowed for 'stop'"
			when 1..  then stop -= 1
			when ..-1 then stop += @text.length
			when nil then # do nothing
			else ::Kernel::fail "All cmps for stop failed"
			end

			::Kernel::fail "todo: step" if step
			@text[start..(stop || start)] = value.call_into :@text
			self
		end

		define_attr :each do |block|
			@text.each_char do |char|
				block.call ::Quest::Text.new char
			end
			self
		end
	end
end