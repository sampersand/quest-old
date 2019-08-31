module Quest
	module HasAttributes
	# 	def attr_getter_setter *attrs
	# 		attr_getter *attrs
	# 		attr_setter *attrs
	# 	end

	# 	def attr_getter *attrs
	# 		attrs.each do |attr|
	# 			define_method attr do
	# 				get_attr :"__#{attr}__"
	# 			end
	# 		end
	# 	end
		
	# 	def attr_setter *attrs
	# 		attrs.each do |attr|
	# 			define_method :"#{attr}=" do |val|
	# 				set_attr :"__#{attr}__", val
	# 			end
	# 		end
	# 	end

		def get_attr attr
			unless attr.is_a?(Symbol) || attr.is_a?(Object)
				warn "#{self.class.name}.get_attr(#{attr.inspect}) received a non-Quest Object and non-Symbol attr '#{attr.inspect}'"
			end

			result = if (result = attrs.get_attr attr).respond_to? :bind_owner
				result.bind_owner self
			else
				result
			end

			unless result.is_a? Object
				warn "#{self.class.name}.get_attr(#{attr.inspect}) returned a non-Quest Object '#{result.inspect}'"
			end

			result
		end

		def set_attr attr, val
			if !val.is_a? Object
				warn "#{self.class.name}.set_attr(#{attr.inspect}) recieved a non-Quest Object val '#{val.inspect}'"
			elsif !attr.is_a?(Symbol) && !attr.is_a?(Object)
				warn "#{self.class.name}.set_attr(#{attr.inspect}) received a non-Quest Object and non-Symbol attr '#{attr.inspect}'"
			end

			attrs.set_attr attr, val
		end

		def del_attr attr
			unless attr.is_a?(Symbol) || attr.is_a?(Object)
				warn "#{self.class.name}.del_attr(#{attr.inspect}) received a non-Quest Object and non-Symbol attr '#{attr.inspect}'"
			end

			result = attrs.del_attr attr

			unless result.is_a? Object
				warn "#{self.class.name}.del_attr(#{attr.inspect}) returned a non-Quest Object '#{result.inspect}'"
			end

			result
		end

		def has_attr? attr
			unless attr.is_a?(Symbol) || attr.is_a?(Object)
				warn "#{self.class.name}.del_attr(#{attr.inspect}) received a non-Quest Object and non-Symbol attr '#{attr.inspect}'"
			end

			attrs.has_attr? attr
		end

		def call_attr attr, *args
			unless attr.is_a?(Symbol) || attr.is_a?(Object)
				warn "#{self.class.name}.call_attr(#{attr.inspect}) received a non-Quest Object and non-Symbol attr '#{attr.inspect}'"
			end

			args.each do |arg|
				unless arg.is_a? Object
					warn "#{self.class.name}.call_attr(#{attr.inspect}) received a non-Quest Object arg '#{arg.inspect}'"
				end
			end

			
			result = 
				if (attribute = get_attr attr).is_a? Block
					attribute.call *args
				else
					# this might lose `self`, so we need to be careful there
					attribute.call_attr :'()', *args
				end

			case attr
			when :@bool then fail "@bool didn't return Boolean (got #{result.inspect} from #{inspect})" unless result.is_a? Boolean
			when :@num then fail "@num didn't return Number (got #{result.inspect} from #{inspect})" unless result.is_a? Number
			when :@text then fail "@text didn't return Text (got #{result.inspect} from #{inspect})" unless result.is_a? Text
			end

			unless result.is_a? Object
				warn "#{self.class.name}.call_attr(#{attr.inspect}) returned a non-Quest Object '#{result.inspect}'"
			end

			result
		end

	end

	class Attributes
		def initialize uid, parent: nil, &block
			@attributes = {
				:__readonly__ => @readonly = [:__uid__]
			}
			@attributes[:__uid__] = uid
			@attributes[:__parent__] = parent if parent

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

		def inspect
			"#{self.class.name}(#{@attributes.keys.map(&:inspect).join ', '})"
		end

		def readonly? attr
			@readonly.include? attr
		end

		def has_attr? attr
			@attributes.include? attr
		end

		def get_attr attr
			@attributes[attr] || @attributes[:__parent__]&.get_attr(attr)
		end

		def set_attr attr, val
			fail "Attr #{attr} is readonly" if readonly? attr
			@attributes[attr] = val
		end

		def del_attr attr
			@attributes.delete attr or ::Quest::Null.new
		end
	end
end