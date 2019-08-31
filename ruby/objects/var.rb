require_relative 'object'

class Quest::Var < Quest::Object
	def initialize var
		warn "Expected a Symbol (got #{var.class})" unless var.is_a? ::Symbol
		@var = var
		super()
		freeze
	end

	define_attrs parent: ::Quest::Object do 
	end
end