require_relative 'object'

class Quest::Number < Quest::Object
	def initialize num
		warn "Expected a Number (got #{num.class})" unless num.is_a? ::Numeric
		@num = num
		super()
	end

	define_attrs parent: ::Quest::Object do 
	end
end