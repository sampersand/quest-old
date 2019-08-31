require_relative 'object'

class Numeric
	def to_q; Quest::Number.new self end
end

class Quest::Number < Quest::Object
	def initialize num
		::Quest::if_debug do
			unless num.is_a? ::Numeric
				::Quest::warn "Number::initialize received a non-Numeric arg '#{num.inspect}'"
			end
		end

		@num = num
		super()
	end

	def clone
		::Quest::Number.new @num
	end

	def inspect
		"Number(#@num)"
	end

	def __num; @num end
	def __to_integer; @num.to_i end

	define_attrs stepparents: [
		::Quest::StepParents::Comparable
	] do 
		define_attr :@text do
			::Quest::Text.new @num.to_s
		end

		define_attr :@num do
			clone
		end

		define_attr :@bool do
			::Quest::Boolean.new !@num.zero?
		end

		# Arithmetic Operators
		%i(+ - * / % ** <=>).each do |method|
			define_attr method do |rhs|
				::Quest::Number.new @num.send method, rhs.call_attr(:@num).__num
			end

			# In-place
			define_attr :"#{method}=" do |rhs|
				@num = @num.send method, rhs.call_attr(:@num).__num
				self
			end
		end

		# Bitwise Operators
		%i(__BIT_AND __BIT_OR __BIT_XOR __BIT_SHL __BIT_SHR).each do |method|
			ruby_methods = %i(__BIT_AND & __BIT_OR | __BIT_XOR ^ __BIT_SHL << __BIT_SHR >>).each_slice(2).to_h

			define_attr method do |rhs|
				::Quest::Number.new __to_integer.send ruby_methods[method], rhs.call_attr(:@num).__to_integer
			end

			# in place
			define_attr :"#{method}=" do |rhs|
				@num = __to_integer.send ruby_methods[method], rhs.call_attr(:@num).__to_integer
				self
			end
		end
		#

		# Unary Operators
		define_attr :__BIT_NOT do
			::Quest::Number.new ~__to_integer
		end

		define_attr :'__UNARY_+' do
			::Quest::Number.new +@num
		end

		define_attr :'__UNARY_-' do
			::Quest::Number.new -@num
		end
	end
end



