require_relative 'rq'

module RQ
	class << self
		private def define_rquest_method name, &block
			method = ::RQ::Block.new &block
			define_method name do# |*args|
				# if args.empty?
					method
				# elsif args.length == 1 && args[0] == :empty
					# method.call
				# else
					# method.call *args
				# end
			end
		end
	end

module_function

	define_rquest_method :dispn do 
		print @_0.join(' '), "\n"
	end

	define_rquest_method :disp do
		print @_0.join ' '
	end

	define_rquest_method :prompt do
		print @_1.to_s if defined? @_1
		gets.chomp
	end

	define_rquest_method :quit do
		if @_1.is_a? Numeric
			exit @_1.to_i
		else
			abort @_1.to_s
		end
	end

	define_rquest_method :random do 
		if defined? @_1 and defined? @_2
			rand @_1..@_2
		elsif defined? @_1
			rand @_1
		else
			rand
		end
	end

	define_rquest_method :_if do
		@_1.to_b ? @_2 : @_3
	end

	define_rquest_method :_unless do
		@_1.to_b ? @_3 : @_2
	end

	define_rquest_method :_while do
		while @_1.().to_b do
			@_2.()
		end
	end

	define_rquest_method :_until do
		until @_1.().to_b do
			@_2.()
		end
	end

	define_rquest_method :_loop do
		loop do
			@_1.()
		end
	end

	define_rquest_method :_for do
		@_1.()
		while @_2.().to_b
			@_4.()
			@_3.()
		end
	end

	define_rquest_method :_return do
		throw :return, [defined?(@_1) ? @_1 : 1, @_2]
	end

	define_rquest_method :switch do
		@_2.method(@_1).call
	end

	$locals = ::RQ::Block.new do
		# TODO: have this correctly list `@` variables as well, eg `@_0`
		::RQ::Delegator.new $ENVIRONMENTS.last.local_variables.map{|var| [var, $ENVIRONMENTS.last.local_variable_get(var)]}.to_h
	end



	class << self
		undef :define_rquest_method
	end
end

