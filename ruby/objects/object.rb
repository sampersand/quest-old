class Quest::Object < BasicObject
# Extensions
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

	private

		def define_attrs parent: ancestors[1], &block
			parent = parent.const_get :ATTRIBUTES if defined? parent.ATTRIBUTES
			const_set :ATTRIBUTES, ::Quest::Attributes.new(__id__, parent: parent, &block)
		end
	end

	kernel_methods %i(
		class respond_to? clone initialize_clone initialize_copy
		inspect instance_variable_get instance_variable_set is_a?
	)

	class << self
		undef kernel_methods
	end

# Instance stuff
	attr_reader :_attributes
	alias :to_s :inspect

	def initialize
		@_attributes = Attributes.new __id__, parent: self.class.const_get(:ATTRIBUTES)
	end

	def warn src=::Kernel::caller.first, msg
		::Kernel::warn "#{src}: #{msg}"
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
		define_attr :@text do
			Text.new to_s
		end

		define_attr :@bool do
			Boolean.new true
		end

		define_attr :clone do
			clone
		end

		define_attr :'!' do
			call_attr(:@bool).call_attr :'!'
		end

		define_attr :=== do |rhs|
			Boolean.new get_attr(:__uid__) == rhs.get_attr(:__uid__)
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



