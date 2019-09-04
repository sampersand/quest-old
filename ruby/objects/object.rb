class Quest::Object < Quest::Pristine
end

require_relative 'block'
require_relative 'ancestors/all'

class Quest::Object
	# def == rhs
	# 	call_attr(:==, rhs).call_into :@bool
	# end

	define_attrs ancestors: [
			::Quest::StepParents::Builtins,
			::Quest::Pristine
		] do

		define_attr :@hash do
			::Quest::Number.new hash
		end

		define_attr :base_ancestor do
			get_attr(:__ancestors__).call_attr :[], ::Quest::Number.new(1)
		end

		define_attr :is_null do
			call_attr(:is_a, ::Quest::Null)
		end

		define_attr :super do |attr|
			get_attr(:__ancestors__).__list
				.lazy
				.map{|ancestor| ancestor.get_attr attr }
				.find{|meth| meth.call_attr(:is_null).call_attr(:'!').call_into(:@bool) }
		end

		define_attr :execute do |block|
			block.call_attr(:bind, self).call_attr(:'()')
		end

		define_attr :init do |block=nil|
			newobj = ::Quest::Pristine.new ancestors: [self]
			block ? newobj.call_attr(:execute, block) : newobj
		end

		define_attr :is_a do |rhs|
			::Quest::Boolean.new(
				self.call_attr(:===, rhs).call_into(:@bool) ||
				(has_attr? :__ancestors__ and get_attr(:__ancestors__).__list.any?{|x| x.call_attr(:is_a, rhs).call_into :@bool })
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



