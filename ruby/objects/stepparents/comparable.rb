require_relative '../object'

class Quest::StepParents::Comparable < Quest::Object
	define_attrs do
		# We're expecting that we'll have `<=>`

		# all of these can be upgraded in children, and take lower precidence than others
		define_attr :< do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_attr(:@num).__num < 0
		end

		define_attr :<= do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_attr(:@num).__num <= 0
		end

		define_attr :> do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_attr(:@num).__num > 0
		end

		define_attr :>= do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_attr(:@num).__num >= 0
		end

		define_attr :== do |rhs|
			::Quest::Boolean.new call_attr(:<=>, rhs).call_attr(:@num).__num == 0
		end
	end
end