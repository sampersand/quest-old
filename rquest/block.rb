class RQ::Block
	attr_accessor :this

	def initialize this=nil, &block
		fail "uncallable block given #{block.inspect}" unless block.respond_to? :call
		@block, @parent = block, this
	end

	class Scope
		def initialize args, bindings, this
			@_0 = args
			@bindings = bindings
			@this = this if this

			args.each_with_index do |arg, index|
				instance_variable_set :"@_#{index + 1}", arg
			end
		end

		def method_missing meth, *args, &block
			@bindings[0..-2].each do |binding|
				value =
					begin
						@binding.eval("method(:#{meth})")
					rescue ::NameError
						next
					end
				return value.call *args, &block
			end

			@bindings.last.eval("method(:#{meth})").call *args, &block
		end
	end


	def [] *args
		call *args, __env: @block.binding
	end

	def call *args, __env: nil, &block_for_binding
		tocall = @block

		__env ||= block_for_binding&.binding and $ENVIRONMENTS.push __env

		levels, value = catch :return do
			result = Scope.new(args, $ENVIRONMENTS.clone, @this).instance_exec &tocall
			$ENVIRONMENTS.pop if __env
			return result
		end

		$ENVIRONMENTS.pop if __env

		if levels <= 1
			value
		else
			throw :return, [levels -1, value]
		end
	end
end

