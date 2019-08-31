class Quest::Object < BasicObject
# Extensions
	ATTRIBUTES = ::Quest::Attributes.new __id__

	class << self
		def attrs
			const_get :ATTRIBUTES
		end

		include ::Quest::HasAttributes

		def kernel_methods meths
			meths.each do |method_name|
				method = ::Kernel::instance_method method_name
				define_method method_name do |*a, &bl|
					method.bind(self).call *a, &bl
				end
			end
		end

		def inherited cls
			cls.const_set :ATTRIBUTES, ::Quest::Attributes.new(__id__)
		end

	private

		def define_attrs parent: ancestors[1], &block
			const_get(:ATTRIBUTES).replace ::Quest::Attributes.new(__id__, parent: parent, &block)
		end
	end

	kernel_methods %i(
		class respond_to? clone initialize_clone initialize_copy
		inspect instance_variable_get instance_variable_set is_a?
		hash
	)

	class << self
		undef kernel_methods
	end

# Instance stuff
	attr_reader :_attributes
	def to_s; inspect end
	alias :eql? :==

	def initialize
		@_attributes = ::Quest::Attributes.new(__id__, parent: self.class)
	end

# Attributes
	def attrs
		@_attributes
	end

	include ::Quest::HasAttributes
end

require_relative 'block'

class Quest::Object
	define_attrs parent: nil do
		define_attr :@hash do
			::Quest::Number.new hash
		end

		define_attr :is_a do |rhs|
			if self.call_attr(:===, rhs).call_attr(:@bool).true?
				::Quest::Boolean.new true
			elsif has_attr? :__parent__
				get_attr(:__parent__).call_attr(:is_a, rhs)
			else
				::Quest::Boolean.new false
			end
		end

		define_attr :@text do
			::Quest::Text.new inspect
		end

 		# this is such a hack, but idk what to do otherwise for quotes on strings
		define_attr :@text_inspect do |*args|
			call_attr :@text, *args
		end

		define_attr :@bool do
			::Quest::Boolean.new true
		end

		define_attr :clone do
			clone
		end

		define_attr :'!' do
			call_attr(:@bool).call_attr :'!'
		end

		define_attr :=== do |rhs|
			::Quest::Boolean.new get_attr(:__uid__).__num == rhs.get_attr(:__uid__).__num
		end

		define_attr :'!==' do |rhs|
			call_attr(:===, rhs).call_attr :'!'
		end

		define_attr :== do |rhs|
			call_attr :===, rhs
		end

		define_attr :'!=' do |rhs|
			call_attr(:==, rhs).call_attr(:'!')
		end
	end
end



