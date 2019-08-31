require_relative '../object'

class Quest::StepParents::Builtins < Quest::Object
	define_attrs parent: nil do
		define_attr :if do |cond, if_true, if_false=nil|
			if cond.call_attr(:@bool).true?
				if_true
			else
				if_false || ::Quest::Null.new
			end
		end

		define_attr :while do |cond, body|
			while cond.call_attr(:'()').call_attr(:@bool).true?
				body.call_attr :'()'
			end || ::Quest::Null.new
		end

		define_attr :disp do |msg|
			::Kernel::puts msg.call_attr(:@text).__text
			msg
		end

		define_attr :rand do |low=nil, high=nil|
			high &&= high.call_attr(:@num).__num
			low &&= low.call_attr(:@num).__num
			::Quest::Number.new(if high
				::Kernel::rand low..high
			elsif low
				::Kernel::rand 0..low
			else
				::Kernel::rand
			end)
		end

		define_attr :prompt do |msg=nil|
			call_attr :disp, msg if msg
			::Quest::Text.new ::Kernel::gets.chomp
		end
	end
end