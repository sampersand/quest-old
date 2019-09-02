require_relative '../object'

class Quest::StepParents::Comparable < Quest::Object
	define_attrs ancestors: [ ::Quest::Object ] do
		# We're expecting that we'll have `<=>`

		# all of these can be upgraded in children, and take lower precidence than others
		define_attr :< do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_into(:@num) < 0
		end

		define_attr :<= do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_into(:@num) <= 0
		end

		define_attr :> do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_into(:@num) > 0
		end

		define_attr :>= do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_into(:@num) >= 0
		end

		define_attr :== do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_into(:@num) == 0
		end
	end
end