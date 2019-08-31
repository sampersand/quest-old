class BasicObject
	def _ &block
		return ::Quest::Block.new &block if block
		to_q
	end
end


class String
	def call rhs
		rhs.c to_sym
	end
end

module Quest::HasAttributes
# class Quest::Pristine
	def call *a; call_attr :'()', *a end
	alias :g :get_attr
	alias :c :call_attr
	alias :s :set_attr

	def method name
		return super if respond_to? name
		get_attr name
	end

	def respond_to_missing? meth, _
		meth[-1] == '=' || respond_to_attr?(meth)
	end

	def method_missing meth, *args
		meth = meth.to_s.gsub('__at_','@').gsub('__plus','+').to_sym
		if meth[-1] == '='
			set_attr meth[0..-2].to_sym, *args
		elsif respond_to_attr? meth
			get_attr meth
		else
			false
		end
	end

	def self.def_call args
		args.each do |a|
			define_method a do |*args2|
				call_attr a.to_s.gsub(/^_(?=(while|if)$)/,'').to_sym, *args2
			end
		end
	end

	def_call %i(
		_while _if prompt rand != + * < > / % disp
		_gcd bind is_a ==
	)
	
	def to_s
		call_attr(:@text).__text
	end
end
