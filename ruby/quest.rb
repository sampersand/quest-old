module Quest

module_function
	def warn src=::Kernel::caller.first, msg
		if $DEBUG == 2
			::Kernel::fail msg 
		else
			::Kernel::warn "#{src}: #{msg}"
		end
	end

	def if_debug
		return unless $DEBUG
		yield
	end

	def quest_object? *objs
		objs.all?{|x| x.is_a?(Quest::Pristine) || x.is_a?(Class) && x.ancestors.include?(Quest::Pristine) }
	end

	alias :quest_objects? :quest_object?
end

require_relative 'attributes'
require_relative 'objects/all'