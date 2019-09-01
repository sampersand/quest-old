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

	define_attrs parents: [
		::Quest::StepParents::Comparable,
		::Quest::Object
	] do 
		define_attr :@text do
			if call_attr(:is_whole).call_into(:@bool)
				::Quest::Text.new @num.to_i.to_s
			else
				::Quest::Text.new @num.to_s
			end
		end

		define_attr :@num do
			clone				
		end

		define_attr :is_whole do
			::Quest::Boolean.new @num.to_i == @num
		end

		define_attr :@bool do
			::Quest::Boolean.new !@num.zero?
		end

		# Arithmetic Operators
		%i(+ - * / % **).each do |method|
			define_attr method do |rhs|
				::Quest::Number.new @num.send method, rhs.call_into(:@num)
			end

			# In-place
			define_attr :"#{method}=" do |rhs|
				@num = @num.send method, rhs.call_into(:@num)
				self
			end
		end

		define_attr :<=> do |rhs|
			::Quest::Number.new (@num <=> rhs.call_into(:@num)) || ::Float::NAN
		end


		# Bitwise Operators
		%i(__BIT_AND __BIT_OR __BIT_XOR __BIT_SHL __BIT_SHR).each do |method|
			ruby_methods = %i(__BIT_AND & __BIT_OR | __BIT_XOR ^ __BIT_SHL << __BIT_SHR >>).each_slice(2).to_h

			define_attr method do |rhs|
				::Quest::Number.new @num.to_i.send ruby_methods[method], rhs.call_into(:@num).to_i
			end

			# in place
			define_attr :"#{method}=" do |rhs|
				@num = @num.to_i.send ruby_methods[method], rhs.call_into(:@num).to_i
				self
			end
		end
		#

		# Unary Operators
		define_attr :__BIT_NOT do
			::Quest::Number.new ~@num.to_i
		end

		define_attr :'++@' do
			@num = @num + 1; self
		end

		define_attr :'@++' do
			old, @num = @num, @num + 1; ::Quest::Number.new old
		end

		define_attr :'--@' do
			@num = @num - 1; self
		end

		define_attr :'@--' do
			old, @num = @num, @num - 1; ::Quest::Number.new old
		end

		define_attr :'+@' do
			::Quest::Number.new +@num
		end

		define_attr :'-@' do
			::Quest::Number.new -@num
		end
	end
end



