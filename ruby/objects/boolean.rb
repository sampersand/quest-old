require_relative 'object'

class TrueClass
	def to_q; Quest::Boolean.new self end
end

class FalseClass
	def to_q; Quest::Boolean.new self end
end

class Quest::Boolean < Quest::Object
	def initialize bool
		::Quest::if_debug do
			unless bool.is_a? ::TrueClass or bool.is_a? ::FalseClass
				::Quest::warn "Boolean::initialize received a non-Boolean arg '#{bool.inspect}'"
			end
		end

		@bool = bool
		super()
	end

	def clone
		::Quest::Boolean.new @bool
	end

	def true?; @bool end

	def inspect
		"Boolean(#@bool)"
	end

	def __bool; @bool end

	define_attrs stepparents: [::Quest::StepParents::Comparable] do 
		define_attr :@text do
			::Quest::Text.new @bool.to_s
		end

		define_attr :<=> do |rhs|
			::Quest::Number.new (@bool <=> rhs.call_attr(:@bool).__bool) || ::Float::NAN
		end

		define_attr :@bool do
			clone
		end

		define_attr :@num do
			::Quest::Number.new (@bool ? 1 : 0)
		end

		define_attr :'!' do
			::Quest::Boolean.new !@bool
		end

		define_attr :__BIT_AND do |rhs|
			::Quest::Boolean.new @bool & rhs.call_attr(:@bool).__bool
		end

		define_attr :__BIT_OR do |rhs|
			::Quest::Boolean.new @bool | rhs.call_attr(:@bool).__bool
		end

		define_attr :__BIT_XOR do |rhs|
			::Quest::Boolean.new @bool ^ rhs.call_attr(:@bool).__bool
		end

		define_attr :== do |rhs|
			::Quest::Boolean.new rhs.is_a?(::Quest::Boolean) && @bool == rhs.__bool
		end
	end
end