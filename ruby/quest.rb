module Quest
module_function
	def quest_object? *objs
		objs.all?{|x| x.is_a? Quest::Object}
	end

	alias :quest_objects? :quest_object?
end

require_relative 'attributes'
require_relative 'objects/all'