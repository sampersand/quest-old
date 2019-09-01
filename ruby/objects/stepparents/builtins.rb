require_relative '../object'

class Quest::StepParents::Builtins < Quest::Object
	define_attrs parents: [ ::Quest::Pristine ] do
		define_attr :if do |cond, if_true, if_false=nil|
			if cond.call_into :@bool
				if_true
			else
				if_false || ::Quest::Null.new
			end
		end

		define_attr :while do |cond, body|
			while cond.call_attr(:'()').call_into :@bool
				body.call_attr :'()'
			end || ::Quest::Null.new
		end

		define_attr :disp do |msg|
			::Kernel::puts msg.call_into :@text
			msg
		end

		define_attr :rand do |low=nil, high=nil|
			high &&= high.call_into :@num
			low &&= low.call_into :@num
			
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

		define_attr :system do |cmd|
			::Quest::Text.new ::Kernel.:`.(cmd)
		end

		define_attr :try do |uniq_obj=nil, meth|
			uniq_obj ||= ::Quest::Pristine.call_attr :init
			levels, result = ::Kernel::catch :return do
				[uniq_obj, meth.call_attr(:'()', uniq_obj)]
			end

			if levels.call_attr(:==, uniq_obj).call_into :@bool
				result
			elsif levels.call_into(:@num) <= 1
				result || ::Quest::Null.new
			else
				throw [levels.call_attr(:-, ::Quest::Number.new(1)), result]
			end
		end

		define_attr :return do |levels=::Quest::Number.new(1), result=nil|
			::Kernel::throw :return, [levels, result]
		end
	end
end




