module Quest
	module HasAttributes
		class << self
			def check_attr self_, method_name, attr
				unless valid_attr? attr
					Quest::warn caller.first, "#{self_.inspect}.#{method_name} received a non-Quest Object and non-Symbol attr '#{attr.inspect}'"
				end
			end

			def check_val self_, method_name, attr, val, name='val'
				unless valid_val? val
					Quest::warn caller.first, "#{self_.inspect}.#{method_name}(#{attr.inspect}) recieved a non-Quest Object #{name} '#{val.inspect}'"
				end
			end

			def check_result self_, method_name, attr, result
				unless valid_result? result
					Quest::warn caller.first, "#{self_.inspect}.#{method_name}(#{attr.inspect}) returned a non-Quest Object '#{result.inspect}'"
				end
			end

		private

			def valid_attr? attr
				attr.is_a? Symbol or ::Quest::quest_object? val
			end

			def valid_val? val
				::Quest::quest_object? val
			end

			def valid_result? val
				::Quest::quest_object? val
			end
		end

		def get_attr attr
			Quest::if_debug do
				HasAttributes::check_attr self, ::Kernel::__method__, attr
			end 

			if (result = attrs.get_attr attr).respond_to? :bind_owner
				result = result.bind_owner self
			end

			Quest::if_debug do
				HasAttributes::check_result self, ::Kernel::__method__, attr, result
			end

			result || Quest::Null.new
		end

		def set_attr attr, val
			Quest::if_debug do
				HasAttributes::check_attr self, ::Kernel::__method__, attr
				HasAttributes::check_val self, ::Kernel::__method__, attr, val
			end

			attrs.set_attr attr, val
		end

		def del_attr attr
			Quest::if_debug do
				HasAttributes::check_attr self, ::Kernel::__method__, attr
			end

			result = attrs.del_attr attr

			Quest::if_debug do
				HasAttributes::check_result self, ::Kernel::__method__, attr, result
			end

			result || Quest::Null::new
		end

		def has_attr? attr
			Quest::if_debug do
				HasAttributes::check_attr self, ::Kernel::__method__, attr
			end

			attrs.has_attr? attr
		end

		def respond_to_attr? attr
			Quest::if_debug do
				HasAttributes::check_attr self, ::Kernel::__method__, attr
			end

			attrs.respond_to_attr? attr
		end


		def call_attr attr, *args
			Quest::if_debug do
				HasAttributes::check_attr self, ::Kernel::__method__, attr

				args.each do |arg|
					HasAttributes::check_val self, ::Kernel::__method__, attr, arg, 'arg'
				end
			end
		
			return Quest::Null.new unless respond_to_attr? attr

			result = if (attribute = get_attr attr).is_a? ::Quest::Block
				attribute.call *args
			else
				# this might lose `self`, so we need to be careful there
				attribute.call_attr :'()', *args
			end

			Quest::if_debug do
				case attr
				when :@bool then
					::Quest::warn "@bool didn't return Boolean (got #{result.inspect} from #{inspect})" unless result.is_a? ::Quest::Boolean
				when :@num then
					::Quest::warn "@num didn't return Number (got #{result.inspect} from #{inspect})" unless result.is_a? ::Quest::Number
				when :@text then
					::Quest::warn "@text didn't return Text (got #{result.inspect} from #{inspect})" unless result.is_a? ::Quest::Text
				else
					HasAttributes::check_result self, ::Kernel::__method__, attr, result
				end
			end

			result
		end

	end

	class Attributes
		def initialize uid, parent=nil, stepparents=nil, &block
			@attributes = Hash.new
			@attributes[:__readonly__] = @readonly = [:__uid__]
			@attributes[:__uid__] = uid
			set_attr :__parent__, parent if parent
			set_attr :__stepparents__, stepparents if stepparents

			if block_given?
				def self.define_attr attr, val=nil, &block
					warn "Both a literal value (#{val.inspect}) and a block (#{block.inspect}) given" if val && block
					fail "Neither a block nor an attribute given" unless val || block
					set_attr attr, val || ::Quest::Block.new(&block)
				end

				instance_exec &block

				class << self
					remove_method :define_attr
				end
			end
		end

		def replace other
			@attributes = other.instance_variable_get :@attributes
			@readonly = other.instance_variable_get :@readonly
		end

		def inspect
			"#{self.class.name}(#{@attributes.keys.map(&:inspect).join ', '})"
		end

		def readonly? attr
			@readonly.include? attr
		end

		def has_attr? attr
			@attributes.include? attr
		end

		def respond_to_attr? attr
			has_attr?(attr) || 
				@attributes[:__parent__]&.respond_to_attr?(attr) ||
				@attributes[:__stepparents__]&.any?{|sp| sp.respond_to_attr?(attr) } ||
				false
		end

		def get_attr attr
			if has_attr? attr
				attribute = @attributes[attr]
				case attr.hash
				when :__uid__.hash then ::Quest::Number.new attribute
				when :__stepparents__.hash then ::Quest::List.new attribute
				else attribute
				end
			else
				@attributes[:__stepparents__]&.each&.lazy&.map{|stepparent| stepparent.get_attr attr }&.find{|x| x} ||
					@attributes[:__parent__]&.get_attr(attr)
			end
		end

		def set_attr attr, val
			raise "Attr #{attr} is readonly" if readonly? attr
			@attributes[attr] = val
		end

		def del_attr attr
			@attributes.delete attr or ::Quest::Null.new
		end
	end
end



