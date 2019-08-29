class RQ::Delegator < BasicObject
	def self.replace_attrs_special_chars attribute
		attribute = attribute.to_s
		%w(
			TILDE ~ BACKTICK ` EXCLAIMATION ! ATSIGN @ POUND # DOLLAR $ PERCENT %
			CARET ^ AMPERSAND & ASTERISK * LPAREN ( RPAREN ) MINUS - UNDERSCORE _
			PLUS + EQUAL = LCURLY { RCURLY } LBRACKET [ RBRACKET ] BACKSLASH \\ 
			PIPE | COLON : SEMICOLON ; QUOTE ' DOUBLEQOUTE " LESSTHAN < COMMA ,
			GREATERTHAN > PERIOD . SLASH / QUESTION ?
			ZERO 0 ONE 1 TWO 2 THREE 3 FOUR 4 FIVE 5 SIX 6 SEVEN 7 EIGHT 8 NINE 9
		).each_slice 2 do |orig, replace|
			attribute.gsub! '__' + orig + '__', replace
		end

		case attribute
		when /^[+-]?\d+(\.\d+)?([eE][-+]?\d+)?$/,
		     /^0[dD]\d+$/, /^0[bB][01]+$/, /^0[oO][0-7]$/, /^0[fF][a-fA-F0-9]$/,
		     attribute.gsub!(/^__NUMBER__/, '') then attribute.to_i
		when attribute.gsub!(/^__TEXT__/, '') then attribute.to_s
		else attribute.to_sym
		end
	end

	def initialize attrs={}
		@attrs = attrs.map{ |key, val| [::RQ::Delegator::replace_attrs_special_chars(key), val]}.to_h

		def self.inspect
			::Kernel.instance_method(:inspect).bind(self).call
		end unless @attrs.include? :inspect

		def self.respond_to? arg, _unknown=false
			::Kernel.instance_method(:respond_to?).bind(self).call arg
		end unless @attrs.include? :respond_to?
	end

	def method name
		::Kernel::instance_method(:method).bind(self).call name
	rescue ::NameError => e
		name = ::RQ::Delegator::replace_attrs_special_chars name
		::Kernel::print "method (#{name.inspect})"
		case name
		when @attrs.method(:include?)
			attr = @attrs[name]
			(attr = attr.clone; attr.this = self) if attr.respond_to? :this=
			attr
		when /=$/ then 
			::Kernel::puts 'foo'
			::Kernel::proc{ |val| @attrs[name[0..-2].to_sym] = val }
		else
			$!.set_backtrace ::Kernel::caller 2
			::Kernel::raise
		end
	end

	def method_missing name, *args, &block
	# 	super
	# rescue ::NameError
		# ::Kernel::puts "hi"
		name = ::RQ::Delegator::replace_attrs_special_chars name
		::Kernel::puts "method_missing: #{name.inspect}"
		self.method(name).call *args, block
	end

	def clone
		::RQ::Delegator.new @attrs.clone
	end

	def __get_attr key; @attrs[key] end # `var.-1`
	def __set_attr key, val; @attrs[key] = val end # `var.= -1, -1`
	def __has_attr key; @attrs.include? key end # `var.? -1`
	def __del_attr key; @attrs.delete key end # `var.~ -`

	def __attrs; @attrs end

	def respond_to_missing? name, _unknown=false
		::Kernel::p("respond to missing? #{name}")
		# return true if ::Kernel::instance_method(:respond_to_missing?).bind(self).call(name, _unknown)
		case ::RQ::Delegator::replace_attrs_special_chars name
		when @attrs.:include? then true
		when ::String then true
		when /=$/ then true
		else false
		end
	end
end
