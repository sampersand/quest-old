
module Quest
	class Block < Object
		def initialize owner=nil, &block
			super()
			@owner, @block = owner, block
		end

		def owned?
			!!@owner
		end

		def bind_owner owner
			# this can be implemented in a variety of ways, but this is the quickest
			Block.new owner, &@block
		end

		def call *args
			if owned?
				@owner.instance_exec *args, &@block
			else
				# do we want this to bind to `null` or something for self?
				@block.call *args
			end
		end

		p ancestors
		exit

		define_attrs parent: Object do
			define_attr
		end
	end
end