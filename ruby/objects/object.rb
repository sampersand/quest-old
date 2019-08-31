class Quest::Object < Quest::Pristine
end

require_relative 'block'
require_relative 'stepparents/all'

class Quest::Object
	define_attrs parent: nil, stepparents: [::Quest::StepParents::Builtins] do
		define_attr :@hash do
			::Quest::Number.new hash
		end

		define_attr :execute do |block|
			block.call_attr(:bind, self).call_attr(:'()')
		end

		define_attr :birth do |block=nil|
			newobj = ::Quest::Pristine.new self
			block ? newobj.call_attr(:execute, block) : newobj
		end

		define_attr :is_a do |rhs|
			::Quest::Boolean.new(
				self.call_attr(:===, rhs).call_attr(:@bool).true? ||
				(has_attr?(:__parent__) && get_attr(:__parent__).call_attr(:is_a, rhs).call_attr(:@bool).true?) ||
				(has_attr?(:__stepparents__) && get_attr(:__stepparents__).__list.any?{|x| x.call_attr(:is_a, rhs).call_attr(:@bool).true? })
			)
		end

		define_attr :@text do
			::Quest::Text.new inspect
		end

 		# this is such a hack, but idk what to do otherwise for quotes on strings
		define_attr :@text_inspect do |*args|
			call_attr :@text, *args
		end

		define_attr :@bool do
			::Quest::Boolean.new true
		end

		define_attr :clone do
			clone
		end

		define_attr :'!' do
			call_attr(:@bool).call_attr :'!'
		end

		define_attr :=== do |rhs|
			::Quest::Boolean.new get_attr(:__uid__).__num == rhs.get_attr(:__uid__).__num
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



