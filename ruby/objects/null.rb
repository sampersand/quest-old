require_relative 'object'

class NilClass
	def to_q; Quest::Null.new end
end

class Quest::Null < Quest::Object
	def clone
		::Quest::Null.new
	end
	
	def inspect
		"Null()"
	end

	define_attrs parents: [ ::Quest::Object ] do
		define_attr :@text do
			::Quest::Text.new 'null'
		end

		define_attr :@bool do
			::Quest::Boolean.new false
		end

		define_attr :@num do
			::Quest::Number.new ::Float::NAN
		end

		define_attr :== do |rhs|
			::Quest::Boolean.new rhs.is_a? ::Quest::Null
		end
		
		define_attr :'()' do
			::Quest::Null.new
		end
	end
end