require_relative 'object'

class Quest::Block < Quest::Object
	def initialize owner=nil, &block
		super()
		@owner, @block = owner, block
	end

	def owned?
		!!@owner
	end

	def bind_owner owner
		# this can be implemented in a variety of ways, but this is the quickest
		::Quest::Block.new owner, &@block
	end

	def call *args
		::Quest::if_debug do
			args.each do |arg|
				if !::Quest::quest_object? arg
					::Quest::warn "block::call received a non-Quest Object arg '#{arg.inspect}'"
				end
			end
		end

		if owned?
			@owner.instance_exec *args, &@block
		else
			# do we want this to bind to `null` or something for self?
			@block.call *args
		end
	end

	define_attrs ancestors: [ ::Quest::Object ] do
		define_attr :'()' do |*a|
			call *a
		end

		define_attr :bind do |owner|
			bind_owner owner
		end
	end
end

