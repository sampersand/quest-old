module Quest::HasAttributes
	alias :g :get_attr
	alias :s :set_attr
	alias :c :call_attr
	def call *a; c :'()', *a end
end

Frac = Quest::Object.g(:init).(::Quest::Block.new {
	s :@num, ::Quest::Block.new{
		g(:numer).g(:/).(g(:denom))
	};

	set_attr :_gcd, ::Quest::Block.new{ |a, b|
		_while(::Quest::Block.new{ b != 0._ }, ::Quest::Block.new{
			t = b;
			b = a % b;
			a = t
		});
		a
	};

	set_attr :init, ::Quest::Block.new{ |numer, denom|
		self.__parents__.__list[0].init.bind(self).(::Quest::Block.new{
			gcd = self._gcd(numer, denom);
			self.numer = numer / gcd;
			self.denom = denom / gcd;
			self
		})
	};

	self.__plus = ::Quest::Block.new{ |rhs|
		parent = self.__parents__.__list[0]
		_if(rhs.is_a(parent), ::Quest::Block.new{
			parent.init.(
				self.numer * rhs.denom + rhs.numer * self.denom,
				self.denom * rhs.denom
			)
		}, ::Quest::Block.new{
			parent.init.(self.numer * rhs, self.denom);
		}).()
	};

	self.__at_text = ::Quest::Block.new{
		self.numer.c(:@text) + _if(self.denom == 1._, ''._, '/'._ + self.denom.c(:@text))
	};
	
	self
});


frac = Frac.init.(3.0._, 4._);
puts (frac + 3._).c(:@text);
puts frac.c(:@num).c(:@text)
puts (frac + Frac.init.(1._, 4._)).c(:@text)
