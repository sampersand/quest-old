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


	define_attrs stepparents: [
		::Quest::StepParents::Comparable,
		::Quest::StepParents::TruthyContainers
	] do
		define_attr :@text do
			clone
		end

		define_attr :<=> do |rhs|
			::Quest::Number.new (@text <=> rhs.call_attr(:@text).__text) || ::Float::NAN
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
			::Quest::Text.new @text + rhs.call_attr(:@text).__text
		end

		define_attr :* do |rhs|
			::Quest::Text.new @text * rhs.call_attr(:@num).__num
		end

		define_attr :[] do |start, stop=start, step=nil|
			start = start.call_attr(:@num).__num
			stop = stop.call_attr(:@num).__num
			step = step.call_attr(:@num).__num if step

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
			else ::Kernel::fail "All cmps for stop failed"
			end

			::Kernel::fail "todo: step" if step
			::Quest::Text.new(@text[start..stop] || '')
		end

		define_attr :[]= do |start, stop=start, step=nil, value|
			start = start.call_attr(:@num).__num
			stop = stop.call_attr(:@num).__num
			step = step.call_attr(:@num).__num if step

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
			else ::Kernel::fail "All cmps for stop failed"
			end

			::Kernel::fail "todo: step" if step
			(text = @text.dup)[start..stop] = value.call_attr(:@text).__text
			::Quest::Text.new text
		end

		define_attr :each do |block|
			@text.each_char do |char|
				block.call ::Quest::Text.new char
			end
			self
		end
	end
end