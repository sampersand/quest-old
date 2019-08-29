module RQ;
end
require_relative 'delegator'

module RQ
module_function
	def _ &block
		fail LocalJumpError, "Block needed" unless block_given?
		Block.new &block
	end

	def exec environment=Object.new, &block
		fail LocalJumpError, "Block needed to exec" unless block_given?

		class << environment
			include RQ
		end

		$ENVIRONMENTS ||= []
		result = catch :return do
			environment.instance_exec do 
				$ENVIRONMENTS << binding
				result = instance_exec &block # return entirely if it's successful
				$ENVIRONMENTS.pop
				return result
			end
		end

		warn "Tried to return #{result[0]} too many levels up" unless result[0] <= 1

		result[1]
	end
end

require_relative 'block'
require_relative 'patches'
require_relative 'methods'