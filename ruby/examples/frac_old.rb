require_relative 'util'

Frac = Quest::Object.call_attr(:init, _{
	self.set_attr :@num, _{
		self.get_attr(:numer).call_attr(:/, self.get_attr(:denom))
	};

	self.set_attr :_gcd, _{ |a, b|
		self.call_attr(:while, _{ b.call_attr(:!=, 0.to_q) }, _{
			t = b;
			b = a.call_attr(:%, b);
			a = t
		});
		a
	};

	self.set_attr :init, _{ |numer, denom|
		self.get_attr(:__parent__).get_attr(:init).call_attr(:bind, self)
			.call_attr(:'()', _{
				gcd = self.call_attr(:_gcd, numer, denom);
				self.set_attr :numer, numer.call_attr(:/, gcd);
				self.set_attr :denom, denom.call_attr(:/, gcd);
				self
			})
	};

	self.set_attr :+, _{ |rhs|
		_Frac = self.get_attr(:__parent__);

		self.call_attr(:if, rhs.call_attr(:is_a, _Frac), _{
			_Frac.call_attr(:init,
				self.get_attr(:numer).call_attr(:*, rhs.get_attr(:denom))
					.call_attr(:+, rhs.get_attr(:numer).call_attr(:*, self.get_attr(:denom))),
				self.get_attr(:denom).call_attr(:*, rhs.get_attr(:denom))
			)
		}, _{
			_Frac.call_attr(:init, self.get_attr(:numer).call_attr(:*, rhs), self.get_attr(:denom))
		}).call_attr(:'()')
	};

	self.set_attr :@text, _{
		numer = self.get_attr(:numer).call_attr(:@text);
		self.call_attr(:if, self.get_attr(:denom).call_attr(:==, 1.to_q), _{
			numer
		}, _{
			numer.call_attr(:+, '/'.to_q).call_attr(:+, self.get_attr(:denom).call_attr(:@text))
		}).call_attr(:'()')
	};
	self
});

frac = Frac.call_attr(:init, 3.to_q, 4.to_q);
puts frac.call_attr(:+, 3.to_q).call_attr(:@text)
puts frac.call_attr(:+, Frac.call_attr(:init, 1.to_q, 4.to_q)).call_attr(:@text)
