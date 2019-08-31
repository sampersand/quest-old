require_relative 'object'

class String
	def to_q; Quest::Text.new self end
end

class Quest::Text < Quest::Object
	def initialize text
		warn "Expected a String (got #{text.class})" unless text.is_a? ::String
		@text = text.freeze
		super()
	end

	def clone
		::Quest::Text.new @text
	end

	def __text; @text end

	def inspect
		"Text(#{@text.inspect})"
	end


	define_attrs do 
		define_attr :@text do
			clone
		end

		define_attr :@bool do
			::Quest::Boolean.new !@text.empty?
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

		define_attr :[] do |start, stop=nil, step=nil|
			if step
				::Kernel::fail "todo: step"
			elsif stop
				::Quest::Text.new(@text[start.call_attr(:@num).__num..stop.call_attr(:@num).__num] || '')
			else
				::Quest::Text.new(@text[start.call_attr(:@num).__num] || '')
			end
		end

		define_attr :[]= do |start, stop=nil, step=nil, value|
			text = @text.clone
			text.unfreeze

			if step
				::Kernel::fail "todo: step"
			elsif stop
				# this can throw if start..stop is out of range. idk what to do then
				text[start.call_attr(:@num).__num..stop.call_attr(:@num).__num] = value.call_attr(:@text).__text
			else
				text[start.call_attr(:@num).__num] = value.call_attr(:@text).__text
			end

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