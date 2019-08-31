require_relative 'util'
$DEBUG = 2
Frac = Quest::Object.get_attr(:birth).(_{
	self.__at_num = _{
		self.numer / self.denom
	};

	self._gcd = _{ |a, b|
		_while(_{ b != 0._ }, _{
			t = b;
			b = a % b;
			a = t
		});
		a
	};

	self.birth = _{ |numer, denom|
		self.__parent__.birth.bind(self).(_{
			gcd = self._gcd(numer, denom);
			self.numer = numer / gcd;
			self.denom = denom / gcd;
			self
		})
	};

	self.__plus = _{ |rhs|
		parent = self.__parent__
		_if(rhs.is_a(parent), _{
			parent.birth.(
				self.numer * rhs.denom + rhs.numer * self.denom,
				self.denom * rhs.denom
			)
		}, _{
			parent.birth.(self.numer * rhs, self.denom);
		}).()
	};

	self.__at_text = _{
		self.numer.c(:@text) + _if(self.denom == 1._, ''._, '/'._ + self.denom.c(:@text))
	};
	
	self
});

frac = Frac.birth.(3.0._, 4._);
puts (frac + 3._).c(:@text);
puts frac.c(:@num).c(:@text)
puts (frac + Frac.birth.(1._, 4._)).c(:@text)
