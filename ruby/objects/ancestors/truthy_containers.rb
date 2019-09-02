require_relative '../object'

class Quest::StepParents::TruthyContainers < Quest::Object
	define_attrs ancestors: [ ::Quest::Object ] do
		# We're expecting that we'll have `length` method

		define_attr :@bool do
			::Quest::Boolean.new !call_attr(:length).call_into(:@num).zero?
		end

	end
end