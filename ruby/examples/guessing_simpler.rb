require_relative 'util'
class Quest::Pristine
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
				call_attr a.to_s.gsub(/^at_/,'@').to_sym, *args2
			end
		end
	end

	def_call %i(while prompt rand != + < > disp if)
end

Quest::Object.get_attr(:birth).call(_{
	self.max = 100._;
	self.secret = self.rand(1._, self.max);
	self.guesses = 0._;
	self.guess = nil._;

	self.while(_{ self.guess != self.secret }, _{
		self.guess = self.prompt("Guess from 1-"._ + self.max).c(:@num);
		self.guesses.c(:'++@');
		self.disp(
			self.if(self.guess < self.secret, _{"too low!"._}, _{
				self.if(self.guess > self.secret, "too high!"._, "perfect!"._)
			}).()
		);
	});
	self.disp("It took you "._ + self.guesses.c(:@text) + ' tries!'._);
});
