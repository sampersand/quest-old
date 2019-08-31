require_relative '../object'

class Quest::StepParents::TruthyContainers < Quest::Object
	define_attrs do
		# We're expecting that we'll have `length` method

		define_attr :@bool do
			::Quest::Boolean.new !call_attr(:length).call_attr(:@num).__num.zero?
		end

	end
end