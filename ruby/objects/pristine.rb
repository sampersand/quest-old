class Quest::Pristine < BasicObject
# Extensions
	ATTRIBUTES = ::Quest::Attributes.new __id__, []

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
			cls.const_set :ATTRIBUTES, ::Quest::Attributes.new(__id__, [])
		end

	private

		def define_attrs ancestors:, &block
			const_get(:ATTRIBUTES).replace ::Quest::Attributes.new(__id__, ancestors, &block)
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

	def initialize ancestors: [self.class]
		@_attributes = ::Quest::Attributes.new(__id__, ancestors)
	end

# Attributes
	def attrs
		@_attributes
	end

	include ::Quest::HasAttributes

	define_attrs ancestors: []
end



