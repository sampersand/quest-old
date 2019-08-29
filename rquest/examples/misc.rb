require_relative 'rq'


p Integer.ancestors

p 1.+(2)
RQ::exec self do

	x = 0
	while_.(_{x < 10}, _{
		x += 1;
		if_.(x < 5, _{
			disp.(x + " < 5"){}
		}, _{
			disp.("#{x} >= 5"){}
		}){}.(){}
	}){}

end

RQ::exec self do

	Frac = _{
		new = _{
			frac = @this.clone;
			frac.class = Frac.object_id;
			frac.numerator = @_1;
			frac.denominator = @_2;
			frac
		};

		to_s = _{ @this.numerator + '/' + @this.denominator }

		# b = binding
		__ASTERISK__ = _{
			other = @_1;

			Frac.new.(@this.numerator * other.numerator, @this.denominator * other.denominator){}
		};

		$locals.(){}
	}.(){};

	puts (Frac.new.(1, 2){}.*(){}.(3){}).to_s.(){}
	# puts Frac.(){};
	# puts Frac.(){}.to_s.(__env: binding)
	# x = 0
	# while_.(_{x < 10}, _{
	# 	x += 1;
	# 	if_.(x < 5, _{
	# 		disp.("x < 5"){}
	# 	}, _{
	# 		disp.("x>=5"){}
	# 	}){}.(){}
	# }){}




end