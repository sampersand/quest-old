module Quest::HasAttributes
	alias :g :get_attr
	alias :s :set_attr
	alias :c :call_attr
	def _ &block; ::Quest::Block.new &block end
	def call *a; c :'()', *a end
end

Frac = Quest::Object.g(:init).(::Quest::Block.new {
	self.set_attr :_gcd, _{ |a, b|
		self.g(:while).(_{ b }, _{
			t = b;
			b = a.c(:%, b)
			a = t
		})
		a
	};

	self.set_attr :init, _{ |n, d|
		# todo: check for zero denom
		gcd = self.g(:_gcd).(n, d);
		n.g(:'/=').(gcd);
		d.g(:'/=').(gcd);

		self.g(:super).('init'.to_q).g(:bind).(self).(_{
			self.set_attr :numer, n
			self.set_attr :denom, d
			self
		})
	};

	self.set_attr :@text, _{
		result = self.g(:numer).g(:@text).();
		self.g(:if).(self.g(:denom).g(:'!=').(1.to_q), _{
			result.g(:'+=').('/'.to_q.g(:+).(self.g(:denom).g(:@text).()))
		}).();
		result
	};

	self.set_attr :@num, _{
		self.g(:numer).g(:/).(self.g(:denom))
	}

	self.set_attr :+, _{ |rhs|
		super_init = self.g(:super).('init'.to_q);
		self.g(:if).( rhs.g(:is_a).(self.g(:base_parent).()), _{
			super_init.(
				self.g(:numer).g(:*).(rhs.g(:denom)).g(:+).(rhs.g(:numer).g(:*).(self.g(:denom))),
				self.g(:denom).g(:*).(rhs.g(:denom))
			)
		}, _{
			super_init.(
				self.g(:numer).g(:*).(rhs.g(:@num).()),
				self.g(:denom)
			)
		}).()
	};

	self.set_attr :clone, _{
		self.g(:base_parent).().g(:init).(self.g(:numer), self.g(:denom))
	}
	
	self
});


frac = Frac.g(:init).(8.0.to_q, 5.to_q);

puts frac.g(:+).(2.5.to_q).c(:@text)






